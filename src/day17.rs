use std::collections::HashMap;
use std::str::FromStr;

use itertools::Itertools;

use aoc2022::parse_to_vec;

const ROCKS: &str = include_str!("../input/2022/day17_rock.txt");
const GAS_PATTERN: &str = include_str!("../input/2022/day17.txt");
const SHAFT_WIDTH: usize = 7;

fn main() {
    let start = std::time::Instant::now();
    let rock_types = parse_to_vec::<RockType>(ROCKS, "\n\n").unwrap();
    let gas_pattern = GAS_PATTERN.trim().chars().collect_vec();

    let part_1 = {
        let mut shaft = Shaft::new(&rock_types, &gas_pattern);
        shaft.simulate(2022);
        shaft.rock_height()
    };

    println!("Part 1: {}", part_1);

    let part_2 = {
        //Search for a two iterations where the shaft and rocks/gas pattern are in the same state
        let ((cycle_start, height_start), (cycle_end, height_end)) = {
            let mut shaft = Shaft::new(&rock_types, &gas_pattern);
            let mut state_map= HashMap::new();

            let mut cycle = None;
            for i in 0..1_000_000_000_000 {
                shaft.simulate(1);
                let minimal_shape = shaft.minimal_shape(); //Use the 'minimal' shape to represent the state
                let state = (minimal_shape, shaft.gas_index, shaft.rock_index);
                if state_map.contains_key(&state) {
                    cycle = Some((state_map[&state], (i, shaft.rock_height())));
                    break;
                }
                state_map.insert(state, (i, shaft.rock_height()));
            }
            cycle.unwrap()
        };

        //---<before>---/-<c>-/-<c>-/ . . . /-<c>-/-<c>-/---<remainder>---/

        let n_cycles = (1_000_000_000_000 - cycle_start) / (cycle_end - cycle_start);
        let remainder = (1_000_000_000_000 - cycle_start) % (cycle_end - cycle_start);

        //Determine which height the remainder contributes
        let remaining_height = {
            let mut shaft = Shaft::new(&rock_types, &gas_pattern);
            shaft.simulate(cycle_end + remainder);
            shaft.rock_height() - height_end
        };
        //Add all of them up to get the total height
        height_start + (height_end - height_start) * n_cycles + remaining_height
    };

    println!("Part 2: {:?}", part_2);
    println!("Time: {:?}", start.elapsed());
}

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

struct Shaft<'a> {
    pub shape: Vec<[bool; SHAFT_WIDTH]>,
    pub rock_types : &'a Vec<RockType>,
    pub rock_index: usize,
    pub gas_pattern: &'a Vec<char>,
    pub gas_index: usize,
}

impl<'a> Shaft<'a> {
    pub fn new(rock_types: &'a Vec<RockType>, gas_pattern: &'a Vec<char>) -> Self {
        Shaft {
            shape: Vec::new(),
            rock_types,
            rock_index: 0,
            gas_pattern,
            gas_index: 0,
        }
    }

    pub fn simulate(&mut self, n_rocks: usize) {
        enum Mode { Falling, Gas, Stuck}

        for _ in 0..n_rocks {
            let rock_type = &self.rock_types[self.rock_index];
            self.rock_index = (self.rock_index + 1) % self.rock_types.len();
            let (mut pos, mut mode) = ((2, self.shape.len() + 3), Mode::Gas); //start state

            loop {
                mode = match mode {
                    Mode::Gas => {
                        let gas = &self.gas_pattern[self.gas_index];
                        self.gas_index = (self.gas_index + 1) % self.gas_pattern.len();
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
                        pos = match self.rock_collides(rock_type, new_pos) {
                            true => pos, //Rock would collide with other rocks, so don't move
                            false => new_pos
                        };
                        Mode::Falling
                    }
                    Mode::Falling => {
                        match pos.1 == 0 {
                            true => Mode::Stuck, //Bottom of shaft reached
                            false => {
                                match pos.1 < self.shape.len() + 1 {
                                    true => { //The rock is in range of other rocks, watch out for collisions
                                        match self.rock_collides(rock_type, (pos.0, pos.1 - 1)) {
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
                    Mode::Stuck => { //Rock has hit something, and is now stuck.
                        rock_type.shape.iter()
                            .map(|(x, y)| (pos.0 + *x as usize, pos.1 + *y as usize))
                            .for_each(|(x, y)| {
                                if self.shape.len() <= y {
                                    self.shape.push([false; SHAFT_WIDTH]);
                                }
                                self.shape[y][x] = true;
                            });
                        break;
                    }
                }
            }
        }
    }

    fn rock_collides(&self, rock_type: &RockType, rel_pos: (usize, usize)) -> bool {
        rock_type.shape.iter()
            .map(|(x, y)| (rel_pos.0 + *x as usize, rel_pos.1 + *y as usize))
            .any(|(x, y)| self.shape.get(y).unwrap_or(&[false; SHAFT_WIDTH])[x])
    }

    pub fn minimal_shape(&self) -> Vec<[bool; SHAFT_WIDTH]>{
        //Only the rows from the top until the row which is fully in the 'shadow' of rocks above are relevant
        let mut sunlight_reach = [true; SHAFT_WIDTH];
        let mut cutoff_index = 0;

        for i in (0..self.shape.len()).rev() {
            (0..SHAFT_WIDTH).filter(|x| self.shape[i][*x]) //Update sunlight map
                .for_each(|x| sunlight_reach[x] = false);
            if sunlight_reach.iter().all(|&x| !x) { //No sunlight left
                cutoff_index = i;
                break;
            }
        }

        (cutoff_index..self.shape.len()).map(|i| self.shape[i]).collect()
    }

    pub fn rock_height(&self) -> usize {
        self.shape.len()
    }
}