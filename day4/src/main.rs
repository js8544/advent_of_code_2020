use std::collections::HashMap;

fn main() {
    let ans = include_str!("input.txt")
        .split("\n\n")
        .map(|item| {
            item.trim()
                .split(&[' ', '\n'][..])
                .fold(HashMap::new(), |mut hash_map, pair| {
                    let mut pair = pair.trim().split(':');
                    let key = pair.next();
                    let value = pair.next();
                    match (key, value) {
                        (Some(k), Some(v)) => {
                            hash_map.insert(k, v);
                            hash_map
                        }
                        _ => hash_map,
                    }
                })
        })
        .filter(|hash_map| {
            println!("passport: {:?}", hash_map);
            let valid = (if let Some(&str) = hash_map.get("byr") {
                str.len() == 4
                    && if let Ok(num) = str.parse::<u32>() {
                        num >= 1920 && num <= 2002
                    } else {
                        false
                    }
            } else {
                false
            }) && (if let Some(&str) = hash_map.get("iyr") {
                str.len() == 4
                    && if let Ok(num) = str.parse::<u32>() {
                        num >= 2010 && num <= 2020
                    } else {
                        false
                    }
            } else {
                false
            }) && (if let Some(&str) = hash_map.get("eyr") {
                str.len() == 4
                    && if let Ok(num) = str.parse::<u32>() {
                        num >= 2020 && num <= 2030
                    } else {
                        false
                    }
            } else {
                false
            }) && (if let Some(&str) = hash_map.get("hgt") {
                if str.ends_with("cm") {
                    str.len() == 5
                        && if let Ok(num) = str[..3].parse::<u32>() {
                            num >= 150 && num <= 193
                        } else {
                            false
                        }
                } else if str.ends_with("in") {
                    str.len() == 4
                        && if let Ok(num) = str[..2].parse::<u32>() {
                            num >= 59 && num <= 76
                        } else {
                            false
                        }
                } else {
                    false
                }
            } else {
                false
            }) && (if let Some(&str) = hash_map.get("hcl") {
                str.len() == 7
                    && str.bytes().next().unwrap() == b'#'
                    && str[1..].bytes().all(|char| {
                        (char >= b'0' && char <= b'9') || (char >= b'a' && char <= b'f')
                    })
            } else {
                false
            }) && (if let Some(&str) = hash_map.get("ecl") {
                match str {
                    "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
                    _ => false,
                }
            } else {
                false
            }) && (if let Some(&str) = hash_map.get("pid") {
                str.len() == 9 && str.bytes().all(|char| char.is_ascii_digit())
            } else {
                false
            });
            println!("{}", valid);
            valid
        })
        .count();
    println!("ans: {}", ans);
}
