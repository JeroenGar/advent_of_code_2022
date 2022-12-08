use std::str::FromStr;

const INPUT: &str = include_str!("../input/2022/day8.txt");

fn main() {
    let start = std::time::Instant::now();
    let tree_grid: TreeGrid = INPUT.parse().unwrap();

    let visibility_matrix = tree_grid.create_visibility_matrix();
    let scenic_matrix = tree_grid.create_scenic_score_matrix();

    let n_visible = visibility_matrix.iter().flatten().filter(|v| **v).count();
    let max_scenic = scenic_matrix.iter().flatten().max().unwrap();

    println!("Time: {}μs", start.elapsed().as_micros());
    println!("Part 1: {}", n_visible);
    println!("Part 2: {}", max_scenic);
}


struct TreeGrid {
    grid: Vec<Vec<u8>>,
}

impl FromStr for TreeGrid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut grid = vec![];
        for line in s.lines() {
            if !line.is_empty() {
                let grid_line = line.chars().map(|c| c.to_string().parse::<u8>().unwrap()).collect::<Vec<u8>>();
                grid.push(grid_line);
            }
        }
        Ok(TreeGrid { grid })
    }
}

impl TreeGrid {
    fn create_visibility_matrix(&self) -> Vec<Vec<bool>> {
        let (width, height) = (self.grid[0].len(), self.grid.len());
        let mut visibility_matrix = vec![vec![false; width]; height];

        //resolve up visibility
        for x in 0..width {
            let mut highest_visible = -1;
            for y in 0..height {
                let tree_height = self.grid[y][x];
                if tree_height as i32 > highest_visible {
                    visibility_matrix[y][x] = true;
                    highest_visible = tree_height as i32;
                }
            }
        }

        //resolve right visibility
        for y in 0..height {
            let mut highest_visible = -1;
            for x in (0..width).rev() {
                let tree_height = self.grid[y][x];
                if tree_height as i32 > highest_visible {
                    visibility_matrix[y][x] = true;
                    highest_visible = tree_height as i32;
                }
            }
        }

        //resolve down visibility
        for x in 0..width {
            let mut highest_visible = -1;
            for y in (0..height).rev() {
                let tree_height = self.grid[y][x];
                if tree_height as i32 > highest_visible {
                    visibility_matrix[y][x] = true;
                    highest_visible = tree_height as i32;
                }
            }
        }

        //resolve left visibility
        for y in 0..height {
            let mut highest_visible = -1;
            for x in 0..width {
                let tree_height = self.grid[y][x];
                if tree_height as i32 > highest_visible {
                    visibility_matrix[y][x] = true;
                    highest_visible = tree_height as i32;
                }
            }
        }

        visibility_matrix
    }

    fn create_scenic_score_matrix(&self) -> Vec<Vec<usize>> {
        let (width, height) = (self.grid[0].len(), self.grid.len());

        let mut scenic_scores = vec![vec![1; width]; height];

        //resolve up visibility
        for x in 0..width {
            let mut last_seen_buffer = [0;10];
            for y in 0..height {
                let tree_height = self.grid[y][x] as usize;
                scenic_scores[y][x] *= y - last_seen_buffer[tree_height];
                for i in 0..=tree_height {
                    last_seen_buffer[i] = y;
                }
            }
        }

        //resolve right visibility
        for y in 0..height {
            let mut last_seen_buffer = [width-1;10];
            for x in (0..width).rev() {
                let tree_height = self.grid[y][x] as usize;
                scenic_scores[y][x] *= last_seen_buffer[tree_height] - x;
                for i in 0..=tree_height {
                    last_seen_buffer[i] = x;
                }
            }
        }

        //resolve down visibility
        for x in 0..width {
            let mut last_seen_buffer = [height-1;10];
            for y in (0..height).rev() {
                let tree_height = self.grid[y][x] as usize;
                scenic_scores[y][x] *= last_seen_buffer[tree_height] - y;
                for i in 0..=tree_height {
                    last_seen_buffer[i] = y;
                }
            }
        }

        //resolve left visibility
        for y in 0..height {
            let mut last_seen_buffer = [0;10];
            for x in 0..width {
                let tree_height = self.grid[y][x] as usize;
                scenic_scores[y][x] *= x - last_seen_buffer[tree_height];
                for i in 0..=tree_height {
                    last_seen_buffer[i] = x;
                }
            }
        }

        scenic_scores
    }
}