use std::str::FromStr;
use std::time::Instant;

use fxhash::{FxHashMap};
use itertools::{Itertools, izip};

use aoc2022::parse_to_vec;

const INPUT: &str = include_str!("../input/2022/day19.txt");

fn main() {
    let start = Instant::now();
    let blueprints = parse_to_vec::<Blueprint>(INPUT, "\n").unwrap();
    let part_1 = {
        let mut results = vec![None; blueprints.len()];
        rayon::scope(|s| {
            for (i, result) in results.iter_mut().enumerate() {
                let blueprint = &blueprints[i];
                s.spawn(move |_| {
                    *result = Some(blueprint.simulate(SimState::new(24, [1, 0, 0, 0], [0;4])));
                });
            }
        });
        results.iter().enumerate().map(|(i, r)| (i + 1) * r.unwrap() as usize).sum::<usize>()
    };

    println!("Part 1: {}", part_1);

    let part_2 = {
        let mut results = vec![None; 3];
        rayon::scope(|s| {
            for (i, result) in results.iter_mut().enumerate() {
                let blueprint = &blueprints[i];
                s.spawn(move |_| {
                    *result = Some(blueprint.simulate(SimState::new(32, [1, 0, 0, 0], [0;4])));
                });
            }
        });
        results.iter().fold(1, |acc, r| acc * r.unwrap() as usize)
    };

    println!("Part 2: {}", part_2);
    println!("Time: {}ms", start.elapsed().as_millis());
}

pub struct Blueprint {
    prices: [[u32; 4]; 4],
}

impl FromStr for Blueprint {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(());
        }
        let r_split = s.split("Each").skip(1).collect_vec();
        let mut prices = [[0; 4]; 4];
        let p_split = r_split[0].trim().split(' ').collect_vec();
        prices[0] = [p_split[3].trim().parse().unwrap(), 0, 0, 0];
        let p_split = r_split[1].trim().split(' ').collect_vec();
        prices[1] = [p_split[3].trim().parse().unwrap(), 0, 0, 0];
        let p_split = r_split[2].trim().split(' ').collect_vec();
        prices[2] = [p_split[3].trim().parse().unwrap(), p_split[6].trim().parse().unwrap(), 0, 0];
        let p_split = r_split[3].trim().split(' ').collect_vec();
        prices[3] = [p_split[3].trim().parse().unwrap(), 0, p_split[6].trim().parse().unwrap(), 0];

        Ok(Blueprint { prices })
    }
}

impl Blueprint {

    fn simulate(&self, start_state: SimState) -> u32 {
        //BFS, with pruning of already seen combinations of robots and resources
        let mut seen_states = FxHashMap::default();
        let mut states_stack = Vec::new();
        let mut max_geodes = 0;
        states_stack.push(start_state);

        while let Some(state) = states_stack.pop() {
            match state.time {
                0 => max_geodes = max_geodes.max(state.resources[3]),
                _ => {
                    self.get_next_states(&state).into_iter()
                        .for_each(|next_state| {
                            if self.upperbound_geodes(&next_state) > max_geodes {
                                match seen_states.get(&(next_state.bots, next_state.resources)){
                                    Some(&seen_time) => {
                                        if seen_time < next_state.time {
                                            //If we've seen this combination before, but with less time left, add it to the stack
                                            seen_states.insert((next_state.bots, next_state.resources), next_state.time);
                                            states_stack.push(next_state);
                                        }
                                    },
                                    None => {
                                        //If this combination is unseen, add it to the stack
                                        seen_states.insert((next_state.bots, next_state.resources), next_state.time);
                                        states_stack.push(next_state);
                                    }
                                }
                            }
                        });
                }
            }
        }
        max_geodes
    }

    fn get_next_states(&self, state: &SimState) -> Vec<SimState> {
        let mut next_states = vec![];

        for i in (0..4).rev() { //reverse order, so the most advanced machines come first
            if i != 3 && state.bots[i] == self.prices.iter().map(|p| p[i]).max().unwrap() {
                //Since we can only build one machine every iteration,
                // we don't need more production of a resource than the most expensive machine requires
                continue;
            }
            let price = &self.prices[i];
            if izip!(state.resources.iter(), price.iter())
                .all(|(r, p)| *r >= *p) {
                //If we have enough resources to build a machine
                let next_time = state.time - 1;
                let next_bots = {
                    let mut next_bots = state.bots;
                    next_bots[i] += 1;
                    next_bots
                };
                let next_resources = {
                    let mut next_resources = state.resources;
                    for (r, p, b) in izip!(next_resources.iter_mut(), price.iter(), state.bots.iter()) {
                        *r = *r + *b - *p;
                    }
                    next_resources
                };
                next_states.push(SimState::new(next_time, next_bots, next_resources));
                if i == 3 {
                    //If we can make a geode machine, all other options are irrelevant
                    return next_states;
                }
            }
        }

        //Add a state for waiting
        let mut wait_state = *state;
        wait_state.time -= 1;
        wait_state.resources.iter_mut().zip(wait_state.bots.iter()).for_each(|(r, b)| *r += *b);
        next_states.push(wait_state);

        next_states
    }

    fn upperbound_geodes(&self, state: &SimState) -> u32 {
        //Assume a new geode machine is produced every iteration until time runs out
        state.resources[3] +
            state.time * state.bots[3] +
            (0..state.time).sum::<u32>()
    }
}

#[derive(Clone, Copy)]
struct SimState {
    time: u32,
    bots: [u32; 4],
    resources: [u32; 4]
}

impl SimState{
    pub fn new(time: u32, bots: [u32; 4], resources: [u32; 4]) -> Self {
        Self { time, bots, resources }
    }
}