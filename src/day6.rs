use std::fs;
use std::path::Path;
use std::time::Instant;

use itertools::Itertools;

fn main() {
    //let input = fs::read_to_string(Path::new("input/2022/day6.txt")).unwrap();
    let input = fs::read_to_string(Path::new("/Users/jern/Downloads/aoc22d6xxl.txt")).expect("Could not read file");
    let char_vec = input.chars().filter(|c| c.is_alphabetic()).collect::<Vec<char>>();
    let start = Instant::now();
    println!("Part 1: {}", find_marker_efficient(&char_vec, 4));
    println!("Part 2: {}", find_marker_efficient(&char_vec, 14));
    println!("Time: {}ms", start.elapsed().as_millis());
}

fn find_marker_naive(input: &Vec<char>, n_unique_chars: usize) -> usize {
    //Returns the index after which the first substring containing n unique characters is found
    input.windows(n_unique_chars).enumerate()
        .find(|(_, window)| {
            window.iter().map(|c| c).unique().count() == n_unique_chars
        })
        .map(|(i, _)| i + n_unique_chars).unwrap()
}

fn find_marker_efficient(input: &Vec<char>, n_unique_chars: usize) -> usize{
    let mut windows = input.windows(n_unique_chars);
    let mut marker_index = n_unique_chars;
    while let Some(window) = windows.next() {
        let mut chars_seen = [false; 26];
        let mut skip_n_windows = None;
        for (i,c) in window.iter().enumerate().rev() {
            //Try to find duplicate characters from the back of the window to the front
            //This maximizes the number of windows we can potentially skip
            let char_seen = &mut chars_seen[*c as usize - 'a' as usize];
            match *char_seen {
                false => *char_seen = true,
                true => {
                    skip_n_windows = Some(i);
                    break;
                }
            }
        }
        match skip_n_windows{
            None => return marker_index,
            Some(n) => {
                marker_index += n;
                for _ in 0..n {
                    windows.next();
                }
            }
        }
        marker_index += 1;
    }
    panic!("No substring with {} unique characters found", n_unique_chars);
}