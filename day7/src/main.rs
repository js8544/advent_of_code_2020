use regex::Regex;
use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    let mut neighbors: HashMap<&str, Vec<&str>> = HashMap::new();
    include_str!("input.txt")
        .lines()
        .map(parse_line)
        .for_each(|(outer, inners)| {
            inners
                .iter()
                .for_each(|&inner| match neighbors.get_mut(inner) {
                    Some(res) => res.push(outer),
                    None => {
                        neighbors.insert(inner, vec![outer]);
                    }
                })
        });
    println!("ans: {}", bfs(neighbors, "shiny gold"));
}

fn parse_line(line: &str) -> (&str, Vec<&str>) {
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
        let inner_re = Regex::new(r"[0-9] ([a-z]+ [a-z]+) bags?.?").unwrap();
        let inner_bags = tokens[1]
            .split(", ")
            .map(|token| inner_re.captures(token).unwrap().get(1).unwrap().as_str())
            .collect::<Vec<&str>>();

        (outer_bag, inner_bags)
    }
}

fn bfs(neighbors: HashMap<&str, Vec<&str>>, source: &str) -> usize {
    let mut queue: VecDeque<&str> = VecDeque::new();
    let mut visited: HashSet<&str> = HashSet::new();
    queue.push_back(source);
    visited.insert(source);
    let default = Vec::<&str>::new();
    loop {
        if queue.is_empty() {
            return visited.len() - 1;
        }
        let cur = queue.pop_front().unwrap();
        neighbors
            .get(cur)
            .unwrap_or(&default)
            .iter()
            .for_each(|&outer| {
                if let Some(_) = visited.get(outer) {
                    ()
                } else {
                    visited.insert(outer);
                    queue.push_back(outer);
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
        assert_eq!(inner_bags, ["bright white", "muted yellow"]);

        let (outer_bag, inner_bags) = parse_line("muted tomato bags contain 5 dim lavender bags.");
        assert_eq!(outer_bag, "muted tomato");
        assert_eq!(inner_bags, ["dim lavender"]);

        let (outer_bag, inner_bags) =
            parse_line("dim magenta bags contain 4 striped orange bags, 2 mirrored turquoise bags, 3 vibrant turquoise bags, 3 pale chartreuse bags.");
        assert_eq!(outer_bag, "dim magenta");
        assert_eq!(
            inner_bags,
            [
                "striped orange",
                "mirrored turquoise",
                "vibrant turquoise",
                "pale chartreuse"
            ]
        );
    }
}
