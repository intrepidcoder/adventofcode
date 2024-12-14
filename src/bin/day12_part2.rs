use adventofcode::grid::{Grid, Neighbors, NeighborsDiag};
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
        // The number of edges equals the number of corners since there are 2 edges per corner and
        // vice versa. Thus, count corners.

        area += 1;
        let neighbors = map.neighbors(pos);
        let north = neighbors.north().is_some_and(|i| map[i] == map[pos]);
        let east = neighbors.east().is_some_and(|i| map[i] == map[pos]);
        let south = neighbors.south().is_some_and(|i| map[i] == map[pos]);
        let west = neighbors.west().is_some_and(|i| map[i] == map[pos]);

        // Count convex corners
        perimeter += (!north && !east) as usize;
        perimeter += (!east && !south) as usize;
        perimeter += (!south && !west) as usize;
        perimeter += (!west && !north) as usize;

        // Count concave corners
        let neighbors_diag = map.neighbors_diag(pos);
        perimeter += (north
            && east
            && neighbors_diag
                .north_east()
                .filter(|&i| map[i] == map[pos])
                .is_none()) as usize;
        perimeter += (east
            && south
            && neighbors_diag
                .south_east()
                .filter(|&i| map[i] == map[pos])
                .is_none()) as usize;
        perimeter += (south
            && west
            && neighbors_diag
                .south_west()
                .filter(|&i| map[i] == map[pos])
                .is_none()) as usize;
        perimeter += (west
            && north
            && neighbors_diag
                .north_west()
                .filter(|&i| map[i] == map[pos])
                .is_none()) as usize;

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
