const DATA: &[u8; 10_000_000] = include_bytes!("../input/2022/day6_extra.txt");

pub fn main() {
    let mut total = 0;
    let mut w = 0;
    let mut n = 1;
    'main: while n <= 94 {
        let mut seen = 0u128;
        for i in (0..n).rev() {
            let mask = 1 << DATA[w + i];
            if seen & mask == mask {
                w += i + 1;
                continue 'main;
            }
            seen |= mask;
        }
        total += w + n;
        n += 1;
    }
}