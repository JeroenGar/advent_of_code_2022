use std::collections::VecDeque;
use std::str::FromStr;
use aoc2022::parse_to_vec;

pub fn main(){
    let input = std::fs::read_to_string("input/2022/day5.txt").unwrap();
    let mut input_split = input.split("\n\n");

    let crate_stacks_input = input_split.next().unwrap();
    let crane_operations = input_split.next().unwrap();

    let mut crate_stacks_lines = crate_stacks_input.split('\n').rev().skip(1).collect::<Vec<&str>>();
    let crate_char_indices : [usize; 9] = [1,5,9,13,17,21,25,29,33]; //indices of the crate characters in the lines

    let crate_stacks = (0..9).map(|i|{
        let crates = crate_stacks_lines.iter()
            .map(|line| line.chars().nth(crate_char_indices[i]).unwrap())
            .filter(|c| !c.is_whitespace())
            .collect::<VecDeque<char>>();
        CrateStack{
            crates
        }
    }).collect::<VecDeque<CrateStack>>();

    let crane_operations : Vec<CraneOp> = parse_to_vec(crane_operations, "\n").unwrap();

    //Do the crane operations
    let mut crate_stacks_1 = crate_stacks.clone();
    crane_operations.iter().for_each(|op| {
        let popped_crates = crate_stacks_1.get_mut(op.from).unwrap().pop_n(op.n_crates);
        crate_stacks_1.get_mut(op.to).unwrap().push(popped_crates);
    });

    let mut crate_stacks_2 = crate_stacks.clone();
    crane_operations.iter().for_each(|op| {
        let mut popped_crates = crate_stacks_2.get_mut(op.from).unwrap().pop_n(op.n_crates);
        popped_crates.reverse(); //CrateMover 9001
        crate_stacks_2.get_mut(op.to).unwrap().push(popped_crates);
    });

    let top_crate_to_string = |cs : VecDeque<CrateStack>| {cs.iter().map(|s| *s.crates.get(s.crates.len()-1).unwrap()).collect::<String>()};

    println!("Part 1: {}", top_crate_to_string(crate_stacks_1));
    println!("Part 2: {}", top_crate_to_string(crate_stacks_2));
}

#[derive(Clone)]
pub struct CrateStack{
    crates: VecDeque<char>
}

impl CrateStack {
    pub fn pop_n(&mut self, n : usize) -> Vec<char> {
        let mut crates = Vec::with_capacity(n);
        for _ in 0..n {
            crates.push(self.crates.pop_back().unwrap());
        }
        crates
    }

    pub fn push(&mut self, crates : Vec<char>) {
        for c in crates {
            self.crates.push_back(c);
        }
    }
}

pub struct CraneOp {
    n_crates: usize,
    from: usize,
    to: usize,
}

impl FromStr for CraneOp {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.split(' ').collect::<Vec<&str>>();
        if split.len() != 6 {
            return Err(());
        }

        let n_crates = split.get(1).unwrap().parse::<usize>().unwrap();
        let from = split.get(3).unwrap().parse::<usize>().unwrap() - 1;
        let to = split.get(5).unwrap().parse::<usize>().unwrap() - 1;

        Ok(Self{n_crates, from, to})
    }
}