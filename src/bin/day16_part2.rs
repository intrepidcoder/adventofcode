use adventofcode::grid::{DirectedPos, Direction, Grid};
use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
    io::{self, BufRead},
};

#[derive(Clone, Copy, Eq, PartialEq)]
struct State {
    cost: usize,
    pos: DirectedPos,
}

impl Ord for State {
    // Reverse order so the heap acts as a min-heap
    fn cmp(&self, other: &Self) -> Ordering {
        if other.cost == self.cost {
            other.pos.index().cmp(&self.pos.index())
        } else {
            other.cost.cmp(&self.cost)
        }
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    println!("{}", solve(io::stdin().lock()));
}

fn solve(mut input: impl BufRead) -> usize {
    let grid = Grid::read_from_buf(&mut input);

    let start_pos = grid.find_char('S').unwrap().directed(Direction::East);
    let mut dist: HashMap<DirectedPos, usize> = HashMap::new();
    let mut heap = BinaryHeap::new();

    dist.insert(start_pos, 0);
    heap.push(State {
        cost: 0,
        pos: start_pos,
    });

    while let Some(State { cost, pos }) = heap.pop() {
        if cost > *dist.get(&pos).unwrap_or(&usize::MAX) {
            continue;
        }

        for (next_pos, next_cost) in [
            (pos.advance(&grid).unwrap(), cost + 1),
            (pos.cw(), cost + 1000),
            (pos.reverse(), cost + 1000),
            (pos.ccw(), cost + 1000),
        ]
        .into_iter()
        {
            if next_pos.value() == '#' {
                continue;
            }
            let next_state = State {
                cost: next_cost,
                pos: next_pos,
            };
            let next_dist = dist.entry(next_pos).or_insert(usize::MAX);
            if next_cost < *next_dist {
                heap.push(next_state);
                *next_dist = next_cost;
            }
        }
    }

    backtrack(dist, &grid)
}

fn backtrack(dist: HashMap<DirectedPos, usize>, grid: &Grid) -> usize {
    let goal_pos = grid.find_char('E').unwrap();
    let mut states: Vec<State> = Vec::new();
    let directions = [
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
    ];

    let goal_cost = *directions
        .iter()
        .flat_map(|d| dist.get(&goal_pos.directed(*d)))
        .min()
        .unwrap();
    directions
        .iter()
        .map(|d| goal_pos.directed(*d))
        .filter(|pos| dist.get(pos).is_some_and(|&c| c == goal_cost))
        .for_each(|pos| {
            states.push(State {
                pos,
                cost: goal_cost,
            });
        });
    let mut indices = HashSet::new();

    while let Some(State { pos, cost }) = states.pop() {
        indices.insert(pos.index());
        for (next_pos, next_cost) in [
            (pos.retreat(grid).unwrap(), cost.saturating_sub(1)),
            (pos.cw(), cost.saturating_sub(1000)),
            (pos.reverse(), cost.saturating_sub(1000)),
            (pos.ccw(), cost.saturating_sub(1000)),
        ]
        .into_iter()
        {
            if dist.get(&next_pos).is_some_and(|&c| c == next_cost) {
                states.push(State {
                    pos: next_pos,
                    cost: next_cost,
                });
            }
        }
    }

    indices.len()
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
    fn test_example_small() {
        let input = open("input/day16/example_small.txt").expect("Missing file");
        assert_eq!(45, solve(input));
    }

    #[test]
    fn test_example_large() {
        let input = open("input/day16/example_large.txt").expect("Missing file");
        assert_eq!(64, solve(input));
    }

    #[test]
    fn test_input() {
        let input = open("input/day16/input.txt").expect("Missing file");
        assert_eq!(692, solve(input));
    }
}
