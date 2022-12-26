use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};
use std::time::Instant;
use itertools::Itertools;

const INPUT: &str = include_str!("../input/2022/day24.txt");

fn main() {
    let start = Instant::now();
    let valley = parse(INPUT);
    let from: (i32, i32) = (0, -1);
    let to: (i32, i32) = (valley.size.0 - 1, valley.size.1);
    let part1 = valley.simulate(0, from, to);
    println!("Part 1: {}", part1);

    let part2 = {
        let back = valley.simulate(part1, to, from);
        valley.simulate(back, from, to)
    };
    println!("Part 2: {}", part2);
    println!("Time: {:?}", start.elapsed());
}

struct Valley {
    blizzards: Vec<Blizzard>,
    column_blizzards : Vec<Vec<usize>>, //indices of blizzards traveling over each column
    row_blizzards : Vec<Vec<usize>>, //indices of blizzards traveling over each row
    size: (i32, i32)
}

impl Valley {
    fn simulate(&self, time: i32, from: (i32, i32), to: (i32, i32)) -> i32 {
        let mut queue = BinaryHeap::new();
        let mut visited = HashSet::new();
        let mut best_time = i32::MAX;
        queue.push(Reverse((manhattan(from, to), from, time)));

        while let Some(Reverse((dist, pos, time))) = queue.pop() {
            if dist > (best_time - time) {
                continue; //can't do better
            }
            if pos == to {
                best_time = best_time.min(time); //arrived
            }

            //consider all neighboring states and add only the relevant ones to the queue
            Self::get_neighbors(pos).into_iter()
                .filter(|&p| self.pos_is_in_valley(p) || p == to || p == from) //position is inside valley
                .filter(|&p| self.pos_is_blizzard_free(p, time + 1)) //no blizzard there
                .for_each(|p| {
                    if !visited.contains(&(p, time + 1)){
                        visited.insert((p, time + 1));
                        //add to the queue sorted by minimimal manhattan distance to the destination
                        queue.push(Reverse((manhattan(p, to), p, time + 1)));
                    }
                });
        }
        best_time
    }

    fn get_neighbors(pos: (i32,i32)) -> [(i32,i32); 5] {
        [
            (pos.0, pos.1), //stay
            (pos.0 - 1, pos.1), //up
            (pos.0 + 1, pos.1), //down
            (pos.0, pos.1 - 1), //left
            (pos.0, pos.1 + 1), //right
        ]
    }

    fn pos_is_in_valley(&self, pos: (i32, i32)) -> bool {
        pos.0 >= 0 && pos.0 < self.size.0 && pos.1 >= 0 && pos.1 < self.size.1
    }

    fn pos_is_blizzard_free(&self, pos: (i32, i32), time: i32) -> bool{
        match self.pos_is_in_valley(pos){
            true => self.column_blizzards[pos.0 as usize].iter()
                .chain(self.row_blizzards[pos.1 as usize].iter())
                .map(|i| &self.blizzards[*i])
                .all(|b| b.pos(time) != pos),
            false => true
        }
    }
}

struct Blizzard {
    start_pos : (i32, i32),
    valley_size: (i32, i32),
    dir: Direction,
}

impl Blizzard {
    fn new(start_pos: (i32,i32), valley_size: (i32, i32), dir: Direction) -> Self {
        Self {
            start_pos,
            valley_size,
            dir
        }
    }

    fn pos(&self, time: i32) -> (i32,i32){
        match &self.dir{
            Direction::Up => (self.start_pos.0, (self.start_pos.1 - time).rem_euclid(self.valley_size.1)),
            Direction::Down => (self.start_pos.0, (self.start_pos.1 + time).rem_euclid(self.valley_size.1)),
            Direction::Left => ((self.start_pos.0 - time).rem_euclid(self.valley_size.0), self.start_pos.1),
            Direction::Right => ((self.start_pos.0 + time).rem_euclid(self.valley_size.0), self.start_pos.1),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn parse(input: &str) -> Valley {
    let mut blizzards = Vec::new();
    let lines = input.lines().collect_vec();
    let valley_size = (lines[0].len() as i32 - 2, lines.len() as i32 - 2);

    for y in 0..lines.len() {
        let line = lines[y];
        for x in 0..line.len(){
            let c = line.chars().nth(x).unwrap();
            let (x, y) = (x as i32 - 1, y as i32 - 1);
            let blizzard = match c {
                '>' => Some(Blizzard::new((x,y), valley_size, Direction::Right)),
                '<' => Some(Blizzard::new((x,y), valley_size, Direction::Left)),
                '^' => Some(Blizzard::new((x,y), valley_size, Direction::Up)),
                'v' => Some(Blizzard::new((x,y), valley_size, Direction::Down)),
                _ => None,
            };
            if let Some(blizzard) = blizzard{
                blizzards.push(blizzard);
            }
        }
    }

    let column_blizzards = (0..valley_size.0).map(|x| blizzards.iter().enumerate()
        .filter(|(_,b)| b.dir == Direction::Up || b.dir == Direction::Down)
        .filter(|(_,b)|  b.start_pos.0 == x)
        .map(|(i,_)| i).collect()).collect_vec();

    let row_blizzards = (0..valley_size.1).map(|y| blizzards.iter().enumerate()
        .filter(|(_,b)| b.dir == Direction::Left || b.dir == Direction::Right)
        .filter(|(_,b)|  b.start_pos.1 == y)
        .map(|(i,_)| i).collect()).collect_vec();

    Valley {
        blizzards,
        column_blizzards,
        row_blizzards,
        size: valley_size
    }
}

fn manhattan(p1: (i32,i32), p2: (i32,i32)) -> i32 {
    (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs()
}