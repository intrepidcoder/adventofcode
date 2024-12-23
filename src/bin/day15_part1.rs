use adventofcode::grid::{Grid, Neighbors};
use std::io::{self, BufRead};

fn main() {
    let mut grid = Grid::read_from_input();
    let mut moves = String::new();
    let mut stdin = io::stdin().lock();
    while let Ok(len) = stdin.read_line(&mut moves) {
        if len == 0 {
            break;
        }
    }
    let moves = moves.trim();

    let start = grid.find_char('@').unwrap();

    let mut pos = start;
    for dir in moves.chars() {
        let advance = |index| match dir {
            '^' => grid.neighbors(index).north(),
            '>' => grid.neighbors(index).east(),
            'v' => grid.neighbors(index).south(),
            '<' => grid.neighbors(index).west(),
            _ => None,
        };

        let Some(next_pos) = advance(pos) else {
            continue;
        };
        match grid[next_pos] {
            '.' => {
                grid.set(pos, '.');
                grid.set(next_pos, '@');
                pos = next_pos;
            }
            'O' => {
                let mut box_pos = next_pos;
                while let Some(next_box) = advance(box_pos) {
                    box_pos = next_box;
                    if grid[next_box] != 'O' {
                        break;
                    }
                }
                if grid[box_pos] == '.' {
                    // Push box
                    grid.set(pos, '.');
                    grid.set(next_pos, '@');
                    grid.set(box_pos, 'O');
                    pos = next_pos;
                }
            }
            _ => (),
        }
    }

    let result: usize = grid
        .iter()
        .enumerate()
        .filter(|&(_, c)| *c == 'O')
        .map(|(i, _)| grid.coords(i))
        .map(|(r, c)| 100 * r + c)
        .sum();
    println!("{result}");
}
