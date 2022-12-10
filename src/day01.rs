use std::fs;
use std::path::Path;
use std::str::FromStr;
use aoc2022::parse_to_vec;

const N_HIGHEST_CALORIE_ELVES : u32 = 3;

fn main(){
    let input = fs::read_to_string(Path::new("../input/2022/day01.txt")).expect("Could not read file");
    let mut elves : Vec<Elf> = parse_to_vec(&input, "\n\n").unwrap();

    println!("Part 1: {}", elves.iter().map(|e| e.cals).max().unwrap());

    let (mut lowest_cals, mut lowest_cals_index) = (0, 0);
    let mut most_cal_elves = [0; N_HIGHEST_CALORIE_ELVES as usize];

    elves.iter().for_each(|elf| {
        if elf.cals > lowest_cals {
            most_cal_elves[lowest_cals_index] = elf.cals;
            let new_lowest = most_cal_elves.iter().enumerate().min_by(|(_, a), (_, b)| a.cmp(b)).unwrap();
            lowest_cals = *new_lowest.1;
            lowest_cals_index = new_lowest.0;
        }
    });

    println!("Part 2: {}", most_cal_elves.iter().sum::<u32>());
}


#[derive(Debug)]
struct Elf{
    pub cals : u32
}

impl FromStr for Elf{
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cals = s.split("\n")
            .filter_map(|cal_str| {
                cal_str.parse::<u32>().ok()
            }).sum();
        Ok(Elf{cals})
    }
}