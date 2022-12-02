extern crate core;

use std::fs;
use std::path::Path;
use std::str::FromStr;

use aoc2022::{parse_to_vec};

fn main() {
    let input = fs::read_to_string(Path::new("input/2022/day2.txt")).expect("Could not read file");

    // Part 1
    let mut rounds: Vec<Round> = parse_to_vec(&input, "\n").unwrap();
    println!("Part 1: {}", rounds.iter().map(|r| r.score()).sum::<u32>());

    // Part 2
    //Resolve the misunderstanding
    rounds.iter_mut().for_each(|r| {
        let r_result = match r.you {
            Move::Rock => RoundResult::Loss,
            Move::Paper => RoundResult::Draw,
            Move::Scissors => RoundResult::Win,
        };
        r.you = r.opp.resulting_move(&r_result);
    });

    println!("Part 2: {}", rounds.iter().map(|r| r.score()).sum::<u32>());
}

struct Round {
    opp : Move,
    you : Move
}

impl Round {
    fn score(&self) -> u32{
        self.opp.round_result(&self.you).score() + self.you.score()
    }
}

impl FromStr for Round {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(" ");
        let opp = match split.next().unwrap() {
            "A" => Move::Rock,
            "B" => Move::Paper,
            "C" => Move::Scissors,
            _ => return Err(())
        };
        let you = match split.next().unwrap() {
            "X" => Move::Rock,
            "Y" => Move::Paper,
            "Z" => Move::Scissors,
            _ => return Err(())
        };
        Ok(Round {opp, you})
    }
}

#[derive(PartialEq, Clone)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    pub fn score(&self) -> u32 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3
        }
    }

    pub fn round_result(&self, you: &Move) -> RoundResult {
        if self == you {
            RoundResult::Draw
        } else if self.winning_counter_move() == *you {
            RoundResult::Win
        } else {
            RoundResult::Loss
        }
    }

    pub fn resulting_move(&self, result: &RoundResult) -> Move {
        match result {
            RoundResult::Win => self.winning_counter_move(),
            RoundResult::Draw => self.clone(),
            RoundResult::Loss => self.losing_counter_move()
        }
    }

    pub fn winning_counter_move(&self) -> Move {
        match self {
            Move::Rock => Move::Paper,
            Move::Paper => Move::Scissors,
            Move::Scissors => Move::Rock
        }
    }

    pub fn losing_counter_move(&self) -> Move {
        match self {
            Move::Rock => Move::Scissors,
            Move::Paper => Move::Rock,
            Move::Scissors => Move::Paper
        }
    }
}

enum RoundResult {
    Win,
    Draw,
    Loss,
}

impl RoundResult {
    fn score(&self) -> u32{
        match self {
            RoundResult::Win => 6,
            RoundResult::Draw => 3,
            RoundResult::Loss => 0
        }
    }
}