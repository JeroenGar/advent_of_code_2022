use std::fs;
use std::path::Path;
use std::str::{FromStr, Split};

pub fn main() {
    let input = fs::read_to_string(Path::new("input/2022/day4.txt")).expect("Could not read file");
    let range_pairs: Vec<RangePair> = aoc2022::parse_to_vec(&input, "\n").unwrap();

    println!("Part 1: {}", range_pairs.iter().filter(|r| { r.inclusive() }).count());
    println!("Part 2: {}", range_pairs.iter().filter(|r| { r.overlaps() }).count());
}

pub struct RangePair {
    a_min: u32, a_max: u32, b_min: u32, b_max: u32,
}

impl FromStr for RangePair {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(",");
        let mut split_1 = split.next().ok_or(())?.split("-");
        let mut split_2 = split.next().ok_or(())?.split("-");

        let next_u32 = |split: &mut Split<&str>| -> Result<u32, Self::Err> {
            split.next().ok_or(())?.parse::<u32>().map_err(|_| ())
        };

        let (a_min, a_max) = (next_u32(&mut split_1)?, next_u32(&mut split_1)?);
        let (b_min, b_max) = (next_u32(&mut split_2)?, next_u32(&mut split_2)?);
        Ok(RangePair { a_min, a_max, b_min, b_max })
    }
}

impl RangePair {
    pub fn inclusive(&self) -> bool {
        self.a_min <= self.b_min && self.b_max <= self.a_max || self.b_min <= self.a_min && self.a_max <= self.b_max
    }

    pub fn overlaps(&self) -> bool {
        !(self.a_min > self.b_max || self.b_min > self.a_max)
    }
}