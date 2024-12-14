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
        let (x, y) = map.coords(pos);
        let digit = map[pos].to_digit(10).unwrap();
        let next_digit = char::from_digit(digit + 1, 10);
        if digit == 9 {
            result += 1;
            continue;
        }

        queue.extend(
            map.neighbors(x, y)
                .filter(|&(r, c)| map.get(r, c) == next_digit)
                .map(|(r, c)| r * map.width() + c)
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
