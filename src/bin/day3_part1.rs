use regex::Regex;
use std::io::{self, Read};

fn main() {
    let mut data = String::new();
    io::stdin()
        .lock()
        .read_to_string(&mut data)
        .expect("Error reading input");
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