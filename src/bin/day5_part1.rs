use itertools::Itertools;
use std::{collections::HashMap, io};

fn main() {
    let mut input_lines = io::stdin().lines();
    let rules: Vec<(usize, usize)> = input_lines
        .by_ref()
        .map(|line| line.expect("IO error"))
        .take_while(|line| !line.is_empty())
        .flat_map(|line| {
            line.split('|')
                .flat_map(|s| s.parse::<usize>())
                .collect_tuple()
        })
        .collect();

    let result: usize = input_lines
        .map(|line: Result<_, _>| {
            line.expect("IO error")
                .split(',')
                .flat_map(|s| s.parse::<usize>())
                .enumerate()
                .map(|(a, b)| (b, a))
                .collect()
        })
        .filter(|update: &HashMap<_, _>| {
            rules
                .iter()
                .all(|(a, b)| update.get(a).zip(update.get(b)).is_none_or(|(i, j)| i < j))
        })
        .map(|update| {
            let middle = update.len() / 2;
            update.into_iter().find(|(_, v)| *v == middle).unwrap().0
        })
        .sum();

    println!("{result}");
}
