use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::str;

fn main() {
    let s: Vec<i64> = include_str!("input.txt")
        .lines()
        .map(str::parse::<i64>)
        .map(Result::unwrap)
        .collect();
    print!("pair: {:?}\n", find_pair_sum_2020(&s));
    print!("triplet: {:?}\n", find_triplet_sum_2020(&s));
}

fn find_pair_sum_2020(s: &Vec<i64>) -> Option<i64> {
    s.iter()
        .fold(
            (HashSet::<i64>::new(), None),
            |(mut hm, res), num| match res {
                Some(_) => (hm, res),
                None => {
                    let target = 2020 - num;
                    if hm.contains(&target) {
                        (hm, Some(num * target))
                    } else {
                        hm.insert(num.clone());
                        (hm, None)
                    }
                }
            },
        )
        .1
}

fn find_triplet_sum_2020(s: &Vec<i64>) -> Option<i64> {
    let all_sums = s
        .iter()
        .tuple_combinations()
        .fold(HashMap::new(), |mut hm, (a, b)| {
            hm.insert(a + b, a * b);
            hm
        });
    match s
        .iter()
        .find(|num| all_sums.get(&(2020 - num.clone())).is_some())
    {
        Some(num) => Some(num * all_sums.get(&(2020 - num.clone())).unwrap()),
        None => None,
    }
}
