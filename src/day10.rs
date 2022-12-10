use std::str::FromStr;

use aoc2022::parse_to_vec;

const INPUT: &str = include_str!("../input/2022/day10.txt");

pub fn main() {
    let instructions = parse_to_vec::<Op>(INPUT, "\n").unwrap();

    let mut cpu = CPU::default();
    let x_history = cpu.run(&instructions);

    println!("Part 1: {}", [20, 60, 100, 140, 180, 220].iter().map(|i| x_history[*i] * (*i as i32)).sum::<i32>());

    println!("Part 2:");
    let x_vec_trimmed = x_history.iter().skip(1).collect::<Vec<&i32>>();
    x_vec_trimmed.chunks(40).for_each(
        |row| {
            for (pixel, &x) in row.iter().enumerate() {
                if [x - 1, *x, x + 1].contains(&(pixel as i32)) {
                    print!("██");
                } else {
                    print!("░░");
                }
            }
            println!();
        }
    )
}

#[derive(Debug, Clone)]
pub enum Op {
    Noop,
    Addx(i32),
}

pub struct CPU {
    pub x: i32,
    pub cycle: usize,
    pub state: CPUState,
}

#[derive(Debug)]
pub enum CPUState {
    Executing(Op, usize)
}

impl FromStr for Op {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(' ');
        let instruction = parts.next().unwrap();
        match instruction {
            "noop" => Ok(Op::Noop),
            "addx" => Ok(Op::Addx(parts.next().unwrap().parse::<i32>().unwrap())),
            _ => Err(()),
        }
    }
}

impl CPU {
    pub fn default() -> Self {
        CPU { x: 1, cycle: 0, state: CPUState::Executing(Op::Noop, 0) }
    }

    pub fn run(&mut self, ops: &[Op]) -> Vec<i32> {
        let mut op_iter = ops.iter();
        let mut x_history = vec![];
        loop {
            x_history.push(self.x);
            self.state = match &self.state {
                CPUState::Executing(op, 0) => {
                    //execute operation
                    match op {
                        Op::Noop => (),
                        Op::Addx(x) => self.x += x,
                    }
                    //fetch next operation
                    match op_iter.next() {
                        Some(Op::Noop) => CPUState::Executing(Op::Noop, 0),
                        Some(Op::Addx(x)) => CPUState::Executing(Op::Addx(*x), 1),
                        None => break,
                    }
                }
                CPUState::Executing(op, cycles_left) => {
                    CPUState::Executing(op.clone(), cycles_left - 1)
                }
            };
            self.cycle += 1;
        }
        x_history
    }
}