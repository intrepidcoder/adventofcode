use adventofcode::grid::{Grid, Pos};
use itertools::Itertools;
use std::{
    collections::VecDeque,
    io::{self, BufRead},
};

fn main() {
    println!("{}", solve(io::stdin().lock(), 100));
}

fn solve(mut input: impl BufRead, min_saved: usize) -> usize {
    let grid = Grid::read_from_buf(&mut input);

    path(&grid)
        .into_iter()
        .combinations(2)
        .map(|vec| vec.into_iter().collect_tuple().unwrap())
        .filter_map(|((pos1, dist1), (pos2, dist2))| {
            let cheat_len = pos1.row().abs_diff(pos2.row()) + pos1.col().abs_diff(pos2.col());
            let orig_len = dist1.abs_diff(dist2);
            if cheat_len <= 20 && cheat_len < orig_len {
                Some(orig_len - cheat_len)
            } else {
                None
            }
        })
        .filter(|&d| d >= min_saved)
        .count()
}

fn path(grid: &Grid) -> Vec<(Pos, usize)> {
    let mut queue = VecDeque::new();
    let mut dist = vec![usize::MAX - 2; grid.len()];
    let start_pos = grid.find_char('S').unwrap();
    let end_pos = grid.find_char('E').unwrap();
    dist[start_pos.index()] = 0;
    queue.push_back(start_pos);
    while let Some(pos) = queue.pop_front() {
        if pos == end_pos {
            break;
        }
        queue.extend(
            pos.neighbors(grid)
                .filter(|&next| next.value() != '#')
                .filter(|&next| {
                    let d = dist[next.index()];
                    if d == usize::MAX - 2 {
                        dist[next.index()] = dist[pos.index()] + 1;
                        true
                    } else {
                        false
                    }
                }),
        );
    }

    let mut pos = end_pos;
    let mut path = Vec::new();
    path.push((pos, dist[pos.index()]));
    while let Some(next) = pos
        .neighbors(grid)
        .find(|&next| dist[next.index()] + 1 == dist[pos.index()])
    {
        pos = next;
        path.push((pos, dist[pos.index()]));
    }

    path
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
        let input = open("input/day20/example.txt").expect("Missing file");
        assert_eq!(3 + 4 + 22 + 12, solve(input, 70));
    }

    #[test]
    fn test_input() {
        let input = open("input/day20/input.txt").expect("Missing file");
        assert_eq!(1016066, solve(input, 100));
    }
}
