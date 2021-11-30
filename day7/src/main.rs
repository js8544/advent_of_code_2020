use regex::Regex;
use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    let mut neighbors: HashMap<&str, Vec<(&str, usize)>> = HashMap::new();
    let mut visited: HashMap<&str, usize> = HashMap::new();
    include_str!("input.txt")
        .lines()
        .map(parse_line)
        .for_each(|(outer, inners)| {
            inners
                .iter()
                .for_each(|&(inner, cnt)| match neighbors.get_mut(outer) {
                    Some(res) => res.push((inner, cnt)),
                    None => {
                        neighbors.insert(outer, vec![(inner, cnt)]);
                    }
                })
        });
    println!("ans: {}", dfs(&neighbors, "shiny gold", &mut visited) - 1);
}

fn parse_line(line: &str) -> (&str, Vec<(&str, usize)>) {
    let tokens = line.split(" contain ").collect::<Vec<&str>>();
    assert!(tokens.len() >= 2);
    // println!("{:?}", tokens);
    let outer_bag = Regex::new(r"([a-z]+ [a-z]+) bags")
        .unwrap()
        .captures(tokens[0])
        .unwrap()
        .get(1)
        .unwrap()
        .as_str();
    if let Some(_) = tokens[1].find("no other bags") {
        (outer_bag, Vec::new())
    } else {
        let inner_re = Regex::new(r"([0-9]) ([a-z]+ [a-z]+) bags?.?").unwrap();
        let inner_bags = tokens[1]
            .split(", ")
            .map(|token| {
                let caps = inner_re.captures(token).unwrap();
                (
                    caps.get(2).unwrap().as_str(),
                    caps.get(1).unwrap().as_str().parse::<usize>().unwrap(),
                )
            })
            .collect::<Vec<(&str, usize)>>();
        // println!("outer: {}, inner: {:?}", outer_bag, inner_bags);
        (outer_bag, inner_bags)
    }
}

fn dfs<'a>(
    neighbors: &HashMap<&'a str, Vec<(&'a str, usize)>>,
    source: &'a str,
    visited: &mut HashMap<&'a str, usize>,
) -> usize {
    println!("Source {}", source);
    if let Some(val) = visited.get(source) {
        println!("{} Found ans: {}", source, val);
        val.to_owned()
    } else {
        if let Some(cur_neighbors) = neighbors.get(source) {
            println!("{:?}", cur_neighbors);
            let val = cur_neighbors.iter().fold(1, |cur, (neighbor, cnt)| {
                cur + cnt * dfs(neighbors, neighbor, visited)
            });
            visited.insert(source, val);
            println!("{} New ans: {}", source, val);
            val
        } else {
            println!("{} No children: 1", source);
            visited.insert(source, 1);
            1
        }
    }
}
fn _bfs(neighbors: &HashMap<&str, Vec<(&str, usize)>>, source: &str) -> usize {
    let mut queue: VecDeque<&str> = VecDeque::new();
    let mut visited: HashSet<&str> = HashSet::new();
    let mut cnt: usize = 0;
    queue.push_back(source);
    visited.insert(source);
    let default = Vec::<(&str, usize)>::new();
    loop {
        if queue.is_empty() {
            return cnt;
        }
        let cur = queue.pop_front().unwrap();
        neighbors
            .get(cur)
            .unwrap_or(&default)
            .iter()
            .for_each(|&(inner, inner_cnt)| {
                println!("outer {}: inner {}, cnt {}", cur, inner, inner_cnt);
                if let Some(_) = visited.get(inner) {
                    ()
                } else {
                    visited.insert(inner);
                    queue.push_back(inner);
                    cnt = cnt + inner_cnt;
                }
            });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        let (outer_bag, inner_bags) =
            parse_line("light red bags contain 1 bright white bag, 2 muted yellow bags.");
        assert_eq!(outer_bag, "light red");
        assert_eq!(inner_bags, [("bright white", 1), ("muted yellow", 2)]);

        let (outer_bag, inner_bags) = parse_line("muted tomato bags contain 5 dim lavender bags.");
        assert_eq!(outer_bag, "muted tomato");
        assert_eq!(inner_bags, [("dim lavender", 5)]);

        let (outer_bag, inner_bags) =
            parse_line("dim magenta bags contain 4 striped orange bags, 2 mirrored turquoise bags, 3 vibrant turquoise bags, 3 pale chartreuse bags.");
        assert_eq!(outer_bag, "dim magenta");
        assert_eq!(
            inner_bags,
            [
                ("striped orange", 4),
                ("mirrored turquoise", 2),
                ("vibrant turquoise", 3),
                ("pale chartreuse", 3)
            ]
        );
    }
}
