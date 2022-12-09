use std::collections::{HashSet};
use std::str::FromStr;
use aoc2022::parse_to_vec;

const INPUT: &str = include_str!("../input/2022/day9.txt");

fn main(){
    let head_actions  = parse_to_vec::<Action>(INPUT, "\n").unwrap();
    let mut rope_1 = vec![RopeSegment {x:0,y:0}; 2];
    let mut rope_2 = vec![RopeSegment {x:0,y:0}; 10];

    println!("Part 1: {}", simulate(&mut rope_1, &head_actions));
    println!("Part 2: {}", simulate(&mut rope_2, &head_actions));

}

fn simulate(rope: &mut Vec<RopeSegment>, head_actions: &Vec<Action>) -> usize {
    let mut tail_pos_set = HashSet::new();
    for action in head_actions {
        for _ in 0..action.dist{
            {
                let mut rope_iter = rope.iter_mut();
                let mut prev_segment = rope_iter.next().unwrap();
                prev_segment.move_in_dir(&action.dir);
                for segment in rope_iter{
                    segment.react(prev_segment);
                    prev_segment = segment;
                }
            }
            tail_pos_set.insert(rope[rope.len()-1].pos());
        }
    }
    tail_pos_set.len()
}

#[derive(Debug, Clone)]
struct RopeSegment {
    x: i32,
    y: i32,
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

impl RopeSegment {
    fn move_in_dir(&mut self, dir: &(i32,i32)){
        self.x += dir.0;
        self.y += dir.1;
    }

    fn react(&mut self, head: &RopeSegment){
        let (dx,dy) = ((head.x - self.x), (head.y - self.y));
        if dx.abs() > 1 || dy.abs() > 1 {
            //not (diagonally) adjacent or overlapping, needs to react
            if dx.abs() > 0 {
                self.x += dx.signum();
            }
            if dy.abs() > 0 {
                self.y += dy.signum();
            }
        }
    }

    fn pos(&self) -> (i32, i32){
        (self.x, self.y)
    }
}