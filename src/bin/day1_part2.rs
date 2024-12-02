use std::collections::HashMap;
use std::io;

fn main() {
    let (left, counts) = io::stdin()
        .lines()
        .map(|line| {
            let line = line.expect("IO Error");
            let mut nums = line.split(' ').flat_map(|s| s.parse::<u64>());
            (nums.next().unwrap(), nums.next().unwrap())
        })
        .fold(
            (Vec::new(), HashMap::new()),
            |(mut vec, mut map), (a, b)| {
                vec.push(a);
                *map.entry(b).or_default() += 1;
                (vec, map)
            },
        );
    let result: u64 = left
        .into_iter()
        .map(|x| x * counts.get(&x).unwrap_or(&0))
        .sum();
    println!("{result}");
}
