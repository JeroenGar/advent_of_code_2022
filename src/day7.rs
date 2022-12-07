use std::slice::Iter;

fn main() {
    let input = std::fs::read_to_string("input/2022/day7.txt").unwrap();
    let lines = input.split('\n').collect::<Vec<&str>>();
    let start = std::time::Instant::now();

    let mut lines_iter = lines.iter();
    lines_iter.next(); //skip first line

    let mut all_dirs = vec![];
    let root_dir = parse_dir(&mut lines_iter, &mut all_dirs);

    let part_1 = all_dirs.iter()
        .filter(|d| **d <= 100_000).sum::<usize>();

    let space_required = 30_000_000 - (70_000_000 - root_dir);
    let part_2 = all_dirs.iter()
        .filter(|d| **d >= space_required)
        .min().unwrap();
    println!("Time: {}us", start.elapsed().as_micros());

    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);
}

pub fn parse_dir(line_iter: &mut Iter<&str>, dirs: &mut Vec<usize>) -> usize {
    assert_eq!(line_iter.next().unwrap(), &"$ ls"); //ensure the first command is to list the files
    let mut total_file_size = 0;
    loop {
        let line = line_iter.next();
        if line == None || line.unwrap().is_empty() {
            break; //go back to parent dir
        }
        let mut line_splitter = line.unwrap().split(' ');
        let first_wrd = line_splitter.next().unwrap();
        if let Ok(file_size) = first_wrd.parse::<usize>() {
            total_file_size += file_size; //file detected
        } else if first_wrd.starts_with("dir") {
            continue; //directory detected, ignore
        } else if first_wrd.chars().nth(0).unwrap() == '$' {
            match line_splitter.next().unwrap() {
                "cd" => match line_splitter.next().unwrap() {
                    ".." => break, //go back to parent dir
                    _ => total_file_size += parse_dir(line_iter, dirs), // go into dir
                }
                _ => panic!("Unexpected command"),
            }
        } else { panic!("Unexpected command"); }
    }
    dirs.push(total_file_size);
    total_file_size
}