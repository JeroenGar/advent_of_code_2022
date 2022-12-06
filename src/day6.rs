use std::fs;
use std::path::Path;
use std::time::Instant;

use itertools::Itertools;

fn main() {
    let input = fs::read_to_string(Path::new("input/2022/day6.txt")).unwrap();
    let char_vec = input.chars().filter(|c| c.is_alphabetic()).collect::<Vec<char>>();
    let start = Instant::now();
    println!("Part 1: {}", find_marker_multithreaded(&char_vec, 4, 8));
    println!("Part 2: {}", find_marker_multithreaded(&char_vec, 14, 8));
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

fn find_marker_more_efficient(input: &Vec<char>, start: usize, stop: usize, n_unique_chars: usize) -> Option<usize>{
    let mut last_seen = [None; 26];
    let mut potential_start = 0;
    for i in start..stop{
        let c = input[i];
        let index = c as usize - 'a' as usize;
        let prev_last_seen = last_seen[index];
        match prev_last_seen{
            None => (),
            Some(prev_i) => {
                if i < prev_i + n_unique_chars{
                    //This characters makes the substring not unique
                    potential_start = usize::max(potential_start, prev_i);
                }
            }
        }
        last_seen[index] = Some(i);
        if i == potential_start + n_unique_chars{
            return Some(potential_start + n_unique_chars + 1);
        }
    }
    None
}


fn find_marker_multithreaded(input: &Vec<char>, n_unique_chars: usize, n_threads: usize) -> usize{
    let n_chars = input.len();
    let overlap = n_unique_chars - 1;
    let indices = (0..n_threads).map(|i| {
        let start = (i * (n_chars) / n_threads);
        let end = ((i + 1) * (n_chars) / n_threads) + overlap;
        (usize::max(0,start), usize::min(end,n_chars))
    }).collect::<Vec<(usize, usize)>>();

    let mut results = (0..n_threads).map(|_| None).collect::<Vec<Option<usize>>>();

    rayon::scope(|s| {
        let mut results_iter_mut = results.iter_mut();
        for i in 0..n_threads {
            let (start,end) = indices[i];
            let mut result_slice = results_iter_mut.next().unwrap();
            s.spawn(move |_| {
                *result_slice = find_marker_more_efficient(input, start, end, n_unique_chars);
            });
        }
    });

    results.into_iter().filter_map(|x| x).min().unwrap()
}