use adventofcode::input;
use regex::Regex;

fn main() {
    let data = input::read_string();
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let result: i64 = re
        .captures_iter(&data)
        .map(|caps| {
            let (_, [a, b]) = caps.extract();

            a.parse::<i64>().unwrap() * b.parse::<i64>().unwrap()
        })
        .sum();
    println!("{result}");
}
