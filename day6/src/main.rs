use itertools::Itertools;
use std::str;

fn main() {
    let ans: usize = include_str!("input.txt")
        .split("\n\n")
        .map(solve_group)
        .sum();
    print!("ans: {}\n", ans);
}

fn solve_group(group: &str) -> usize {
    let people = group.trim().lines().map(|str| str.trim());
    println!("{:?}", people);
    let ans = (b'a'..=b'z')
        .filter(|&char| people.clone().all(|person| person.bytes().contains(&char)))
        .map(|x| {
            println!("{}", str::from_utf8(&vec![x][..]).unwrap());
            x
        })
        .count();
    println!("ans: {}", ans);
    ans
}

#[cfg(test)]
mod tests {
    use crate::solve_group;

    #[test]
    fn test_solve_group1() {
        let group = "a\na";
        assert_eq!(solve_group(group), 1);
    }

    #[test]
    fn test_solve_group2() {
        let group = "\nva\naav\nwdandjawv\n";
        assert_eq!(solve_group(group), 2);
    }

    #[test]
    fn test_solve_group3() {
        let group = "\na\nb\nc\n";
        assert_eq!(solve_group(group), 0);
    }
}
