use adventofcode::input;
use std::iter;

fn main() {
    let map = input::read_grid();
    let mut visited: Vec<Vec<bool>> =
        iter::repeat_n(iter::repeat_n(false, map[0].len()).collect(), map.len()).collect();

    let dirs = [0, 1, 0, -1];
    let (mut dir_x, mut dir_y) = (3, 0);

    let (mut pos_x, mut pos_y) = map
        .iter()
        .enumerate()
        .find_map(|(i, row)| {
            row.iter()
                .enumerate()
                .find(|(_, c)| **c == '^')
                .map(|(j, _)| (i, j))
        })
        .unwrap();

    loop {
        visited[pos_x][pos_y] = true;

        let next_x = (pos_x as isize + dirs[dir_x]) as usize;
        let next_y = (pos_y as isize + dirs[dir_y]) as usize;

        match map.get(next_x).and_then(|row| row.get(next_y)) {
            Some(next_loc) if *next_loc == '#' => {
                // Turn
                dir_x = (dir_x + 1) & 3;
                dir_y = (dir_y + 1) & 3;
            }
            Some(_) => {
                // Move
                pos_x = next_x;
                pos_y = next_y;
            }
            None => break,
        }
    }
    let result: usize = visited
        .iter()
        .map(|row| row.iter().map(|v| *v as usize).sum::<usize>())
        .sum();
    println!("{result}");
}
