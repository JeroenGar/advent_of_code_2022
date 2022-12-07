use std::iter::Peekable;

pub fn main() {
    let (mut lines, mut sum) = (include_str!("../input/2022/day7.txt").lines().peekable(), 0);
    let start = std::time::Instant::now();
    sh(&mut lines, &mut sum);
    println!("Time: {}μs", start.elapsed().as_micros());
    println!("{}", sum);
}

fn sh(lines: &mut Peekable<impl Iterator<Item = &'static str>>, sum: &mut usize) -> usize {
    let mut size = 0;
    while let Some(i) = lines.next() {
        match i {
            "$ cd .." => break,
            _ if i.starts_with("$ l") => {
                size = std::iter::from_fn(|| lines.next_if(|i| !i.starts_with('$')))
                    .filter(|i| !i.starts_with('d'))
                    .map(|i| i.split(' ').next().unwrap().parse::<usize>().unwrap())
                    .sum()
            }
            _ => size += sh(lines, sum),
        }
    }
    if size <= 100_000 {
        *sum += size;
    }
    size
}