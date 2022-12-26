use std::str::FromStr;

use itertools::Itertools;
use scan_fmt::scan_fmt;

const INPUT: &str = include_str!("../input/2022/day22.txt");

fn main() {
    let maze = INPUT.split("\n\n").next().unwrap().parse::<Maze>().unwrap();
    let instructions = parse_instructions(INPUT.split("\n\n").last().unwrap());

    println!("Part 1: {}", simulate(&maze, &instructions));
}

fn simulate(maze: &Maze, instructions: &Vec<Instr>) -> usize {
    let mut orient = Orient::Right;
    let mut pos = maze.start();

    for instr in instructions.iter() {
        dbg!(instr);
        match instr {
            Instr::Turn(rot) => {
                orient = orient.turn(rot);
                dbg!(&orient);
            },
            Instr::Move(n) => {
                for _ in 0..*n {
                    let new_pos = maze.go(pos, &orient);
                    if new_pos == pos {
                        break;
                    }
                    pos = new_pos;
                    dbg!(pos);
                }
            }
        }
    }
    let facing = match orient{
        Orient::Right => 0,
        Orient::Down => 1,
        Orient::Left => 2,
        Orient::Up => 3,
    };
    1000 * (pos.0 + 1) + 4 * (pos.1 + 1) + facing
}

struct Maze {
    rows: Vec<Row>,
}

impl Maze {
    fn start(&self) -> (usize, usize) {
        (0, self.rows[0].offset)
    }

    fn go(&self, pos: (usize, usize), orient: &Orient) -> (usize, usize) {
        match orient {
            Orient::Up => (self.go_up(pos), pos.1),
            Orient::Down => (self.go_down(pos), pos.1),
            Orient::Left => (pos.0, self.rows[pos.0].go_left(pos.1).unwrap()),
            Orient::Right => (pos.0, self.rows[pos.0].go_right(pos.1).unwrap()),
        }
    }

    fn go_up(&self, pos: (usize, usize)) -> usize {
        let next_row = (pos.0 as i32 - 1).rem_euclid(self.rows.len() as i32) as usize;
        match self.rows[next_row].is_free(pos.1) {
            Ok(true) => next_row,
            Ok(false) => pos.0,
            Err(_) => {
                //move down row by row (wrapping around) until we find a row where pos.1 does not exist, then move up one row
                let wrapped_row = (0..self.rows.len())
                    .map(|i| (pos.0 as i32 + i as i32).rem_euclid(self.rows.len() as i32) as usize)
                    .tuple_windows()
                    .find(|(_, i)| self.rows[*i].is_free(pos.1).is_err())
                    .map(|(i, _)| i)
                    .unwrap();
                match self.rows[wrapped_row].is_free(pos.1) {
                    Ok(true) => wrapped_row,
                    Ok(false) => pos.0,
                    Err(_) => panic!("wrapped row should exist at pos.1")
                }
            }
        }
    }

    fn go_down(&self, pos: (usize, usize)) -> usize {
        let next_row = (pos.0 as i32 + 1).rem_euclid(self.rows.len() as i32) as usize;
        match self.rows[next_row].is_free(pos.1) {
            Ok(true) => next_row,
            Ok(false) => pos.0,
            Err(_) => {
                //move down row by row (wrapping around) until we find a row where pos.1 does not exist, then move up one row
                let wrapped_row = (0..self.rows.len())
                    .map(|i| (pos.0 as i32 - i as i32).rem_euclid(self.rows.len() as i32) as usize)
                    .tuple_windows()
                    .find(|(_, i)| self.rows[*i].is_free(pos.1).is_err())
                    .map(|(i, _)| i)
                    .unwrap();
                match self.rows[wrapped_row].is_free(pos.1) {
                    Ok(true) => wrapped_row,
                    Ok(false) => pos.0,
                    Err(_) => panic!("wrapped row should exist at pos.1")
                }
            }
        }
    }
}

struct Row {
    offset: usize,
    walls: Vec<bool>,
}

impl Row {
    fn go_right(&self, pos: usize) -> Result<usize, ()> {
        let new_pos = (pos as i32 + 1 - self.offset as i32).rem_euclid(self.walls.len() as i32) as usize + self.offset;
        match self.is_free(new_pos)? {
            true => Ok(new_pos),
            false => Ok(pos),
        }
    }

    fn go_left(&self, pos: usize) -> Result<usize, ()> {
        let new_pos = (pos as i32 - 1 - self.offset as i32).rem_euclid(self.walls.len() as i32) as usize + self.offset;
        match self.is_free(new_pos)? {
            true => Ok(new_pos),
            false => Ok(pos),
        }
    }

    fn is_free(&self, pos: usize) -> Result<bool, ()> {
        match pos >= self.offset && pos < self.offset + self.walls.len() {
            true => Ok(!self.walls[pos - self.offset]),
            false => Err(())
        }
    }
}

impl FromStr for Maze {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rows = s.lines().filter_map(|line| line.parse::<Row>().ok()).collect_vec();
        Ok(Maze { rows })
    }
}

impl FromStr for Row {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(());
        }
        let offset = s.len() - s.trim().len();
        let walls = s.trim().chars().map(|c| c == '#').collect_vec();

        Ok(Row { offset, walls })
    }
}

#[derive(Debug)]
enum Instr {
    Turn(Rot),
    Move(usize),
}

fn parse_instructions(s: &str) -> Vec<Instr> {
    let mut remaining = s.trim().to_owned();
    let mut instructions = Vec::new();
    while !remaining.is_empty() {
        match scan_fmt!(&remaining, "{}R{}", usize, String){
            Ok((n,  s)) => {
                instructions.push(Instr::Move(n));
                instructions.push(Instr::Turn(Rot::Cw));
                remaining = s;
            },
            Err(_) => {
                match scan_fmt!(&remaining, "{}L{}", usize, String){
                    Ok((n, s)) => {
                        instructions.push(Instr::Move(n));
                        instructions.push(Instr::Turn(Rot::Ccw));
                        remaining = s;
                    },
                    Err(_) => {
                        match scan_fmt!(&remaining, "{}", usize){
                            Ok(n) => {
                                instructions.push(Instr::Move(n));
                                remaining = "".to_owned();
                            },
                            Err(_) => panic!("could not parse {}", remaining)
                        }
                    }
                }
            }
        }
    }
    instructions
}

#[derive(Debug)]
enum Rot {
    Cw,
    Ccw,
}

#[derive(Debug)]
enum Orient {
    Up,
    Down,
    Left,
    Right,
}

impl Orient {
    fn turn(&self, turn: &Rot) -> Self {
        match turn {
            Rot::Cw => match self {
                Orient::Up => Orient::Right,
                Orient::Right => Orient::Down,
                Orient::Down => Orient::Left,
                Orient::Left => Orient::Up
            },
            Rot::Ccw => match self {
                Orient::Up => Orient::Left,
                Orient::Left => Orient::Down,
                Orient::Down => Orient::Right,
                Orient::Right => Orient::Up
            }
        }
    }
}


