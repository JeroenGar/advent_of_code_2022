use std::fs;
use std::path::Path;

use itertools::Itertools;

fn main() {
    let input = fs::read_to_string(Path::new("input/2022/day6.txt")).expect("Could not read file");
    println!("Part 1: {}", find_marker(&input, 4));
    println!("Part 2: {}", find_marker(&input, 14));
}

fn find_marker(input: &str, n_unique_chars: usize) -> usize {
    //Returns the index after which the first substring containing n unique characters is found
    let char_vec = input.chars().collect::<Vec<char>>();

    char_vec.windows(n_unique_chars).enumerate()
        .find(|(i, window)| {
            window.iter().map(|c| c).unique().count() == n_unique_chars
        })
        .map(|(i, _)| i + n_unique_chars).unwrap()
}