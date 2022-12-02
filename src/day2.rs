extern crate core;

use std::fs;
use std::path::Path;
use std::str::FromStr;

use aoc2022::{parse_to_vec};

fn main() {
    let input = fs::read_to_string(Path::new("input/2022/day2.txt")).expect("Could not read file");

    // Part 1
    let mut rounds: Vec<Round> = parse_to_vec(&input, "\n").unwrap();
    println!("Part 1: {}", rounds.iter().map(|r| r.score() as u32).sum::<u32>());

    // Part 2
    // Resolve the misunderstanding
    rounds.iter_mut().for_each(|r| {
        r.1 = match r.1 {
            0 => losing_counter(r.0),
            1 => r.0,
            2 => winning_counter(r.0),
            _ => panic!("invalid input"),
        };
    });

    println!("Part 2: {}", rounds.iter().map(|r| r.score() as u32).sum::<u32>());
}

struct Round(u8, u8);

impl Round {
    fn score(&self) -> u8{
        move_score(self.1) + round_score(self.0, self.1)
    }
}

impl FromStr for Round {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 3 {
            return Err(());
        }
        let opp = s.chars().nth(0).unwrap() as u8 - 65; //ASCII: A = 65
        let you = s.chars().nth(2).unwrap() as u8 - 88; //ASCII: X = 88
        Ok(Round(opp,you))
    }
}

fn losing_counter(m : u8) -> u8 {
    (m + 2) % 3
}

fn winning_counter(m : u8) -> u8 {
    (m + 1) % 3
}

fn move_score(m : u8) -> u8 {
    m + 1
}

fn round_score(opp : u8, you : u8) -> u8 {
    return if opp == you {
        3
    } else if you == winning_counter(opp) {
        6
    } else {
        0
    }
}