use itertools::Itertools;
use std::{collections::HashMap, io};

fn main() {
    let mut height = 0;
    let mut width = 0;
    let antennas = io::stdin()
        .lines()
        .enumerate()
        .flat_map(|(i, line)| -> Vec<_> {
            line.expect("IO error")
                .chars()
                .enumerate()
                .inspect(|(j, _)| {
                    height = height.max(i);
                    width = width.max(*j);
                })
                .filter(|(_, c)| *c != '.')
                .map(move |(j, c)| (i as isize, j as isize, c))
                .collect()
        })
        .fold(
            HashMap::new(),
            |mut antennas: HashMap<_, Vec<_>>, (i, j, c)| {
                antennas.entry(c).or_default().push((i, j));

                antennas
            },
        );
    let height: isize = height as isize + 1;
    let width: isize = width as isize + 1;

    let result = antennas
        .into_values()
        .flat_map(|nodes| {
            nodes
                .into_iter()
                .combinations(2)
                .flat_map(move |mut pairs| {
                    let (x1, y1) = pairs[0];
                    let (x2, y2) = pairs[1];
                    let dx = x2 - x1;
                    let dy = y2 - y1;
                    pairs[0] = (x1 - dx, y1 - dy);
                    pairs[1] = (x2 + dx, y2 + dy);

                    pairs
                })
        })
        .filter(|(x, y)| (0..height).contains(x) && (0..width).contains(y))
        .unique()
        .count();

    println!("{result}");
}
