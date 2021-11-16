fn main() {
    let ans = include_str!("input.txt")
        .lines()
        .filter(|&line| check_valid(line))
        .count();
    println!("ans: {}\n", ans);
}

#[derive(PartialEq, Debug)]
struct PasswordPolicy {
    pos: [usize; 2],
    byte: u8,
}

#[derive(thiserror::Error, Debug)]
enum ParseError {
    #[error("expected {0}")]
    Expected(&'static str),
}

fn parse_line(line: &str) -> anyhow::Result<(PasswordPolicy, &str)> {
    let (policy, password) = {
        let mut tokens = line.split(':');
        (
            tokens
                .next()
                .ok_or(ParseError::Expected("password policy"))?,
            tokens
                .next()
                .ok_or(ParseError::Expected("password"))?
                .trim(),
        )
    };

    let (pos, byte) = {
        let mut tokens = policy.split(' ');
        (
            tokens.next().ok_or(ParseError::Expected("policy range"))?,
            tokens.next().ok_or(ParseError::Expected("policy byte"))?,
        )
    };

    let byte = if byte.as_bytes().len() == 1 {
        byte.as_bytes()[0]
    } else {
        return Err(ParseError::Expected("password policy byte to be exactly 1 byte").into());
    };

    let (first, second) = {
        let mut tokens = pos.split('-');
        (
            tokens.next().ok_or(ParseError::Expected("policy pos 1"))?,
            tokens.next().ok_or(ParseError::Expected("policy pos 2"))?,
        )
    };

    Ok((
        PasswordPolicy {
            pos: [first.parse()?, second.parse()?],
            byte,
        },
        password,
    ))
}

fn check_password_valid(policy: &PasswordPolicy, password: &str) -> bool {
    policy
        .pos
        .iter()
        .filter(|&pos| match password.as_bytes().get(*pos - 1) {
            Some(b) => *b == policy.byte,
            None => false,
        })
        .count()
        == 1
}

fn check_valid(line: &str) -> bool {
    match parse_line(line) {
        Ok((policy, password)) => check_password_valid(&policy, password),
        Err(_) => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        let line = "14-19 x: lrxxxtxxxxxxxzxxxxx";
        let (pp, password) = parse_line(line).unwrap();
        assert_eq!(
            pp,
            PasswordPolicy {
                pos: [14, 19],
                byte: b'x',
            }
        );
        assert_eq!(password, "lrxxxtxxxxxxxzxxxxx");
    }

    #[test]
    fn test_check_password_valid() {
        let pp = PasswordPolicy {
            pos: [1, 3],
            byte: b'a',
        };
        assert_eq!(check_password_valid(&pp, "zeus"), false, "no 'a's");
        assert_eq!(check_password_valid(&pp, "aades"), true, "single 'a' at 1");
        assert_eq!(check_password_valid(&pp, "baaana"), true, "single 'a' at 3");
        assert_eq!(check_password_valid(&pp, "aaa"), false, "'a' on both");
    }
}
