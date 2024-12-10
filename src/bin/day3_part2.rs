use adventofcode::input;

#[derive(Debug, Eq, PartialEq)]
enum Token {
    Do,
    Dont,
    Mul(i64, i64),
}

fn parse_begin(s: &str) -> Option<Token> {
    if s.starts_with("do()") {
        Some(Token::Do)
    } else if s.starts_with("don't()") {
        Some(Token::Dont)
    } else if s.starts_with("mul(") {
        let start = s.get(4..)?;
        let first_len = start.find(|c: char| !c.is_ascii_digit())?;
        if start.get(first_len..first_len + 1)? != "," {
            return None;
        }
        let first = &start[..first_len];
        let second_start = first_len + 1;
        let second_len = &start[second_start..].find(|c: char| !c.is_ascii_digit())?;
        let second = &start[second_start..second_start + second_len];
        if start.get(second_start + second_len..second_start + second_len + 1)? != ")" {
            return None;
        }

        Some(Token::Mul(first.parse().unwrap(), second.parse().unwrap()))
    } else {
        None
    }
}

fn main() {
    let data = input::read_string();
    let mut result = 0;
    let mut enabled = true;
    for i in 0..data.len() {
        if let Some(token) = parse_begin(&data[i..]) {
            match token {
                Token::Do => enabled = true,
                Token::Dont => enabled = false,
                Token::Mul(a, b) if enabled => result += a * b,
                _ => (),
            }
        }
    }
    println!("{result}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_begin() {
        assert_eq!(parse_begin("mul(bla"), None);
        assert_eq!(parse_begin("mul(7bla"), None);
        assert_eq!(parse_begin("mul(7,bla"), None);
        assert_eq!(parse_begin("mul(7,40bla"), None);
        assert_eq!(parse_begin("mul(7,40)bla"), Some(Token::Mul(7, 40)));
        assert_eq!(parse_begin("mul("), None);
        assert_eq!(parse_begin("mul(7"), None);
        assert_eq!(parse_begin("mul(7,"), None);
        assert_eq!(parse_begin("mul(7,40"), None);
        assert_eq!(parse_begin("mul(7,40)"), Some(Token::Mul(7, 40)));
    }
}
