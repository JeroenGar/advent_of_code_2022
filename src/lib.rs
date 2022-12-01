use std::str::FromStr;

pub fn parse_to_vec<T : FromStr>(string : &str, sep: &str) -> Result<Vec<T>, ()> {
    Ok(string
        .split(sep)
        .filter_map(|line| line.parse::<T>().ok())
        .collect()
    )
}