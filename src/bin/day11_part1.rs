use adventofcode::input;

fn main() {
    let mut stones: Vec<usize> = input::read_string()
        .trim()
        .split(' ')
        .flat_map(|s| s.parse())
        .collect();

    for _ in 0..25 {
        stones = stones
            .into_iter()
            .flat_map(|n| {
                if n == 0 {
                    return vec![1];
                }
                let digits = n.ilog10() + 1;
                if digits % 2 == 0 {
                    let split = 10_usize.pow(digits / 2);
                    vec![n / split, n % split]
                } else {
                    vec![n.checked_mul(2024).expect("overflow")]
                }
            })
            .collect();
    }
    println!("{}", stones.len());
}
