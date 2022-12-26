use std::str::FromStr;

use itertools::Itertools;
use num::integer::Roots;
use scan_fmt::scan_fmt;

use crate::CubeSide::{Bottom, East, North, South, Top, West};
use crate::Orient::{Down, Left, Right, Up};

const INPUT: &str = include_str!("../input/2022/day22.txt");
const SIDE_LOCATIONS: [(usize, usize); 6] = [(1, 1), (0, 3), (1, 0), (2, 0), (1, 2), (0, 2)];

//HARD CODED FOR REAL INPUT

//      +---+---+
//      | N | E |
//      +---+---+
//      | T |
//  +---+---+
//  | W | S |
//  +---+---+
//  | B |
//  +---+

fn main() {
    let cube = INPUT.split("\n\n").next().unwrap().parse::<Cube>().unwrap();
    let instructions = parse_instructions(INPUT.split("\n\n").last().unwrap());

    println!("Part 2: {}", simulate(&cube, &instructions));
}

fn simulate(cube: &Cube, instructions: &Vec<Instr>) -> usize {
    let (mut side, mut row, mut col, mut orient) = cube.start();

    for instr in instructions.iter() {
        match instr {
            Instr::Turn(rot) => {
                orient = orient.turn(rot);
            }
            Instr::Move(n) => {
                for _ in 0..*n {
                    let new_pos = cube.go((side, row, col, orient));
                    if new_pos == (side, row, col, orient) {
                        break;
                    }
                    (side, row, col, orient) = new_pos;
                }
            }
        }
    }
    let dim = cube.dim;
    let (col_extra, row_extra) = match side {
        Top => SIDE_LOCATIONS[0],
        Bottom => SIDE_LOCATIONS[1],
        North => SIDE_LOCATIONS[2],
        East => SIDE_LOCATIONS[3],
        South => SIDE_LOCATIONS[4],
        West => SIDE_LOCATIONS[5],
    };

    let (row, col) = (row + row_extra * dim, col + col_extra * dim);

    let facing = match orient {
        Right => 0,
        Down => 1,
        Left => 2,
        Up => 3,
    };
    1000 * (row + 1) + 4 * (col + 1) + facing
}

struct OutOfBoundsErr;

struct Cube {
    dim: usize,
    sides: Vec<Square>
}

impl Cube {
    fn start(&self) -> (CubeSide, usize, usize, Orient) {
        (North, 0, 0, Right)
    }

    fn go(&self, (side, row, col, orient): (CubeSide, usize, usize, Orient)) -> (CubeSide, usize, usize, Orient) {
        match self.sides[side as usize].go((row, col), orient) {
            Ok((n_row, n_col)) => {
                match self.sides[side as usize].is_wall((n_row, n_col)) {
                    true => (side, row, col, orient),
                    false => (side, n_row, n_col, orient),
                }
            }
            Err(OutOfBoundsErr) => {
                let new = self.switch_sides((side, row, col, orient));
                match self.sides[new.0 as usize].is_wall((new.1, new.2)) {
                    true => (side, row, col, orient),
                    false => new,
                }
            }
        }
    }

    fn switch_sides(&self, (side, row, col, orient): (CubeSide, usize, usize, Orient)) -> (CubeSide, usize, usize, Orient) {
        let last = self.dim - 1;
        let (flipped_row, flipped_col) = (last - row, last - col);
        match side {
            Top => {
                match orient {
                    Up => (North, last, col, Up),
                    Down => (South, 0, col, Down),
                    Left => (West, 0, row, Down),
                    Right => (East, last, row, Up),
                }
            }
            Bottom => {
                match orient {
                    Up => (West, last, col, Up),
                    Down => (East, 0, col, Down),
                    Left => (North, 0, row, Down),
                    Right => (South, last, row, Up),
                }
            }
            North => {
                match orient {
                    Up => (Bottom, col, 0, Right),
                    Down => (Top, 0, col, Down),
                    Left => (West, flipped_row, 0, Right),
                    Right => (East, row, 0, Right),
                }
            }
            East => {
                match orient {
                    Up => (Bottom, last, col, Up),
                    Down => (Top, col, last, Left),
                    Left => (North, row, last, Left),
                    Right => (South, flipped_row, last, Left),
                }
            }
            South => {
                match orient {
                    Up => (Top, last, col, Up),
                    Down => (Bottom, col, last, Left),
                    Left => (West, row, last, Left),
                    Right => (East, flipped_row, last, Left),
                }
            }
            West => {
                match orient {
                    Up => (Top, col, 0, Right),
                    Down => (Bottom, 0, col, Down),
                    Left => (North, flipped_row, 0, Right),
                    Right => (South, row, 0, Right),
                }
            }
        }
    }
}

struct Square {
    dim: usize,
    rows: Vec<Vec<bool>>,
}

impl Square {
    fn go(&self, from: (usize, usize), orient: Orient) -> Result<(usize, usize), OutOfBoundsErr> {
        let to = match orient {
            Up => (from.0.checked_sub(1).ok_or(OutOfBoundsErr)?, from.1),
            Down => (from.0 + 1, from.1),
            Left => (from.0, from.1.checked_sub(1).ok_or(OutOfBoundsErr)?),
            Right => (from.0, from.1 + 1),
        };

        if to.0 >= self.dim || to.1 >= self.dim {
            Err(OutOfBoundsErr)
        } else {
            Ok(to)
        }
    }
    fn is_wall(&self, (row, col): (usize, usize)) -> bool {
        self.rows[row][col]
    }
}

impl FromStr for Cube {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines().collect_vec();
        let dim = (lines.iter().map(|l| l.trim().len()).sum::<usize>() / 6).sqrt();

        let sides = SIDE_LOCATIONS.iter().map(|(x, y)| {
            let x_range = (x * dim)..((x + 1) * dim);
            let y_range = (y * dim)..((y + 1) * dim);
            let square = y_range.map(|i| &lines[i][x_range.clone()]).collect_vec();
            parse_square(&square)
        }).collect_vec();

        Ok(Cube {
            dim,
            sides
        })
    }
}

fn parse_square(lines: &Vec<&str>) -> Square {
    let dim = lines[0].trim().len();
    let rows = lines.iter().map(|line| {
        line.trim().chars().map(|c| c == '#').collect_vec()
    }).collect_vec();

    Square {
        dim,
        rows,
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
        match scan_fmt!(&remaining, "{}R{}", usize, String) {
            Ok((n, s)) => {
                instructions.push(Instr::Move(n));
                instructions.push(Instr::Turn(Rot::Cw));
                remaining = s;
            }
            Err(_) => {
                match scan_fmt!(&remaining, "{}L{}", usize, String) {
                    Ok((n, s)) => {
                        instructions.push(Instr::Move(n));
                        instructions.push(Instr::Turn(Rot::Ccw));
                        remaining = s;
                    }
                    Err(_) => {
                        match scan_fmt!(&remaining, "{}", usize) {
                            Ok(n) => {
                                instructions.push(Instr::Move(n));
                                remaining = "".to_owned();
                            }
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
                Up => Right,
                Right => Down,
                Down => Left,
                Left => Up
            },
            Rot::Ccw => match self {
                Up => Left,
                Left => Down,
                Down => Right,
                Right => Up
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CubeSide {
    Top,
    //0
    Bottom,
    //1
    North,
    //2
    East,
    //3
    South,
    //4
    West,
    //5
}

#[cfg(test)]
mod tests {
    use crate::{Cube, INPUT};
    use crate::CubeSide::{Bottom, East, North, South, Top, West};
    use crate::Orient::{Down, Left, Right, Up};

    #[test]
    fn transformation_table_is_reversible() {
        let cube = INPUT.split("\n\n").next().unwrap().parse::<Cube>().unwrap();

        let sides = [Top, Bottom, North, East, South, West];
        let flip_orient = |o| {
            match o {
                Up => Down,
                Down => Up,
                Left => Right,
                Right => Left,
            }
        };

        let last = cube.dim - 1;

        for side in sides {
            let checks = [(side, 0,1, Up), (side, last, 1, Down), (side, 1, 0, Left), (side, 1, last, Right)];

            for side in sides {
                let checks = [(side, 0,1, Up), (side, last, 1, Down), (side, 1, 0, Left), (side, 1, last, Right)];

                for check in checks {
                    let old_pos = check;
                    assert!(cube.sides[old_pos.0 as usize].go((old_pos.1, old_pos.2), old_pos.3).is_err());
                    let new_pos = cube.switch_sides(old_pos);
                    assert!(cube.sides[new_pos.0 as usize].go((new_pos.1, new_pos.2), flip_orient(new_pos.3)).is_err());
                    let should_be_old_pos = cube.switch_sides((new_pos.0, new_pos.1, new_pos.2, flip_orient(new_pos.3)));
                    assert_eq!((old_pos.0, old_pos.1, old_pos.2, flip_orient(old_pos.3)), should_be_old_pos);
                }
            }
        }
    }
}