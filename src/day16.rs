use std::cmp::{Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::str::FromStr;
use std::time::Instant;

use itertools::Itertools;

const INPUT: &str = include_str!("../input/2022/day16.txt");

fn main() {
    let start = Instant::now();
    let network = INPUT.parse::<Network>().unwrap();

    let part_1 = {
        let mut active_valves = vec![false; network.valves.len()];
        active_valves[network.start] = true;
        dfs(&network, network.start, 0, 0, 30, &mut active_valves)
    };
    println!("Part 1: {}", part_1);

    let part_2 = {
        let non_zero_pressure_valves = network.valves.iter().enumerate()
            .filter(|(_, v)| v.pressure_rate != 0).map(|(i, _)| i).collect_vec();
        let mut already_seen = HashSet::new();
        //Collect all unique (dual) subsets of active valves
        let mut groups : Vec<(Vec<bool>, Vec<bool>)> = Vec::new();
        for i in 1..=non_zero_pressure_valves.len() {
            for group in non_zero_pressure_valves.iter().combinations(i) {
                let mut this = vec![false; network.valves.len()];
                group.iter().for_each(|i| this[**i] = true);
                let that = this.iter().enumerate().map(|(i,&b)| {
                    match non_zero_pressure_valves.contains(&i) {
                        false => false,
                        true => !b,
                    }
                }).collect_vec();
                if !already_seen.contains(&this){
                    already_seen.insert(this.clone());
                    already_seen.insert(that.clone());
                    groups.push((this, that));
                }
            }
        }
        let n_groups = groups.len();
        let mut counter = 0;
        groups.drain(..).map(|(mut me, mut elephant)|{
            if counter % 100 == 0 {
                println!("{} / {}", counter, n_groups);
            }
            counter += 1;

            //Find the optimal path for each group
            me[network.start] = true;
            elephant[network.start] = true;
            dfs(&network, network.start, 0, 0, 26, &mut me) +
                dfs(&network, network.start, 0, 0, 26, &mut elephant)
        }).max().unwrap()
    };

    println!("Part 2: {}", part_2);
    println!("Time: {:?}", start.elapsed());
}

struct Network {
    pub valves: Vec<Valve>,
    pub sp_map: HashMap<(usize, usize), usize>,
    pub start: usize,
}

struct Valve {
    pressure_rate: usize,
}

impl FromStr for Network {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let valves = s.lines()
            .map(|l| l.split(';').next().unwrap())
            .map(|l| l.parse::<Valve>().unwrap())
            .collect::<Vec<Valve>>();
        let name_map = s.lines().enumerate()
            .map(|(i, l)| (l.split(' ').nth(1).unwrap().to_string(), i))
            .collect::<HashMap<String, usize>>();

        let edges = s.lines()
            .map(|l| {
                l.split(' ').skip(9)
                    .map(|c| c.strip_suffix(',').unwrap_or(c))
                    .map(|c| *name_map.get(c).unwrap())
                    .collect::<Vec<usize>>()
            }).collect();

        let sp_map = shortest_path_matrix(&edges);
        let start = *name_map.get("AA").unwrap();

        Ok(Network { valves, sp_map, start })
    }
}

impl FromStr for Valve {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let flow_rate = s.split('=').nth(1).unwrap().parse::<usize>().unwrap();
        Ok(Valve { pressure_rate: flow_rate })
    }
}

fn shortest_path_matrix(edges: &Vec<Vec<usize>>) -> HashMap<(usize, usize), usize> {
    //Floyd-Warshall would be faster here, but I'm lazy and this is certainly not the bottleneck today...
    let mut shortest_paths = HashMap::new();
    for start in 0..edges.len() {
        for end in start + 1..edges.len() {
            let sp = dijkstra(edges, start, end);
            shortest_paths.insert((start, end), sp);
            shortest_paths.insert((end, start), sp);
        }
    }
    shortest_paths
}

fn dijkstra(edges: &Vec<Vec<usize>>, start: usize, end: usize) -> usize {
    let mut distances = vec![usize::MAX; edges.len()];
    distances[start] = 0;
    let mut queue = BinaryHeap::new();
    queue.push(Reverse((0, start))); //Reverse to make it a min heap

    while let Some(Reverse((distance, node))) = queue.pop() {
        if node == end {
            break;
        }
        for to in &edges[node] {
            let new_distance = distance + 1;
            if new_distance < distances[*to] {
                distances[*to] = new_distance;
                queue.push(Reverse((new_distance, *to)));
            }
        }
    }
    distances[end]
}

fn dfs(network: &Network, location: usize, mut total_pressure: usize, mut pressure_rate: usize, mut time_left: usize, active_valves: &mut Vec<bool>) -> usize {
    if time_left == 0 {
        return total_pressure;
    }
    total_pressure += pressure_rate;

    //open the valve
    if !active_valves[location] {
        active_valves[location] = true;
        time_left -= 1;
        pressure_rate += network.valves[location].pressure_rate;
    }

    //eventual pressure if we would stay here and do nothing
    let mut max_pressure = total_pressure + (pressure_rate * time_left);

    for valve in 0..network.valves.len() {
        if !active_valves[valve] && network.valves[valve].pressure_rate > 0 {
            //visit the inactive valves
            let travel_time = network.sp_map[&(location, valve)];
            if time_left > travel_time {
                let result = dfs(
                    network,
                    valve,
                    total_pressure + (pressure_rate * travel_time),
                    pressure_rate,
                    time_left - travel_time,
                    active_valves,
                );
                max_pressure = max_pressure.max(result);
            }
        }
    }

    active_valves[location] = false;

    max_pressure
}
