use itertools::Itertools;

fn main() {
    let nums: Vec<u64> = include_str!("input.txt")
        .lines()
        .map(|line| line.trim().parse::<u64>().unwrap())
        .collect();
    const WINDOW_SIZE: usize = 25;
    let res = nums[..].windows(WINDOW_SIZE + 1).find(|&window| {
        let preamble = &window[..WINDOW_SIZE];
        let cur = window[WINDOW_SIZE];

        !(preamble.iter().tuple_combinations().any(|(first, second)| {
            if first + second == cur {
                println!("cur: {}, first: {}, second: {}", cur, first, second);
                true
            } else {
                false
            }
        }))
    });
    match res {
        Some(arr) => println!("ans: {}", arr[WINDOW_SIZE]),
        None => println!("no ans"),
    };
}
