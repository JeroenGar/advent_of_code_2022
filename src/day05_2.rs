use std::str::FromStr;
use std::time::Instant;

use itertools::Itertools;

use aoc2022::parse_to_vec;

const INPUT: &str = include_str!("../input/2022/day05.txt");

pub fn main() {
    let start = Instant::now();
    let mut input_split = INPUT.split("\n\n");

    let crate_stacks_input = input_split.next().unwrap();
    let crane_operations = input_split.next().unwrap();

    let crate_stacks_lines = crate_stacks_input.split('\n').rev().skip(1).collect::<Vec<&str>>();

    let n_crates = (crate_stacks_lines[0].len() + 1) / 4;
    let crate_char_indices = (0..n_crates).map(|i| 1 + (4 * i)).collect::<Vec<usize>>();

    let crate_stacks = (0..n_crates).map(|i| {
        crate_stacks_lines.iter()
            .map(|line| line.chars().nth(crate_char_indices[i]).unwrap())
            .filter(|c| !c.is_whitespace())
            .collect::<Vec<char>>()
    }).collect::<Vec<Vec<char>>>();

    let crane_ops: Vec<CraneOp> = parse_to_vec(crane_operations, "\n").unwrap();
    //We will simulate the crane operations in reverse so we need to reverse the operation and the order
    let rev_crane_ops: Vec<CraneOp> = crane_ops.iter().rev().map(|op| op.reverse()).collect();

    println!("Parsed in {:?}", start.elapsed());

    let start_part_1 = Instant::now();
    //Positions are indexed based on their current stack
    let mut positions_1 = (0..n_crates).map(|i| vec![Position::new(i)]).collect::<Vec<Vec<Position>>>();

    //Execute all crane operations (in reverse)
    solve(&rev_crane_ops, &mut positions_1, true);

    let dur_part_1 = start_part_1.elapsed();

    println!("Part 1: {}", to_string(&positions_1, &crate_stacks));
    println!("in {:?}", dur_part_1);

    //Do the same for part 2 but without reversing the crate order when executing the crane operations
    let start_part_2 = Instant::now();

    let mut positions_2 = (0..n_crates).map(|i| vec![Position::new(i)]).collect::<Vec<Vec<Position>>>();
    solve(&rev_crane_ops, &mut positions_2, false);
    let dur_part_2 = start_part_2.elapsed();

    println!("Part 2: {}", to_string(&positions_2, &crate_stacks));
    println!("in {:?}", dur_part_2);
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
    pub fn reverse(&self) -> Self {
        CraneOp {
            n_crates: self.n_crates,
            from: self.to,
            to: self.from,
        }
    }
}

#[derive(Clone)]
pub struct Position {
    pub org_stack: usize,
    pub curr_stack: usize,
    pub n_crates_on_top: usize,
}

impl Position {
    pub fn new(org_stack: usize) -> Self {
        Self { org_stack, curr_stack: org_stack, n_crates_on_top: 0 }
    }

    pub fn do_operation(&mut self, op: &CraneOp, reverse: bool) {
        if self.curr_stack == op.from {
            //Crates are being moved from this stack
            if self.n_crates_on_top < op.n_crates {
                //Position changes stack
                self.curr_stack = op.to;
                if reverse {
                    self.n_crates_on_top = op.n_crates - self.n_crates_on_top - 1;
                } else {
                    self.n_crates_on_top = self.n_crates_on_top;
                }
            } else {
                //Not enough crates are moved to affect this position's stack
                self.n_crates_on_top -= op.n_crates;
            }
        } else if self.curr_stack == op.to {
            //Crates are being moved on top of this stack
            self.n_crates_on_top += op.n_crates;
        }
    }
}

pub fn solve(crane_ops: &Vec<CraneOp>, positions: &mut Vec<Vec<Position>>, reverse: bool) {
    let mut changed_positions = vec![];
    crane_ops.iter().for_each(|op| {
        //Execute operation in all positions located in the "from" stack
        {
            let pos_in_from = &mut positions[op.from];
            pos_in_from.iter_mut().enumerate().for_each(|(i, p)| {
                debug_assert!(p.curr_stack == op.from);
                p.do_operation(op, reverse);
                if p.curr_stack == op.to {
                    //Position has changed stack
                    changed_positions.push(i);
                }
            });
        }
        //Execute operation in all positions located in the "to" stack
        {
            let pos_in_to = &mut positions[op.to];
            pos_in_to.iter_mut().for_each(|p| {
                debug_assert!(p.curr_stack == op.to);
                p.do_operation(op, reverse);
            });
        }
        //Update the positions vector for the positions which have changed stack
        changed_positions.iter().rev().for_each(|i| {
            let pos = positions[op.from].swap_remove(*i);
            positions[op.to].push(pos);
        });
        changed_positions.clear();
    });
}

pub fn to_string(position: &Vec<Vec<Position>>, crate_stacks: &Vec<Vec<char>>) -> String {
    //Search the correct char for each position and order them in the original stack order
    position.iter().flatten()
        .sorted_by(|a, b| a.org_stack.cmp(&b.org_stack))
        .map(|p| {
            let crate_stack = &crate_stacks[p.curr_stack];
            let crate_char = crate_stack[crate_stack.len() - 1 - p.n_crates_on_top];
            crate_char
        }).collect()
}

