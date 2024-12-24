use adventofcode::grid::{Direction, Grid};
use std::io::{self, BufRead};

fn solve(mut input: impl BufRead) -> usize {
    let mut grid = Grid::read_from_buf(&mut input);
    let mut moves = String::new();
    while let Ok(len) = input.read_line(&mut moves) {
        if len == 0 {
            break;
        }
    }
    let moves = moves.trim();

    let start = grid.find_char('@').unwrap();

    let mut pos = start;
    for dir in moves.chars() {
        let direction = match dir {
            '^' => Direction::North,
            '>' => Direction::East,
            'v' => Direction::South,
            '<' => Direction::West,
            _ => continue,
        };

        let next_pos = pos.directed(direction).advance(&grid).unwrap();
        match next_pos.value() {
            '.' => {
                grid.set(pos, '.');
                grid.set(next_pos.pos(), '@');
                pos = next_pos.pos();
            }
            'O' => {
                let mut box_pos = next_pos;
                while let Some(next_box) = box_pos.advance(&grid) {
                    box_pos = next_box;
                    if next_box.value() != 'O' {
                        break;
                    }
                }
                if box_pos.value() == '.' {
                    // Push box
                    grid.set(pos, '.');
                    grid.set(next_pos.pos(), '@');
                    grid.set(box_pos.pos(), 'O');
                    pos = next_pos.pos();
                }
            }
            _ => (),
        }
    }

    grid.iter()
        .enumerate()
        .filter(|&(_, c)| *c == 'O')
        .map(|(i, _)| grid.coords(i))
        .map(|(r, c)| 100 * r + c)
        .sum()
}

fn main() {
    println!("{}", solve(io::stdin().lock()));
}

#[cfg(test)]
mod test {
    use super::solve;
    use std::{
        fs::File,
        io::{self, BufReader},
    };

    fn open(path: &str) -> io::Result<BufReader<File>> {
        let file = File::open(path)?;
        Ok(BufReader::new(file))
    }

    #[test]
    fn test_small_example() {
        let input = open("input/day15/example_small.txt").expect("Missing file");
        assert_eq!(2028, solve(input));
    }

    #[test]
    fn test_large_example() {
        let input = open("input/day15/example_large.txt").expect("Missing file");
        assert_eq!(10092, solve(input));
    }

    #[test]
    fn test_input() {
        let input = open("input/day15/input.txt").expect("Missing file");
        assert_eq!(1476771, solve(input));
    }
}
