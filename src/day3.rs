use std::fs;
use std::str::FromStr;

use aoc2022::parse_to_vec;

fn main() {
    let input = fs::read_to_string("input/2022/day3.txt").expect("Could not read file");
    let rucksacks: Vec<Rucksack> = parse_to_vec(&input, "\n").unwrap();

    let sum_misplaced_items = rucksacks.iter()
        .map(|r| r.misplaced_item.unwrap())
        .sum::<u32>();

    println!("Part 1: {}", sum_misplaced_items);

    let sum_badges = rucksacks.chunks(3)
        .map(|chunk| {
            find_badge(&chunk[0], &chunk[1], &chunk[2]).unwrap()
        }).sum::<u32>();

    println!("Part 2: {}", sum_badges);
}

struct Rucksack {
    pub item_present: [bool; 53],
    pub misplaced_item: Option<u32>,
}

impl FromStr for Rucksack {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() == 0 {
            return Err(());
        }
        let (s1, s2) = s.split_at(s.len() / 2);
        let (mut item_in_c1, mut item_in_c2) = ([false; 53], [false; 53]);

        s1.chars().for_each(|c| item_in_c1[(item_value_mapper(c))] = true);
        s2.chars().for_each(|c| item_in_c2[(item_value_mapper(c))] = true);

        let misplaced_item = item_in_c1.iter()
            .zip(item_in_c2.iter())
            .zip(0..53)
            .find(|((a, b), i)| **a && **b)
            .map(|(_, i)| i);

        let item_present = item_in_c1.iter().zip(item_in_c2.iter())
            .map(|(a, b)| *a || *b)
            .collect::<Vec<bool>>().try_into().unwrap();

        Ok(Rucksack { item_present, misplaced_item })
    }
}

fn item_value_mapper(c: char) -> usize {
    if c <= 'z' && c >= 'a' {
        return c as usize - 'a' as usize + 1;
    }
    if c <= 'Z' && c >= 'A' {
        return c as usize - 'A' as usize + 27;
    } else {
        panic!("invalid char: {}", c)
    }
}

fn find_badge(r1: &Rucksack, r2: &Rucksack, r3: &Rucksack) -> Result<u32, ()> {
    for i in 0..53 {
        if r1.item_present[i] && r2.item_present[i] && r3.item_present[i] {
            return Ok((i) as u32);
        }
    }
    Err(())
}

