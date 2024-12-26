use adventofcode::grid::Grid;
use itertools::Itertools;
use std::{
    collections::VecDeque,
    io::{self, BufRead},
};

fn main() {
    println!("{}", solve(&mut io::stdin().lock(), 71, 1024));
}

fn solve(input: impl BufRead, size: usize, bytes: usize) -> usize {
    let mut grid = Grid::new(size, size, '.');
    input
        .lines()
        .take(bytes)
        .map(|line| -> (usize, usize) {
            line.expect("IO error")
                .split(',')
                .flat_map(|s| s.parse())
                .collect_tuple()
                .unwrap()
        })
        .for_each(|(x, y)| {
            let pos = grid.get_coords(x, y).unwrap();
            grid.set(pos, '#');
        });

    let mut queue = VecDeque::new();
    let mut dist = vec![usize::MAX; size * size];
    dist[0] = 0;
    queue.push_back(grid.pos(0));
    while let Some(pos) = queue.pop_front() {
        queue.extend(
            pos.neighbors(&grid)
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

    dist[size * size - 1]
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
        assert_eq!(22, solve(input, 7, 12));
    }

    #[test]
    fn test_input() {
        let input = open("input/day18/input.txt").expect("Missing file");
        assert_eq!(506, solve(input, 71, 1024));
    }
}
