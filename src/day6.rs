use std::fs;
use std::ops::Range;
use std::path::Path;
use std::time::Instant;

use itertools::Itertools;

fn main() {
    let input = std::fs::read_to_string("input/2022/day6.txt").unwrap();
    println!("{} chars", input.len());
    let char_vec = input.chars().filter(|c| {
        *c as u32 >= 32 && *c as u32 <= 126
    }).collect::<Vec<char>>();

    println!("Part 1: {}", find_marker_naive(&char_vec, 4).unwrap());
    println!("Part 2: {}", find_marker_naive(&char_vec, 14).unwrap());
    println!();
    //Extra
    let input = std::fs::read_to_string("input/2022/day6_extra.txt").unwrap();
    println!("{} chars", input.len());
    let char_vec = input.chars().filter(|c| {
        *c as u32 >= 32 && *c as u32 <= 126
    }).collect::<Vec<char>>();
    let n_threads = 8;

    println!("{} threads", n_threads);

    let start = Instant::now();
    println!("Part 1: {} ({}µs)", find_range_markers_mt(&char_vec, 94..95, n_threads).iter().sum::<usize>(), start.elapsed().as_micros());

    let start = Instant::now();
    println!("Part 2: {} ({}µs)", find_range_markers_mt(&char_vec, 1..95, n_threads).iter().sum::<usize>(), start.elapsed().as_micros());
}

fn find_marker_naive(input: &Vec<char>, n_unique_chars: usize) -> Option<usize> {
    //Returns the index after which the first substring containing n unique characters is found
    input.windows(n_unique_chars).enumerate()
        .find(|(_, window)| {
            window.iter().map(|c| c).unique().count() == n_unique_chars
        })
        .map(|(i, _)| i + n_unique_chars)
}

fn find_marker_efficient(input: &Vec<char>, start: usize, stop: usize, n_unique_chars: usize) -> Option<usize>{
    let mut last_seen = [None; 95];
    let mut potential_start = start as i64 -1;
    for i in start..stop{
        let c = input[i];
        let index = c as usize - ' ' as usize;
        let prev_last_seen = last_seen[index];
        match prev_last_seen{
            None => (),
            Some(prev_i) => {
                if i < prev_i + n_unique_chars{
                    //This character makes the substring not unique, the next potential start is after the previous occurrence of this character
                    potential_start = i64::max(potential_start, prev_i as i64);
                }
            }
        }
        last_seen[index] = Some(i);
        if i == (potential_start + n_unique_chars as i64) as usize{
            return Some((potential_start + n_unique_chars as i64) as usize + 1);
        }
    }
    None
}

fn find_range_markers_mt(input: &Vec<char>, range: Range<usize>, n_threads: usize) -> Vec<usize> {
    let blocks = create_blocks(input.len(), range.end, n_threads);
    let mut results = (0..n_threads).map(|_| vec![]).collect::<Vec<Vec<Option<usize>>>>();

    rayon::scope(|s| {
        let mut results_iter_mut = results.iter_mut();
        for i in 0..n_threads {
            let (mut start,end) = blocks[i];
            let mut result_slice = results_iter_mut.next().unwrap();
            let t_range = range.clone();
            s.spawn(move |_| {
                for j in t_range{
                    let result = find_marker_efficient(input, start, end, j);
                    if result.is_some(){
                        start = result.unwrap() - j;
                    }
                    else{
                        start = end;
                    }
                    result_slice.push(result);
                }
            });
        }
    });
    (0..range.len()).map(|i| {
        results.iter().filter_map(|r| r[i]).min().unwrap()
    }).collect()
}


fn create_blocks(input_len : usize, n_unique_chars: usize, n_threads: usize) -> Vec<(usize,usize)>{
    let overlap = n_unique_chars - 1;
    (0..n_threads).map(|i| {
        let start = (i * (input_len) / n_threads);
        let end = ((i + 1) * (input_len) / n_threads) + overlap;
        (usize::max(0,start), usize::min(end,input_len))
    }).collect::<Vec<(usize, usize)>>()
}
