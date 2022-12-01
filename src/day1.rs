use std::fs;
use std::path::Path;
use std::str::FromStr;
use aoc2022::parse_to_vec;

const N_HIGHEST_CALORIE_ELVES : u32 = 3;

fn main(){
    let input = fs::read_to_string(Path::new("input/2022/day1.txt")).expect("Could not read file");
    let mut elves : Vec<Elf> = parse_to_vec(&input, "\n\n").unwrap();

    println!("Part 1: {}", elves.iter().map(|e| e.cals).max().unwrap());

    let mut n_most_cals_sum = 0;
    let mut n_most_cal_elves_indices = [elves.len(); N_HIGHEST_CALORIE_ELVES as usize];

    for i in 0..N_HIGHEST_CALORIE_ELVES{
        let (max_cal_elf_index, cals) = elves.iter().enumerate()
            .filter_map(|(i, e)| {
                match n_most_cal_elves_indices.contains(&i) {
                    true => None,
                    false => Some((i, e.cals))
                }
            })
            .max_by(|(_, c1), (_, c2)| c1.cmp(c2))
            .unwrap();
        n_most_cals_sum += cals;
        n_most_cal_elves_indices[i as usize] = max_cal_elf_index;
    }

    println!("Part 2: {}", n_most_cals_sum);
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