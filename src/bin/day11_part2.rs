use adventofcode::input;
use std::collections::HashMap;

fn count(n: usize, blinks: u32, cache: &mut HashMap<(usize, u32), usize>) -> usize {
    if blinks == 0 {
        return 1;
    }
    if let Some(c) = cache.get(&(n, blinks)) {
        return *c;
    }
    let result = if n == 0 {
        count(1, blinks - 1, cache)
    } else {
        let digits = n.ilog10() + 1;
        if digits % 2 == 0 {
            let split = 10_usize.pow(digits / 2);
            count(n / split, blinks - 1, cache) + count(n % split, blinks - 1, cache)
        } else {
            count(n.checked_mul(2024).expect("overflow"), blinks - 1, cache)
        }
    };

    cache.insert((n, blinks), result);
    result
}

fn main() {
    let mut cache = HashMap::new();

    const BLINKS: u32 = 75;
    let result: usize = input::read_string()
        .trim()
        .split(' ')
        .flat_map(|s| s.parse())
        .map(|n| count(n, BLINKS, &mut cache))
        .sum();
    println!("{result}");
}
