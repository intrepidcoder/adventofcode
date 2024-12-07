use std::io;

fn main() {
    const BASE: usize = 3;
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
            (0..(BASE.pow(values.len() as u32))).any(|mask| {
                *total
                    == values
                        .iter()
                        .fold((mask * BASE, 0), |(cur_mask, acc), val| {
                            match cur_mask % BASE {
                                0 => (cur_mask / BASE, acc + val),
                                1 => (cur_mask / BASE, acc * val),
                                2 => (
                                    cur_mask / BASE,
                                    acc * (10usize.pow(val.checked_ilog10().unwrap_or(1) + 1))
                                        + val,
                                ),
                                _ => unreachable!(),
                            }
                        })
                        .1
            })
        })
        .map(|(total, _)| total)
        .sum();
    println!("{result}");
}
