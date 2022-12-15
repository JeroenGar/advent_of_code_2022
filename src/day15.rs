use std::ops::RangeInclusive;
use std::str::FromStr;
use std::time::Instant;
use itertools::Itertools;

use aoc2022::parse_to_vec;

const INPUT: &str = include_str!("../input/2022/day15.txt");

fn main() {
    let start = Instant::now();
    let sensors: Vec<Sensor> = parse_to_vec(INPUT, "\n").unwrap();

    let part1 = {
        let row = 2000000;
        //Each sensor has a (or no) range of blocked locations on this row
        let ranges = sensors.iter()
            .filter_map(|s| match s.y_in_range(row) {
                true => Some(s.min_x_in_range(row).unwrap()..=s.max_x_in_range(row).unwrap()),
                false => None,
            }).collect::<Vec<RangeInclusive<i32>>>();
        //merge overlapping ranges
        let merged_ranges = merge_overlapping(&ranges);
        //count elements in the ranges and subtract the number of known beacons inside the ranges
        let n_in_range = merged_ranges.iter().map(|r| r.end() - r.start() + 1).sum::<i32>();

        let beacons_on_row = sensors.iter().filter(|s| s.b_y == row).map(|s| s.b_x).unique().collect::<Vec<i32>>();
        let n_beacons_in_range = beacons_on_row.iter().filter(|x| merged_ranges.iter().any(|r| r.contains(x))).count() as i32;
        n_in_range - n_beacons_in_range
    };

    println!("Part 1: {}", part1);

    let part2 = {
        let range = 4000000;
        let (c_x,c_y) = (range/2, range/2);

        //filter sensors that are out of range
        let relevant_sensors = sensors.iter()
            .filter(|s| s.r + range >= (s.x - c_x).abs() + (s.y - c_y).abs())
            .collect::<Vec<&Sensor>>();

        //Since there is only a single place for the distress beacon,
        //its distance must be r+1 from some sensor, only take into account these points
        let loc = relevant_sensors.iter()
            .flat_map(|s| s.coords_with_distance(s.r + 1))
            .filter(|(x, y)| (0..=range).contains(x) && (0..=range).contains(y))
            .find(|(x, y)| relevant_sensors.iter().all(|s| !s.in_range(*x, *y)))
            .unwrap();

        loc.0 as usize * range as usize + loc.1 as usize
    };

    println!("Part 2: {}", part2);
    println!("Time: {}us", start.elapsed().as_micros());
}

#[derive(Debug, Clone)]
struct Sensor {
    x: i32, y: i32, r: i32, b_x: i32, b_y: i32,
}

impl Sensor {
    fn in_range(&self, x: i32, y: i32) -> bool {
        (self.x - x).abs() + (self.y - y).abs() <= self.r
    }
    fn y_in_range(&self, y: i32) -> bool {
        (self.y - y).abs() <= self.r
    }
    fn min_x_in_range(&self, y: i32) -> Option<i32> {
        match self.r - (self.y - y).abs() {
            r if r < 0 => None,
            r => Some(self.x - r),
        }
    }
    fn max_x_in_range(&self, y: i32) -> Option<i32> {
        match self.r - (self.y - y).abs() {
            r if r < 0 => None,
            r => Some(self.x + r),
        }
    }
    fn coords_with_distance(&self, d: i32) -> Vec<(i32, i32)> {
        (self.x - d..=self.x + d).map(|x| (x, self.y - (d - (x - self.x).abs()))).collect()
    }
}

impl FromStr for Sensor {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let splitted = s.split(' ').collect::<Vec<&str>>();
        if splitted.len() != 10 { return Err(()); }
        let x = splitted[2][2..splitted[2].len() - 1].parse::<i32>().unwrap();
        let y = splitted[3][2..splitted[3].len() - 1].parse::<i32>().unwrap();
        let b_x = splitted[8][2..splitted[8].len() - 1].parse::<i32>().unwrap();
        let b_y = splitted[9][2..splitted[9].len()].parse::<i32>().unwrap();
        let r = (x - b_x).abs() + (y - b_y).abs();
        Ok(Sensor { x, y, r, b_x, b_y })
    }
}

fn merge_overlapping(ranges: &[RangeInclusive<i32>]) -> Vec<RangeInclusive<i32>> {
    let mut result = Vec::from(ranges);
    let overlap = |r1: &RangeInclusive<i32>, r2: &RangeInclusive<i32>| { r1.start().max(r2.start()) <= r1.end().min(r2.end()) };

    let mut overlap_detected = true;

    'overlap: while overlap_detected {
        overlap_detected = false;
        for i in 0..result.len() {
            for j in i + 1..result.len() {
                if overlap(&result[i], &result[j]) {
                    result[i] = *result[i].start().min(result[j].start())..=*result[i].end().max(result[j].end());
                    result.remove(j);
                    overlap_detected = true;
                    continue 'overlap;
                }
            }
        }
    }
    result
}