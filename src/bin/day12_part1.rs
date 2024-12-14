use adventofcode::grid::Grid;
use std::collections::VecDeque;

fn main() {
    let map = Grid::read_from_input();
    let mut visited = vec![false; map.len()];
    let result: usize = (0..map.len())
        .map(|i| {
            if !visited[i] {
                floodfill(&map, &mut visited, i)
            } else {
                0
            }
        })
        .sum();

    println!("{result}");
}

fn floodfill(map: &Grid, visited: &mut [bool], start: usize) -> usize {
    let mut queue = VecDeque::new();
    queue.push_back(start);
    let mut area = 0;
    let mut perimeter = 0;
    visited[start] = true;

    while let Some(pos) = queue.pop_back() {
        area += 1;

        perimeter += 4 - map
            .neighbors(pos)
            .filter(|&next| map[next] == map[start])
            .count();
        queue.extend(
            map.neighbors(pos)
                .filter(|&next| map[next] == map[start])
                .filter(|&next| {
                    let v = visited[next];
                    if !v {
                        visited[next] = true;
                    }
                    !v
                }),
        );
    }

    area * perimeter
}
