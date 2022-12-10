use std::str::FromStr;
use std::time::Instant;
use fxhash::FxHashSet;
use aoc2022::parse_to_vec;

const INPUT: &str = include_str!("../input/2022/day09.txt");

fn main(){
    let start = Instant::now();
    let head_actions  = parse_to_vec(INPUT, "\n").unwrap();
    let mut rope = vec![RopeSegment(0,0); 10];

    let (part_1, part_2) = simulate(&mut rope, &head_actions);
    let duration = start.elapsed();

    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);
    println!("Time: {}ms", duration.as_millis());
}

fn simulate(rope: &mut [RopeSegment], head_actions: &[Action]) -> (usize,usize) {
    let mut pos_1_set = FxHashSet::default();
    let mut pos_9_set = FxHashSet::default();
    for action in head_actions {
        for _ in 0..action.dist {
            let mut rope_iter = rope.iter_mut();
            let mut prev_segment = rope_iter.next().unwrap();
            prev_segment.move_in_dir(&action.dir);
            for segment in rope_iter {
                segment.react(prev_segment);
                prev_segment = segment;
            }
            pos_1_set.insert(rope[1].clone());
            pos_9_set.insert(rope[9].clone());
        }
    }
    (pos_1_set.len(),pos_9_set.len())
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
            self.0 += dx.signum();
            self.1 += dy.signum();
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