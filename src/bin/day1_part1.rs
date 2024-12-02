use std::io;

fn main() {
    let (mut first, mut second): (Vec<_>, Vec<_>) = io::stdin()
        .lines()
        .map(|line| {
            let line = line.expect("IO Error");
            let mut nums = line.split(' ').flat_map(|s| s.parse::<i64>());
            (nums.next().unwrap(), nums.next().unwrap())
        })
        .unzip();
    first.sort();
    second.sort();
    let result: i64 = first
        .iter()
        .zip(second.iter())
        .map(|(x, y)| i64::abs(x - y))
        .sum();
    println!("{result}");
}
