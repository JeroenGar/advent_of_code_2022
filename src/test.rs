use std::ops::Rem;

pub fn main() {
    let result = (-3_i32).rem_euclid(30);
    dbg!(result);
}