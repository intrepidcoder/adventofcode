use adventofcode::input;
use itertools::Itertools;
use regex::Regex;

fn main() {
    let data = input::read_string();
    let re = Regex::new(r"[XY][+=](\d+)").unwrap();
    let result: usize = re
        .captures_iter(&data)
        .map(|caps| {
            let (_, [a]) = caps.extract();
            a.parse::<usize>().unwrap()
        })
        .tuples()
        .flat_map(|(ax, ay, bx, by, px, py)| {
            (0..=100)
                .flat_map(|a| {
                    (0..=100)
                        .map(move |b| (a, b))
                        .filter(|&(a, b)| a * ax + b * bx == px && a * ay + b * by == py)
                        .map(|(a, b)| 3 * a + b)
                })
                .min()
        })
        .sum();
    println!("{result}");
}
