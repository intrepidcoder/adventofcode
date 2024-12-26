use adventofcode::grid::{DirectedPos, Direction, Grid};
use std::io::{self, BufRead};

fn main() {
    println!("{}", solve(io::stdin().lock()));
}

fn solve(mut input: impl BufRead) -> usize {
    let mut grid_string = String::new();
    while let Ok(len) = input.read_line(&mut grid_string) {
        if len <= 1 {
            break;
        }
    }

    let widened_string = grid_string
        .replace('#', "##")
        .replace('O', "[]")
        .replace('.', "..")
        .replace('@', "@.");
    let mut grid: Grid = widened_string.parse().unwrap();
    let mut moves = String::new();
    while let Ok(len) = input.read_line(&mut moves) {
        if len == 0 {
            break;
        }
    }
    let moves = moves.trim();

    let start = grid.find_char('@').unwrap();
    grid.set(start, '.');

    let mut pos = start;
    for dir in moves.chars() {
        let direction = match dir {
            '^' => Direction::North,
            '>' => Direction::East,
            'v' => Direction::South,
            '<' => Direction::West,
            _ => continue,
        };

        let dir_pos = pos.directed(direction);
        let next_pos = dir_pos.advance(&grid).unwrap();
        match next_pos.value() {
            '.' => {
                pos = next_pos.pos();
            }
            '[' | ']' => {
                if direction == Direction::North || direction == Direction::South {
                    let first_bracket = if direction == Direction::North {
                        '['
                    } else {
                        ']'
                    };

                    if next_pos.value() == first_bracket {
                        if can_push(next_pos, &grid) {
                            do_push(next_pos, &mut grid);
                            pos = next_pos.pos();
                        }
                    } else {
                        let box_pos = dir_pos.ccw45().advance(&grid).unwrap().cw45();
                        if can_push(box_pos, &grid) {
                            do_push(box_pos, &mut grid);
                            pos = next_pos.pos();
                        }
                    }
                } else if can_push(next_pos, &grid) {
                    do_push(next_pos, &mut grid);
                    pos = next_pos.pos();
                }
            }
            _ => (),
        }
    }

    grid.iter()
        .enumerate()
        .filter(|&(_, c)| *c == '[')
        .map(|(i, _)| grid.coords(i))
        .map(|(r, c)| 100 * r + c)
        .sum()
}

fn can_push(pos: DirectedPos, grid: &Grid) -> bool {
    let first_bracket;
    let second_bracket;
    if pos.direction() == Direction::North || pos.direction() == Direction::East {
        first_bracket = '[';
        second_bracket = ']';
    } else {
        first_bracket = ']';
        second_bracket = '[';
    }
    assert_eq!(pos.value(), first_bracket);
    let next = pos.advance(grid).unwrap();

    if pos.direction() == Direction::North || pos.direction() == Direction::South {
        let side = pos.cw45().advance(grid).unwrap().ccw45();
        match (next.value(), side.value()) {
            ('#', _) => false,
            (_, '#') => false,
            ('.', '.') => true,
            _ => {
                let mut result = true;
                if side.value() == first_bracket {
                    result = result && can_push(side, grid);
                }
                if next.value() == first_bracket {
                    result = result && can_push(next, grid);
                } else if next.value() == second_bracket {
                    result = result && can_push(pos.ccw45().advance(grid).unwrap().cw45(), grid);
                }
                result
            }
        }
    } else {
        let next = pos.advance(grid).unwrap();
        assert_eq!(next.value(), second_bracket);
        let next = next.advance(grid).unwrap();
        match next.value() {
            '#' => false,
            '.' => true,
            b if b == first_bracket => can_push(next, grid),
            _ => false,
        }
    }
}

fn do_push(pos: DirectedPos, grid: &mut Grid) {
    let first_bracket;
    let second_bracket;
    if pos.direction() == Direction::North || pos.direction() == Direction::East {
        first_bracket = '[';
        second_bracket = ']';
    } else {
        first_bracket = ']';
        second_bracket = '[';
    }
    assert_eq!(pos.value(), first_bracket);
    let next = pos.advance(grid).unwrap();

    if pos.direction() == Direction::North || pos.direction() == Direction::South {
        let side = pos.cw45().advance(grid).unwrap().ccw45();
        if side.value() == first_bracket {
            do_push(side, grid);
        }
        if next.value() == first_bracket {
            do_push(next, grid);
        } else if next.value() == second_bracket {
            do_push(pos.ccw45().advance(grid).unwrap().cw45(), grid);
        }
        grid.set(next.pos(), first_bracket);
        grid.set(side.pos(), second_bracket);
        grid.set(pos.pos(), '.');
        grid.set(pos.cw().advance(grid).unwrap().pos(), '.');
    } else {
        let next = pos.advance(grid).unwrap();
        assert_eq!(next.value(), second_bracket);
        let next_next = next.advance(grid).unwrap();
        if next_next.value() == first_bracket {
            do_push(next_next, grid);
        }
        grid.set(next_next.pos(), second_bracket);
        grid.set(next.pos(), first_bracket);
        grid.set(pos.pos(), '.');
    }
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
    fn test_large_example() {
        let input = open("input/day15/example_large.txt").expect("Missing file");
        assert_eq!(9021, solve(input));
    }

    #[test]
    fn test_input() {
        let input = open("input/day15/input.txt").expect("Missing file");
        assert_eq!(1468005, solve(input));
    }
}
