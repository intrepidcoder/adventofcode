use itertools::Itertools;
use std::{cmp::Ordering, collections::HashMap, io};

fn main() {
    let mut input_lines = io::stdin().lines();
    let rules: HashMap<(usize, usize), bool> = input_lines
        .by_ref()
        .map(|line| line.expect("IO error"))
        .take_while(|line| !line.is_empty())
        .flat_map(|line| {
            line.split('|')
                .flat_map(|s| s.parse::<usize>())
                .collect_tuple()
        })
        .flat_map(|(a, b)| [((a, b), true), ((b, a), false)])
        .collect();

    let result: usize = input_lines
        .map(|line: Result<_, _>| {
            line.expect("IO error")
                .split(',')
                .flat_map(|s| s.parse::<usize>())
                .collect()
        })
        .filter(|update: &Vec<_>| {
            !update
                .iter()
                .is_sorted_by(|a, b| *rules.get(&(**a, **b)).unwrap())
        })
        .map(|mut update| {
            update.sort_by(|a, b| match rules.get(&(*a, *b)) {
                Some(true) => Ordering::Less,
                Some(false) => Ordering::Greater,
                _ => unreachable!(),
            });
            update[update.len() / 2]
        })
        .sum();

    println!("{result}");
}
