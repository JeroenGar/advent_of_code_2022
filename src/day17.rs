use std::collections::HashMap;
use std::str::FromStr;

use itertools::Itertools;

use aoc2022::parse_to_vec;

const ROCKS: &str = include_str!("../input/2022/day17_rock.txt");
const GAS_PATTERN: &str = include_str!("../input/2022/day17.txt");
const SHAFT_WIDTH: usize = 7;

fn main() {
    let rock_types = parse_to_vec::<RockType>(ROCKS, "\n\n").unwrap();
    let gas_pattern = GAS_PATTERN.trim().chars().collect_vec();

    let part_1 = {
        let mut shaft = vec![];
        simulate(&mut shaft, &rock_types, &mut 0, &gas_pattern, &mut 0, 2022);
        shaft.len()
    };

    println!("Part 1: {}", part_1);

    let part_2 = {
        //search for a two iterations where the shaft and rocks/gas pattern are in the same state
        let ((cycle_start, height_start), (cycle_end, height_end)) = {
            let (mut shaft, mut rock_index, mut gas_index) = (vec![], 0, 0);
            let mut state_map: HashMap<SimState, (usize, usize)> = HashMap::new();

            let mut cycle = None;
            for i in 0..1_000_000_000_000 {
                simulate(&mut shaft, &rock_types, &mut rock_index, &gas_pattern, &mut gas_index, 1);
                let state = SimState::new(&shaft, gas_index, rock_index);
                if state_map.contains_key(&state) {
                    cycle = Some((state_map[&state], (i, shaft.len())));
                    break;
                }
                state_map.insert(state, (i, shaft.len()));
            }
            cycle.unwrap()
        };

        //---<before>---/-<c>-/-<c>-/ . . . /-<c>-/-<c>-/---<remainder>---/

        let n_cycles = (1_000_000_000_000 - cycle_start) / (cycle_end - cycle_start);
        let remainder = (1_000_000_000_000 - cycle_start) % (cycle_end - cycle_start);

        //Determine which height the remainder contributes
        let remaining_height = {
            let (mut shaft, mut rock_index, mut gas_index) = (vec![], 0, 0);
            simulate(&mut shaft, &rock_types, &mut rock_index, &gas_pattern, &mut gas_index, cycle_end + remainder);
            shaft.len() - height_end
        };
        //Add all of them up to get the total height
        height_start + (height_end - height_start) * n_cycles + remaining_height
    };

    println!("Part 2: {:?}", part_2);
}

fn simulate(shaft: &mut Vec<[bool; SHAFT_WIDTH]>, rock_types: &Vec<RockType>, rock_type_index: &mut usize, gas_pattern: &Vec<char>, gas_index: &mut usize, n_rocks: usize) {
    for _ in 0..n_rocks {
        let rock_type = &rock_types[*rock_type_index];
        *rock_type_index = (*rock_type_index + 1) % rock_types.len();
        let (mut pos, mut mode) = ((2, shaft.len() + 3), Mode::Gas); //start state

        loop {
            mode = match mode {
                Mode::Gas => {
                    let gas = &gas_pattern[*gas_index];
                    *gas_index = (*gas_index + 1) % gas_pattern.len();
                    let new_pos = match gas {
                        '>' => {
                            match SHAFT_WIDTH > pos.0 + rock_type.width as usize {
                                true => (pos.0 + 1, pos.1),
                                false => (pos.0, pos.1) //Rock is against the right wall
                            }
                        }
                        '<' => {
                            match pos.0 > 0 {
                                true => (pos.0 - 1, pos.1),
                                false => (pos.0, pos.1) //Rock is against the left wall
                            }
                        }
                        _ => { panic!("Invalid gas character: {}", gas) }
                    };
                    pos = match collides(shaft, rock_type, new_pos) {
                        true => pos, //Rock would collide with other rocks, so don't move
                        false => new_pos
                    };
                    Mode::Falling
                }
                Mode::Falling => {
                    match pos.1 == 0 {
                        true => Mode::Stuck, //Bottom of shaft reached
                        false => {
                            match pos.1 < shaft.len() + 1 {
                                true => { //The rock is in range of other rocks, watch out for collisions
                                    match collides(shaft, rock_type, (pos.0, pos.1 - 1)) {
                                        true => Mode::Stuck,
                                        false => {
                                            pos.1 -= 1;
                                            Mode::Gas
                                        }
                                    }
                                }
                                false => { //The rock is not in range of other rocks, so just fall
                                    pos.1 -= 1;
                                    Mode::Gas
                                }
                            }
                        }
                    }
                }
                Mode::Stuck => { //Rock has hit something, and is now stuck. Add it to the shaft
                    rock_type.shape.iter()
                        .map(|(x, y)| (pos.0 + *x as usize, pos.1 + *y as usize))
                        .for_each(|(x, y)| {
                            if shaft.len() <= y {
                                shaft.push([false; SHAFT_WIDTH]);
                            }
                            shaft[y][x] = true;
                        });
                    break;
                }
            }
        }
    }
}

enum Mode {
    Falling,
    Gas,
    Stuck,
}

//bottom left is (0,0)
struct RockType {
    pub shape: Vec<(u8, u8)>,
    pub width: u8,
}

impl FromStr for RockType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut shape = Vec::new();
        for (y, line) in s.lines().rev().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    shape.push((x as u8, y as u8));
                }
            }
        }
        let width = shape.iter().map(|(x, _)| x).max().unwrap() + 1;
        Ok(RockType { shape, width })
    }
}

fn collides(shaft: &[[bool; SHAFT_WIDTH]], rock_type: &RockType, pos: (usize, usize)) -> bool {
    rock_type.shape.iter()
        .map(|(x, y)| (pos.0 + *x as usize, pos.1 + *y as usize))
        .any(|(x, y)| shaft.get(y).unwrap_or(&[false; SHAFT_WIDTH])[x])
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct SimState {
    pub shaft: Vec<[bool; SHAFT_WIDTH]>,
    pub gas: usize,
    pub rock: usize,
}

impl SimState {
    pub fn new(shaft: &Vec<[bool; SHAFT_WIDTH]>, gas: usize, rock: usize) -> Self {
        SimState {
            shaft: minimal_representation_shaft(shaft),
            gas,
            rock,
        }
    }
}

fn minimal_representation_shaft(shaft: &Vec<[bool; SHAFT_WIDTH]>) -> Vec<[bool; SHAFT_WIDTH]> {
    //Only the rows from the top until the row which is fully in the 'shadow' of rocks above are relevant
    let mut sunlight_reach = [true; SHAFT_WIDTH];
    let mut cutoff_index = 0;

    for i in (0..shaft.len()).rev() {
        (0..SHAFT_WIDTH).filter(|x| shaft[i][*x]) //Update sunlight map
            .for_each(|x| sunlight_reach[x] = false);
        if sunlight_reach.iter().all(|&x| !x) { //No sunlight left
            cutoff_index = i;
            break;
        }
    }

    (cutoff_index..shaft.len()).map(|i| shaft[i]).collect()
}

fn print_shaft(shaft: &[[bool; SHAFT_WIDTH]]) {
    for row in shaft.iter().rev() {
        for col in row.iter() {
            print!("{}", if *col { '#' } else { '.' });
        }
        println!();
    }
}