use std::collections::HashSet;
use std::hash::BuildHasherDefault;
use std::str::FromStr;

use fxhash::{FxHasher, FxHashSet};
use itertools::Itertools;

const INPUT: &str = include_str!("../input/2022/day18.txt");

fn main() {
    let start = std::time::Instant::now();
    let droplet = INPUT.parse::<Droplet>().unwrap();

    println!("Part 1: {}", droplet.total_surface_area());
    println!("Part 2: {}", droplet.exterior_surface_area());
    println!("Time: {:?}", start.elapsed());
}


struct Droplet {
    cubes: Vec<(i32, i32, i32)>,
}

impl Droplet {
    fn total_surface_area(&self) -> usize {
        let mut cube_set = FxHashSet::default();
        self.cubes.iter().for_each(|c| { cube_set.insert(c); });

        self.cubes.iter().map(|c| {
            get_neighbors(*c).iter().filter(|n| !cube_set.contains(n)).count()
        }).sum()
    }

    fn exterior_surface_area(&self) -> usize {
        let mut cube_set = FxHashSet::default();
        self.cubes.iter().for_each(|c| { cube_set.insert(c); });
        let (x_min, x_max) = self.cubes.iter().map(|c| c.0).minmax().into_option().unwrap();
        let (y_min, y_max) = self.cubes.iter().map(|c| c.1).minmax().into_option().unwrap();
        let (z_min, z_max) = self.cubes.iter().map(|c| c.2).minmax().into_option().unwrap();

        self.cubes.iter().map(|c| {
            get_neighbors(*c).iter()
                .filter(|n| !cube_set.contains(n))
                .filter(|n| cube_exterior(**n, ((x_min, y_min, z_min), (x_max, y_max, z_max)), &cube_set))
                .count()
        }).sum()
    }
}

fn get_neighbors(c: (i32, i32, i32)) -> [(i32, i32, i32); 6] {
    let (x, y, z) = c;
    [
        (x + 1, y, z),
        (x - 1, y, z),
        (x, y + 1, z),
        (x, y - 1, z),
        (x, y, z + 1),
        (x, y, z - 1),
    ]
}

fn cube_exterior(c: (i32, i32, i32), bbox: ((i32, i32, i32), (i32, i32, i32)), cube_set: &HashSet<&(i32, i32, i32), BuildHasherDefault<FxHasher>>) -> bool {
    //Flood fill until we reach a cube outside the bbox, or have explored all neighbors
    //There is probably some heuristic to optimize the candidate order, but this is fast enough
    let mut next_candidates = vec![c];
    let mut visited = FxHashSet::default();

    while !next_candidates.is_empty() {
        let current = next_candidates.pop().unwrap();
        if visited.contains(&current) {
            continue;
        }
        visited.insert(current);
        let (x, y, z) = current;
        if x < bbox.0.0 || x > bbox.1.0 || y < bbox.0.1 || y > bbox.1.1 || z < bbox.0.2 || z > bbox.1.2 {
            return true;
        }
        if !cube_set.contains(&current) {
            get_neighbors(current).iter().for_each(|n| {
                next_candidates.push(*n);
            });
        }
    }
    false
}

impl FromStr for Droplet {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cubes = s.lines().filter_map(
            |line| {
                match line.is_empty() {
                    true => None,
                    false => {
                        let split = line.split(',').collect_vec();
                        Some((split[0].parse().unwrap(), split[1].parse().unwrap(), split[2].parse().unwrap()))
                    }
                }
            }
        ).collect_vec();
        Ok(Droplet { cubes })
    }
}