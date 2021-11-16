use itertools::Itertools;
use std::collections::BTreeSet;

fn main() {
    let all_set = include_str!("input.txt")
        .lines()
        .map(compute_line)
        .collect::<BTreeSet<u32>>();
    all_set.iter().tuple_windows().for_each(|(this, next)| {
        if next - this != 1 {
            print!("ans: {}", this + 1);
            return;
        }
    })
}

fn compute_line(line: &str) -> u32 {
    parse_str(&line[..7], (b'F', b'B')) * 8 + parse_str(&line[7..], (b'L', b'R'))
}

fn parse_str(str: &str, chars: (u8, u8)) -> u32 {
    let mut num = 0;
    str.as_bytes()
        .iter()
        .rev()
        .enumerate()
        .for_each(|(i, &char)| {
            if char == chars.1 {
                num |= 1 << i;
            }
        });
    num
}
