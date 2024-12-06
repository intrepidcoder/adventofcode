use std::{collections::HashMap, io};

fn main() {
    let mut input_lines = io::stdin().lines();
    let mut rules: Vec<(usize, usize)> = vec![];
    while let Some(rule) = {
        let line = input_lines.next().unwrap().expect("IO error");
        let mut nums = line.split('|').flat_map(|s| s.parse::<usize>());
        nums.next().zip(nums.next())
    } {
        rules.push(rule);
    }
    let rules = rules;

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
