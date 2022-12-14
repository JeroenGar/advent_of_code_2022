use std::str::FromStr;
use std::time::Instant;

const INPUT: &str = include_str!("../input/2022/day14.txt");

fn main() {
    let start = Instant::now();
    let mut cave : Cave = INPUT.parse().unwrap();
    let part1 = cave.simulate();
    println!("Part 1: {}", part1);
    cave.add_bedrock();
    println!("Part 2: {}", part1 + cave.simulate());
    println!("Time: {}ms", start.elapsed().as_millis());
}

#[derive(Debug, Clone)]
struct Cave {
    start: (i32, i32),
    shape: (i32, i32),
    grid: Vec<Vec<bool>>,
}

impl Cave {
    fn simulate(&mut self) -> usize {
        let mut sand_counter = 0;
        let left = |pos: (i32, i32)| { (pos.0 - 1, pos.1) };
        let right = |pos: (i32, i32)| { (pos.0 + 1, pos.1) };
        let under = |pos: (i32, i32)| { (pos.0, pos.1 + 1) };

        loop {
            let mut curr = self.start;
            //loop fall down
            while !self.out_of_bounds(curr) {
                if self.is_supported(curr) {
                    //check left diagonal
                    if !self.is_supported(left(curr)) {
                        curr = left(curr);
                    } else if !self.is_supported(right(curr)) {
                        curr = right(curr);
                    } else {
                        //resting place reached
                        self.grid[curr.0 as usize][curr.1 as usize] = true;
                        sand_counter += 1;
                        break;
                    }
                } else {
                    curr = under(curr);
                }
            }
            if self.out_of_bounds(curr) || self.start_covered() {
                break; //abyss or top reached
            }
        }
        sand_counter
    }

    fn is_supported(&self, mut pos: (i32, i32)) -> bool {
        pos = (pos.0, pos.1 + 1);
        match self.out_of_bounds(pos) {
            true => false,
            false => self.grid[pos.0 as usize][pos.1 as usize],
        }
    }

    fn out_of_bounds(&self, pos: (i32, i32)) -> bool {
        pos.0 < 0 || pos.1 < 0 || pos.0 >= self.shape.0 as i32 || pos.1 >= self.shape.1 as i32
    }

    fn start_covered(&self) -> bool {
        self.grid[self.start.0 as usize][self.start.1 as usize]
    }

    fn add_bedrock(&mut self) {
        //expand the grid to accommodate the bedrock
        let bedrock_level = self.shape.1 + 1;
        let max_width = 4 * bedrock_level; //to be safe
        for column in self.grid.iter_mut() {
            column.extend(vec![false, true]);
        }
        let mut inserts_front = 0;
        let mut inserts_back = 0;
        while self.grid.len() < max_width as usize {
            let mut new_column = vec![false; bedrock_level as usize];
            new_column.push(true);
            match inserts_front <= inserts_back {
                true => {
                    inserts_front += 1;
                    self.grid.insert(0, new_column)
                },
                false => {
                    inserts_back += 1;
                    self.grid.push(new_column)
                },
            }
        }
        self.shape = (max_width, bedrock_level + 1);
        self.start.0 += inserts_front;
    }
}


impl FromStr for Cave {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines();
        let mut rocks = vec![];

        for line in lines {
            let rock_lines = line.split(" -> ").collect::<Vec<&str>>();
            rock_lines.windows(2).for_each(|w| {
                let from = (w[0].split(',').nth(0).unwrap().parse::<usize>().unwrap(), w[0].split(',').nth(1).unwrap().parse::<usize>().unwrap());
                let to = (w[1].split(',').nth(0).unwrap().parse::<usize>().unwrap(), w[1].split(',').nth(1).unwrap().parse::<usize>().unwrap());
                for x in usize::min(from.0, to.0)..=(usize::max(from.0, to.0)) {
                    for y in usize::min(from.1, to.1)..=(usize::max(from.1, to.1)) {
                        rocks.push((x, y));
                    }
                }
            })
        }

        let bbox = rocks.iter().chain(&[(500, 0)]).fold((usize::MAX, usize::MAX, usize::MIN, usize::MIN), |acc, (x, y)| {
            (acc.0.min(*x), acc.1.min(*y), acc.2.max(*x), acc.3.max(*y))
        });

        let mut grid = vec![vec![false; (bbox.3 - bbox.1 + 1) as usize]; (bbox.2 - bbox.0 + 1) as usize];
        rocks.iter().for_each(|(x, y)| grid[(x - bbox.0) as usize][(y - bbox.1) as usize] = true);

        let shape = ((bbox.2 - bbox.0 + 1) as i32, (bbox.3 - bbox.1 + 1) as i32);
        let start = (500 - bbox.0 as i32, 0);

        Ok(
            Cave {
                start,
                shape,
                grid,
            }
        )
    }
}
