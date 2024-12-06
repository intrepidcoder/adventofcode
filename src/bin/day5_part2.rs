use std::{cmp::Ordering, collections::HashMap, io};

fn main() {
    let mut input_lines = io::stdin().lines();
    let mut rules: HashMap<(usize, usize), bool> = HashMap::new();
    while let Some((a, b)) = {
        let line = input_lines.next().unwrap().expect("IO error");
        let mut nums = line.split('|').flat_map(|s| s.parse::<usize>());
        nums.next().zip(nums.next())
    } {
        rules.insert((a, b), true);
        rules.insert((b, a), false);
    }
    let rules = rules;

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
