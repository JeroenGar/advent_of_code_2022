use std::fs;
use std::str::FromStr;
use aoc2022::parse_to_vec;

fn main(){
    let input = fs::read_to_string("input/2022/day3.txt").expect("Could not read file");

    let rucksacks : Vec<Rucksack> = parse_to_vec(&input, "\n").unwrap();

    let sum_misplaced_items = rucksacks.iter()
        .filter_map(|r| r.find_misplaced_item().ok())
        .map(|item| item_value_mapper(item))
        .sum::<u32>();

    println!("Part 1: {}", sum_misplaced_items);

    let sum_badges = rucksacks.chunks(3)
        .filter_map(|chunk| {
            find_badge(&chunk[0], &chunk[1], &chunk[2]).ok()
        })
        .map(|badge| item_value_mapper(badge)).sum::<u32>();

    println!("Part 2: {}", sum_badges);
}

struct Rucksack{
    comp_1 : Vec<char>, //sorted
    comp_2 : Vec<char>, //sorted
}

impl FromStr for Rucksack{
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() == 0 {
            return Err(());
        }
        let (comp_1, comp_2) = s.split_at(s.len()/2);

        let mut comp_1 : Vec<char> = comp_1.chars().collect();
        let mut comp_2 : Vec<char> = comp_2.chars().collect();

        comp_1.sort();
        comp_2.sort();

        Ok(Rucksack{comp_1, comp_2})
    }
}

impl Rucksack{
    pub fn find_misplaced_item(&self) -> Result<char, ()>{
        let (mut i1, mut i2) = (0,0);

        while i1 < self.comp_1.len() && i2 < self.comp_2.len(){
            let c1 = self.comp_1[i1];
            let c2 = self.comp_2[i2];

            if c1 == c2{
                return Ok(c1)
            }
            else if c1 < c2{
                i1 += 1;
            }else{
                i2 += 1;
            }
        }
        Err(())
    }
}

fn item_value_mapper(c : char) -> u32 {
    if c <= 'z' && c >= 'a' {
        return c as u32 - 'a' as u32 + 1
    }
    if c <= 'Z' && c >= 'A' {
        return c as u32 - 'A' as u32 + 27
    }
    else{
        panic!("invalid char: {}", c)
    }
}

fn find_badge(r1 : &Rucksack, r2 : &Rucksack, r3 : &Rucksack) -> Result<char, ()> {
    let mut r1_content : Vec<&char> = r1.comp_1.iter().chain(r1.comp_2.iter()).collect();
    let mut r2_content : Vec<&char> = r2.comp_1.iter().chain(r2.comp_2.iter()).collect();
    let mut r3_content : Vec<&char> = r3.comp_1.iter().chain(r3.comp_2.iter()).collect();

    r1_content.sort();
    r2_content.sort();
    r3_content.sort();

    let (mut i1, mut i2, mut i3) = (0,0,0);

    while i1 < r1_content.len() && i2 < r2_content.len() && i3 < r3_content.len(){
        let c1 = r1_content[i1];
        let c2 = r2_content[i2];

        if c1 == c2{
            while i3 < r3_content.len() {
                let c3 = r3_content[i3];
                if c3 == c1 {
                    return Ok(c1.clone());
                }
                else if c3 < c1{
                    i3 += 1;
                }
                else{
                    break;
                }
            }
            i1 += 1;
            i2 += 1;
        }
        else if c1 < c2{
            i1 += 1;
        }else{
            i2 += 1;
        }
    }
    Err(())
}

