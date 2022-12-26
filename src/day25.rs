use itertools::Itertools;

const INPUT: &str = include_str!("../input/2022/day25.txt");

fn main() {
    let lines = INPUT.split('\n').collect::<Vec<&str>>();
    let converted = lines.iter().map(|l| convert_snafu(l)).collect_vec();
    let decimal = converted.iter().sum::<i64>();
    println!("Part 1: {}", convert_to_snafu(decimal));
}

fn convert_snafu(s: &str) -> i64 {
    let n_digits = s.trim().len();

    let mut result = 0;
    for (c, place) in (s.chars().rev()).zip(0..n_digits) {
        match c {
            '-' => result -= 5_i64.pow(place as u32),
            '=' => result -= 2 * 5_i64.pow(place as u32),
            _ => result += (c as i64 - 0x30) * 5_i64.pow(place as u32)
        }
    }
    result
}

fn convert_to_snafu(n: i64) -> String {
    let mut result = String::new();
    let mut n = n;

    while n > 0 {
        let digit = n % 5;

        if digit <= 2 {
            result.push((digit + 0x30) as u8 as char);
            n /= 5;
        } else if digit == 3 {
            result.push('=');
            n = n / 5 + 1;
        } else if digit == 4 {
            result.push('-');
            n = n / 5 + 1;
        }
    }
    result.chars().rev().collect()
}