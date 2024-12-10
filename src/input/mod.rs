use std::io::{self, Read};

pub fn read_grid() -> Vec<Vec<char>> {
    io::stdin()
        .lines()
        .map(|line| line.expect("IO error").chars().collect())
        .collect()
}

pub fn read_string() -> String {
    let mut input = String::new();
    io::stdin()
        .lock()
        .read_to_string(&mut input)
        .expect("Error reading input");

    input
}
