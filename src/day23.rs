use std::collections::{HashMap};
use std::time::Instant;
use itertools::Itertools;
use crate::Direction::{N, S, E, W, NE, NW, SE, SW};

const INPUT: &str = include_str!("../input/2022/day23.txt");

pub fn main(){
    let start = Instant::now();
    let mut elves = parse(INPUT);
    elves.simulate(10);
    let part_1 = elves.empty_tiles_in_bbox();
    println!("Part 1: {}", part_1);
    elves.simulate(u32::MAX as usize);
    let part_2 = elves.iterations;
    println!("Part 2: {}", part_2);
    println!("Time: {:?}", start.elapsed());
}

struct Elves{
    locations: HashMap<(i32,i32), Elf>,
    iterations: usize,
}

impl Elves {
    fn simulate(&mut self, n_times: usize){
        for i in self.iterations..(self.iterations + n_times) {
            let proposals = self.locations.values()
                .flat_map(|elf| elf.propose(&self.locations, i))
                .collect_vec();

            let mut proposal_to_map = HashMap::new();
            for Proposal(from, to) in proposals {
                proposal_to_map.insert(to, match proposal_to_map.get(&to) {
                    Some(_) => Err(ConflictingProposals),
                    None => Ok(from),
                });
            }

            let mut no_elves_moved = true;
            for (to, from) in proposal_to_map {
                if let Ok(from) = from {
                    no_elves_moved = false;
                    let mut elf = self.locations.remove(&from).unwrap();
                    elf.move_to(to);
                    assert!(!self.locations.contains_key(&to));
                    self.locations.insert(to, elf);
                }
            }
            self.iterations += 1;
            if no_elves_moved {
                break;
            }
        }
    }

    fn empty_tiles_in_bbox(&self) -> usize {
        let (min_x, max_x, min_y, max_y) = self.locations.keys().fold(
            (i32::MAX,i32::MIN,i32::MAX,i32::MIN), |(min_x, max_x, min_y, max_y), (x, y)| {
            (min_x.min(*x), max_x.max(*x), min_y.min(*y), max_y.max(*y))
        });
        (min_x..=max_x).map(|x| (min_y..=max_y).filter(|&y| !self.locations.contains_key(&(x,y))).count()).sum()
    }
}

struct Proposal((i32,i32), (i32,i32));
struct ConflictingProposals;

struct Elf{
    x : i32,
    y : i32,
}

impl Elf{
    fn propose(&self, others: &HashMap<(i32,i32), Elf>, iteration : usize) -> Option<Proposal> {
        if [N,S,E,W,NE,NW,SE,SW].iter().map(|&d| self.step_in_dir(d)).all(|p| !others.contains_key(&p)) {
            return None;
        }
        const RULES: [([Direction; 3], Direction); 4] = [([N,NE,NW], N),([S,SE,SW], S),([W,NW,SW], W),([E,NE,SE], E)];

        for i in 0..RULES.len() {
            let rule_index = (i + iteration) % RULES.len(); // rotate rules
            let (rule, dir) = &RULES[rule_index];
            if rule.iter().map(|&d| self.step_in_dir(d)).all(|p| !others.contains_key(&p)) {
                return Some(Proposal((self.x, self.y), self.step_in_dir(*dir)));
            }
        }
        None
    }

    fn step_in_dir(&self, dir: Direction) -> (i32, i32){
        match dir{
            N => (self.x, self.y-1),
            S => (self.x, self.y+1),
            W => (self.x-1, self.y),
            E => (self.x+1, self.y),
            NE => (self.x+1, self.y-1),
            NW => (self.x-1, self.y-1),
            SE => (self.x+1, self.y+1),
            SW => (self.x-1, self.y+1),
        }
    }

    fn move_to(&mut self, (x,y): (i32,i32)){
        self.x = x;
        self.y = y;
    }
}

#[derive(Clone, Copy)]
enum Direction{
    N, S, E, W, NE, NW, SE, SW
}

fn parse(input: &str) -> Elves{
    let mut elves = Elves{
        locations: HashMap::new(),
        iterations: 0,
    };
    for (y,line) in input.lines().enumerate(){
        for (x, c) in line.chars().enumerate(){
            if c == '#'{
                elves.locations.insert((x as i32, y as i32), Elf{x: x as i32, y: y as i32});
            }
        }
    }
    elves
}
