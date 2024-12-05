use std::io;

fn main() {
    let grid: Vec<Vec<char>> = io::stdin()
        .lines()
        .map(|line| line.expect("IO error").chars().collect())
        .collect();

    let mut result: usize = 0;

    for x in 0..grid.len() {
        for y in 0..grid[0].len() {
            let forward_diag = (0..=2)
                .flat_map(|i| grid.get(x + i).and_then(|row| row.get(y + i)))
                .copied();
            let reverse_diag = (0..=2)
                .flat_map(|i| grid.get(x + i).and_then(|row| row.get(y + 2 - i)))
                .copied();
            result += ((forward_diag.clone().eq("MAS".chars()) || forward_diag.eq("SAM".chars()))
                && (reverse_diag.clone().eq("MAS".chars()) || reverse_diag.eq("SAM".chars())))
                as usize;
        }
    }
    println!("{result}");
}
