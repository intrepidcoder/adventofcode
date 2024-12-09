use itertools::Itertools;
use std::io;

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
                .map(move |(j, c)| (c, (i as isize, j as isize)))
                .collect()
        })
        .into_group_map();
    let height: isize = height as isize + 1;
    let width: isize = width as isize + 1;

    let result = antennas
        .into_values()
        .flat_map(|nodes| {
            nodes.into_iter().combinations(2).flat_map(move |pairs| {
                let (x1, y1) = pairs[0];
                let (x2, y2) = pairs[1];
                let dx = x2 - x1;
                let dy = y2 - y1;
                let count = width.max(height);
                (-count..=count).map(move |n| (x1 + n * dx, y1 + n * dy))
            })
        })
        .filter(|(x, y)| (0..height).contains(x) && (0..width).contains(y))
        .unique()
        .count();

    println!("{result}");
}
