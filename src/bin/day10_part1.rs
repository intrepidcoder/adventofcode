use adventofcode::grid::Grid;
use std::collections::VecDeque;

fn main() {
    let map = Grid::read_from_input();
    let result: usize = (0..map.len())
        .filter(|&i| map[i] == '0')
        .map(|i| floodfill(&map, i))
        .sum();

    println!("{result}");
}

fn floodfill(map: &Grid, start: usize) -> usize {
    let mut visited = vec![false; map.len()];
    let mut queue = VecDeque::new();
    queue.push_back(start);
    let mut result = 0;
    visited[start] = true;

    while let Some(pos) = queue.pop_back() {
        let digit = map[pos].to_digit(10).unwrap();
        if digit == 9 {
            result += 1;
            continue;
        }
        let next_digit = char::from_digit(digit + 1, 10).unwrap();

        queue.extend(
            map.neighbors(pos)
                .filter(|&next| map[next] == next_digit)
                .filter(|&next| {
                    let v = visited[next];
                    if !v {
                        visited[next] = true;
                    }
                    !v
                }),
        );
    }

    result
}
