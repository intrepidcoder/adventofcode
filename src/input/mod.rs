use std::io;

pub fn read_grid() -> Vec<Vec<char>> {
    io::stdin()
        .lines()
        .map(|line| line.expect("IO error").chars().collect())
        .collect()
}
