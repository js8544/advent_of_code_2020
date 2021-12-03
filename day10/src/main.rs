use std::collections::HashMap;

fn main() {
    let mut arr: Vec<u64> = include_str!("input.txt")
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect();
    arr.sort_unstable();
    println!("sorted: {:?}", arr);
    let last = arr.last().unwrap();

    let mut init: HashMap<u64, u64> = HashMap::new();
    init.insert(0, 1);
    let cnt_map = arr.iter().fold(init, |mut acc_map, &val| {
        let cnt = (1u64..4.min(val + 1)).fold(0, |acc, diff| match acc_map.get(&(val - diff)) {
            Some(v) => acc + v,
            None => acc,
        });
        println!("val: {}, cnt: {}", val, cnt);
        acc_map.insert(val, cnt);
        acc_map
    });

    println!("ans: {}", cnt_map.get(last).unwrap());
}
