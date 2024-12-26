use adventofcode::grid::Grid;
use itertools::Itertools;
use std::{
    collections::VecDeque,
    io::{self, BufRead},
};

fn main() {
    let (x, y) = solve(io::stdin().lock(), 71, 1024);
    println!("{x},{y}");
}

fn solve(input: impl BufRead, size: usize, bytes: usize) -> (usize, usize) {
    let mut grid = Grid::new(size, size, '.');
    input
        .lines()
        .map(|line| -> (usize, usize) {
            line.expect("IO error")
                .split(',')
                .flat_map(|s| s.parse())
                .collect_tuple()
                .unwrap()
        })
        .enumerate()
        .find(|&(i, (x, y))| {
            let pos = grid.get_coords(x, y).unwrap();
            grid.set(pos, '#');

            i > bytes && !is_reachable(&grid)
        })
        .unwrap()
        .1
}

fn is_reachable(grid: &Grid) -> bool {
    let mut queue = VecDeque::new();
    let mut dist = vec![usize::MAX; grid.len()];
    dist[0] = 0;
    queue.push_back(grid.pos(0));
    while let Some(pos) = queue.pop_front() {
        queue.extend(
            pos.neighbors(grid)
                .filter(|&next| next.value() == '.')
                .filter(|&next| {
                    let d = dist[next.index()];
                    if d == usize::MAX {
                        dist[next.index()] = dist[pos.index()] + 1;
                        true
                    } else {
                        false
                    }
                }),
        );
    }

    dist[grid.len() - 1] < usize::MAX
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
    fn test_example() {
        let input = open("input/day18/example.txt").expect("Missing file");
        assert_eq!((6, 1), solve(input, 7, 12));
    }

    #[test]
    fn test_input() {
        let input = open("input/day18/input.txt").expect("Missing file");
        assert_eq!((62, 6), solve(input, 71, 1024));
    }
}
