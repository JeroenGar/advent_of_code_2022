use std::fs;
use std::path::Path;
use std::time::Instant;

use itertools::Itertools;

fn main() {
    let input = fs::read_to_string(Path::new("input/2022/day6.txt")).expect("Could not read file");
    let char_vec = input.chars().collect::<Vec<char>>();
    let start = Instant::now();
    println!("Part 1: {}", find_marker_efficient(&char_vec, 4));
    println!("Part 2: {}", find_marker_efficient(&char_vec, 14));
    println!("Time: {}us", start.elapsed().as_micros());
}

fn find_marker_naive(input: &Vec<char>, n_unique_chars: usize) -> usize {
    //Returns the index after which the first substring containing n unique characters is found

    input.windows(n_unique_chars).enumerate()
        .find(|(i, window)| {
            window.iter().map(|c| c).unique().count() == n_unique_chars
        })
        .map(|(i, _)| i + n_unique_chars).unwrap()
}

fn find_marker_efficient(input: &Vec<char>, n_unique_chars: usize) -> usize {
    let mut windows = input.windows(n_unique_chars);

    let mut marker_index = n_unique_chars;
    while let Some(window) = windows.next() {
        let n_unique_in_window = window.iter().map(|c| c).unique().count();
        if n_unique_in_window == n_unique_chars {
            return marker_index;
        }
        else {
            let n_to_skip = n_unique_chars - n_unique_in_window;
            marker_index += n_to_skip;
            for _ in 0..n_to_skip {
                windows.next();
            }
        }
        marker_index += 1;
    }
    panic!("No substring with {} unique characters found", n_unique_chars);
}