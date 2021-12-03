fn main() {
    let mut arr: Vec<u64> = vec![0u64];
    arr.extend(
        include_str!("input.txt")
            .lines()
            .map(|line| line.parse::<u64>().unwrap()),
    );
    arr.sort_unstable();
    println!("sorted: {:?}", arr);
    let (one_cnt, three_cnt) = arr.windows(2).fold((0, 1), |(one_cnt, three_cnt), window| {
        let diff = window[1] - window[0];
        (one_cnt + (diff == 1) as u64, three_cnt + (diff == 3) as u64)
    });
    println!(
        "one: {}, three: {}, ans: {}",
        one_cnt,
        three_cnt,
        one_cnt * three_cnt
    );
}
