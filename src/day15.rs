use std::str::FromStr;
use std::time::Instant;

use aoc2022::parse_to_vec;

const INPUT: &str = include_str!("../input/2022/day15.txt");

fn main() {
    let start = Instant::now();
    let sensors: Vec<Sensor> = parse_to_vec(INPUT, "\n").unwrap();

    let row = 2000000;

    let min_x_in_range = sensors.iter()
        .filter_map(|s| s.min_x_in_range(row))
        .min().unwrap();

    let max_x_in_range = sensors.iter()
        .filter_map(|s| s.max_x_in_range(row))
        .max().unwrap();

    let relevant_sensors = sensors.iter().filter(|s| s.min_x_in_range(row).is_some()).collect::<Vec<&Sensor>>();
    let beacons_on_row = sensors.iter().filter(|s| s.b_y == row).map(|s| s.b_x).collect::<Vec<i32>>();

    let part1 = (min_x_in_range..=max_x_in_range)
        .filter(|x| relevant_sensors.iter()
            .any(|s| s.in_range(*x, row)))
        .filter(|x| !beacons_on_row.contains(x))
        .count();

    println!("Part 1: {}", part1);

    let range = 4000000;
    let (c_x,c_y) = (range/2, range/2);

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

    println!("Part 2: {}", loc.0 as usize * range as usize + loc.1 as usize);
    println!("Time: {}ms", start.elapsed().as_millis());
}

#[derive(Debug, Clone)]
struct Sensor {
    x: i32, y: i32, r: i32, b_x: i32, b_y: i32,
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

impl Sensor {
    fn in_range(&self, x: i32, y: i32) -> bool {
        (self.x - x).abs() + (self.y - y).abs() <= self.r
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