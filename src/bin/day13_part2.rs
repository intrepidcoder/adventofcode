use adventofcode::input;
use itertools::Itertools;
use regex::Regex;

fn main() {
    let data = input::read_string();
    let re = Regex::new(r"[XY][+=](\d+)").unwrap();
    let result: isize = re
        .captures_iter(&data)
        .map(|caps| {
            let (_, [a]) = caps.extract();
            a.parse::<isize>().unwrap()
        })
        .tuples()
        .map(|(ax, ay, bx, by, px, py)| (ax, ay, bx, by, px + 10000000000000, py + 10000000000000))
        .flat_map(|(ax, ay, bx, by, px, py)| {
            // Use Cramer's Rule. The input is "nice" in that this assertion always holds.
            assert_ne!(ax * by, ay * bx, "System is singular: {ax} {ay} {bx} {by}");
            let det = ax * by - ay * bx;
            let cramer_a = px * by - py * bx;
            let cramer_b = ax * py - ay * px;
            let a = cramer_a / det;
            let b = cramer_b / det;
            if a * ax + b * bx == px && a * ay + b * by == py {
                Some(3 * a + b)
            } else {
                None
            }
        })
        .sum();
    println!("{result}");
}
