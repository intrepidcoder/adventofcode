use adventofcode::input;
use std::iter;

fn main() {
    let map = input::read_grid();
    let start_pos = map
        .iter()
        .enumerate()
        .find_map(|(i, row)| {
            row.iter()
                .enumerate()
                .find(|(_, &c)| c == '^')
                .map(|(j, _)| (i, j))
        })
        .unwrap();

    let result: usize = (0..map.len())
        .map(|i| {
            (0..map[i].len())
                .filter(|&j| map[i][j] == '.' && stuck((i, j), &map, start_pos))
                .count()
        })
        .sum();
    println!("{result}");
}

fn stuck(block_pos: (usize, usize), map: &[Vec<char>], start_pos: (usize, usize)) -> bool {
    let dirs_x = [-1, 0, 1, 0];
    let dirs_y = [0, 1, 0, -1];

    let mut dir = 0;
    let (mut pos_x, mut pos_y) = start_pos;

    let mut visited: Vec<Vec<u8>> =
        iter::repeat_n(iter::repeat_n(0, map[0].len()).collect(), map.len()).collect();

    while !in_mask(visited[pos_x][pos_y], dir) {
        visited[pos_x][pos_y] |= 1 << dir;

        let next_x = (pos_x as isize + dirs_x[dir]) as usize;
        let next_y = (pos_y as isize + dirs_y[dir]) as usize;

        match map.get(next_x).and_then(|row| row.get(next_y)) {
            Some(next_loc) if (next_x, next_y) == block_pos || *next_loc == '#' => {
                // Turn
                dir = (dir + 1) & 3;
            }
            Some(_) => {
                // Move
                pos_x = next_x;
                pos_y = next_y;
            }
            None => return false,
        }
    }

    true
}

fn in_mask(mask: u8, dir: usize) -> bool {
    mask & (1 << dir) > 0
}
