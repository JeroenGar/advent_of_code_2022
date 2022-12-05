use std::collections::VecDeque;
use std::str::FromStr;
use std::time::Instant;
use itertools::Itertools;

use aoc2022::parse_to_vec;

pub fn main() {
    let start = Instant::now();
    let input = std::fs::read_to_string("input/2022/day5_210MB.txt").unwrap();
    let mut input_split = input.split("\n\n");

    let crate_stacks_input = input_split.next().unwrap();
    let crane_operations = input_split.next().unwrap();

    let mut crate_stacks_lines = crate_stacks_input.split('\n').rev().skip(1).collect::<Vec<&str>>();

    let n_crates = (crate_stacks_lines[0].len() + 1) / 4;
    let crate_char_indices = (0..n_crates).map(|i| 1 + (4* i)).collect::<Vec<usize>>();

    let crate_stacks = (0..n_crates).map(|i| {
        let crates = crate_stacks_lines.iter()
            .map(|line| line.chars().nth(crate_char_indices[i]).unwrap())
            .filter(|c| !c.is_whitespace())
            .collect::<Vec<char>>();
        CrateStack {
            crates
        }
    }).collect::<Vec<CrateStack>>();

    let crane_operations: Vec<CraneOp> = parse_to_vec(crane_operations, "\n").unwrap();
    let reversed_flipped_crane_ops: Vec<CraneOp> = crane_operations.iter().rev().map(|op| op.flip()).collect();

    println!("Parsed in {}ms", start.elapsed().as_millis());

    let start_part_1 = Instant::now();

    // the final positions of chars we have to print
    let mut positions_1 = (0..n_crates).map(|i| Position::new(i, 0)).collect::<Vec<Position>>();

    //Do the crane ops in reverse
    reversed_flipped_crane_ops.iter().for_each(|op| {
        positions_1.iter_mut().for_each(|p| {
            p.do_operation(op, true);
        });
    });

    let dur_part_1 = start_part_1.elapsed();

    let find_chars = |pos : Vec<Position>| -> String {
        pos.iter()
            .map(|p| {
                let crate_stack = &crate_stacks[p.stack_index];
                let crate_char = crate_stack.crates[crate_stack.crates.len() - 1 - p.n_crates_on_top];
                crate_char
            }).collect()
    };

    println!("Part 1: {} in {}ms", find_chars(positions_1), dur_part_1.as_millis());

    let start_part_2 = Instant::now();

    let mut positions_2 = (0..n_crates).map(|i| Position::new(i, 0)).collect::<Vec<Position>>();

    //Do the crane ops in reverse
    reversed_flipped_crane_ops.iter().for_each(|op| {
        positions_2.iter_mut().for_each(|p| {
            p.do_operation(op, false);
        });
    });

    let dur_part_2 = start_part_2.elapsed();

    println!("Part 2: {} in {}ms", find_chars(positions_2), dur_part_2.as_millis());
}

#[derive(Clone)]
pub struct CrateStack {
    crates: Vec<char>,
}

pub struct CraneOp {
    n_crates: usize,
    from: usize,
    to: usize,
}

impl FromStr for CraneOp {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.split(' ').collect::<Vec<&str>>();
        if split.len() != 6 { return Err(()); }

        let n_crates = split.get(1).unwrap().parse::<usize>().unwrap();
        let from = split.get(3).unwrap().parse::<usize>().unwrap() - 1;
        let to = split.get(5).unwrap().parse::<usize>().unwrap() - 1;

        Ok(Self { n_crates, from, to })
    }
}

impl CraneOp {
    pub fn flip(&self) -> Self {
        CraneOp {
            n_crates: self.n_crates,
            from: self.to,
            to: self.from,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Position {
    pub stack_index: usize,
    pub n_crates_on_top: usize,
}

impl Position {
    pub fn new(stack_index: usize, n_crates_on_top: usize) -> Self {
        Self { stack_index, n_crates_on_top }
    }

    pub fn do_operation(&mut self, op: &CraneOp, reverse: bool) {
        if op.from == self.stack_index {
            //Crates are being moved from this stack
            if self.n_crates_on_top < op.n_crates {
                //The crate being is moved
                self.stack_index = op.to;
                if reverse {
                    self.n_crates_on_top = op.n_crates - self.n_crates_on_top - 1;
                } else {
                    self.n_crates_on_top = self.n_crates_on_top;
                }
            } else {
                // Not enough crates are moved to affect this position
                self.n_crates_on_top -= op.n_crates;
            }
        } else if op.to == self.stack_index {
            //Crates are being moved to this stack
            self.n_crates_on_top += op.n_crates;
        }
    }
}