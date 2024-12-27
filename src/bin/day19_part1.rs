use std::io::{self, BufRead};

fn main() {
    println!("{}", solve(&mut io::stdin().lock()));
}

fn solve(input: impl BufRead) -> usize {
    let mut lines = input.lines().map(|line| line.unwrap());

    let pattern_line = lines.next().unwrap();
    let patterns: Vec<&str> = pattern_line.split(',').map(|p| p.trim()).collect();
    lines.next();

    lines
        .filter(|design| is_possible(design, patterns.as_slice()))
        .count()
}

fn is_possible(design: &str, patterns: &[&str]) -> bool {
    let mut possible = vec![false; design.len() + 1];
    possible[0] = true;

    for i in 0..design.len() {
        if possible[i] {
            for &pat in patterns.iter() {
                if design[i..].starts_with(pat) {
                    possible[i + pat.len()] = true;
                }
            }
        }
    }
    possible[design.len()]
}

#[cfg(test)]
mod test {
    use super::solve;
    use std::{
        fs::File,
        io::{self, BufReader},
    };

    fn open(path: &str) -> io::Result<BufReader<File>> {
        let file = File::open(path)?;
        Ok(BufReader::new(file))
    }

    #[test]
    fn test_example() {
        let input = open("input/day19/example.txt").expect("Missing file");
        assert_eq!(6, solve(input));
    }

    #[test]
    fn test_input() {
        let input = open("input/day19/input.txt").expect("Missing file");
        assert_eq!(236, solve(input));
    }
}
