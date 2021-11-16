fn main() {
    let map = include_str!("input.txt");
    let ans: u64 = [
        (1, 1).into(),
        (3, 1).into(),
        (5, 1).into(),
        (7, 1).into(),
        (1, 2).into(),
    ]
    .iter()
    .map(|policy| compute(policy, map))
    .product();
    println!("ans: {}", ans);
}

struct Policy {
    right: usize,
    down: usize,
}

impl From<(usize, usize)> for Policy {
    fn from((right, down): (usize, usize)) -> Self {
        Policy { right, down }
    }
}

fn compute(policy: &Policy, map: &str) -> u64 {
    map.lines()
        .step_by(policy.down)
        .fold((0u64, 0), |(ans, pos), line| {
            (
                ans + ((line.as_bytes()[pos % line.len()] == b'#') as u64),
                (pos + policy.right) % line.len(),
            )
        })
        .0
}
