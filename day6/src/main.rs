use itertools::Itertools;

fn main() {
    let ans: usize = include_str!("input.txt")
        .split("\n\n")
        .map(|line| line.bytes().filter(u8::is_ascii_lowercase).unique().count())
        .sum();
    print!("ans: {}\n", ans);
}
