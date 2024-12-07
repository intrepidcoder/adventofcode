use std::io;

fn main() {
    let result: usize = io::stdin()
        .lines()
        .map(|line| -> (usize, Vec<usize>) {
            line.unwrap()
                .split_once(':')
                .map(|(total, values)| {
                    (
                        total.parse().unwrap(),
                        values
                            .split_ascii_whitespace()
                            .flat_map(|s| s.parse())
                            .collect(),
                    )
                })
                .unwrap()
        })
        .filter(|(total, values)| {
            (0..(1 << values.len())).any(|mask| {
                *total
                    == values
                        .iter()
                        .fold((mask * 2, 0), |(cur_mask, acc), val| {
                            if cur_mask & 1 == 0 {
                                (cur_mask >> 1, acc + val)
                            } else {
                                (cur_mask >> 1, acc * val)
                            }
                        })
                        .1
            })
        })
        .map(|(total, _)| total)
        .sum();
    println!("{result}");
}
