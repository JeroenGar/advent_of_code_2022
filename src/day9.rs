use std::collections::{HashSet};
use std::str::FromStr;
use aoc2022::parse_to_vec;

const INPUT: &str = include_str!("../input/2022/day9.txt");

fn main(){
    let head_actions  = parse_to_vec::<Action>(INPUT, "\n").unwrap();
    let mut rope_1 = vec![RopeSegment(0,0); 2];
    let mut rope_2 = vec![RopeSegment(0,0); 10];

    println!("Part 1: {}", simulate(&mut rope_1, &head_actions));
    println!("Part 2: {}", simulate(&mut rope_2, &head_actions));

}

fn simulate(rope: &mut Vec<RopeSegment>, head_actions: &Vec<Action>) -> usize {
    let mut tail_pos_set = HashSet::new();
    for action in head_actions {
        for _ in 0..action.dist{
            tail_pos_set.insert(rope[rope.len()-1].clone());
            let mut rope_iter = rope.iter_mut();
            let mut prev_segment = rope_iter.next().unwrap();
            prev_segment.move_in_dir(&action.dir);
            for segment in rope_iter{
                segment.react(prev_segment);
                prev_segment = segment;
            }
        }
    }
    tail_pos_set.len()
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct RopeSegment(i32,i32);

impl RopeSegment {
    fn move_in_dir(&mut self, dir: &(i32,i32)){
        self.0 += dir.0;
        self.1 += dir.1;
    }

    fn react(&mut self, prev: &RopeSegment){
        let (dx,dy) = ((prev.0 - self.0), (prev.1 - self.1));
        if dx.abs() > 1 || dy.abs() > 1 {
            //not (diagonally) adjacent or overlapping, segment needs to react
            if dx.abs() > 0 {
                self.0 += dx.signum();
            }
            if dy.abs() > 0 {
                self.1 += dy.signum();
            }
        }
    }
}

struct Action {
    dir: (i32,i32),
    dist: usize,
}

impl FromStr for Action {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(' ');
        let dir = match split.next().unwrap() {
            "U" => (0,1),
            "D" => (0,-1),
            "L" => (-1,0),
            "R" => (1,0),
            _ => return Err(())
        };
        let dist = split.next().unwrap().parse::<usize>().unwrap();
        Ok(Action {dir, dist})
    }
}