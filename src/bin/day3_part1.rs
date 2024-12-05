use regex::Regex;
use std::io;

fn main() {
    let result: i64 = io::stdin()
        .lines()
        .map(|line| {
            let line = line.expect("IO Error");
            let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

            re.captures_iter(&line)
                .map(|caps| {
                    let (_, [a, b]) = caps.extract();

                    a.parse::<i64>().unwrap() * b.parse::<i64>().unwrap()
                })
                .sum::<i64>()
        })
        .sum();
    println!("{result}");
}
