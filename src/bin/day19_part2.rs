use std::io::{self, BufRead};

fn main() {
    println!("{}", solve(&mut io::stdin().lock()));
}

fn solve(input: impl BufRead) -> usize {
    let mut lines = input.lines().map(|line| line.unwrap());

    let pattern_line = lines.next().unwrap();
    let patterns: Vec<&str> = pattern_line.split(',').map(|p| p.trim()).collect();
    lines.next();

    lines.map(|design| ways(&design, patterns.as_slice())).sum()
}

fn ways(design: &str, patterns: &[&str]) -> usize {
    let mut possible = vec![0; design.len() + 1];
    possible[0] = 1;

    for i in 0..design.len() {
        if possible[i] > 0 {
            for &pat in patterns.iter() {
                if design[i..].starts_with(pat) {
                    possible[i + pat.len()] += possible[i];
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
        assert_eq!(16, solve(input));
    }

    #[test]
    fn test_input() {
        let input = open("input/day19/input.txt").expect("Missing file");
        assert_eq!(643685981770598, solve(input));
    }
}
