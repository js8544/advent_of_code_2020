fn main() {
    let ans = include_str!("input.txt")
        .lines()
        .map(compute_line)
        .max()
        .unwrap();
    print!("ans: {}\n", ans);
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
