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

    while let Some(index) = queue.pop_back() {
        // The number of edges equals the number of corners since there are 2 edges per corner and
        // vice versa. Thus, count corners.

        area += 1;
        let pos = map.pos(index);
        let north = pos.north(map).is_some_and(|p| p.value() == map[index]);
        let east = pos.east(map).is_some_and(|p| p.value() == map[index]);
        let south = pos.south(map).is_some_and(|p| p.value() == map[index]);
        let west = pos.west(map).is_some_and(|p| p.value() == map[index]);

        // Count convex corners
        perimeter += (!north && !east) as usize;
        perimeter += (!east && !south) as usize;
        perimeter += (!south && !west) as usize;
        perimeter += (!west && !north) as usize;

        // Count concave corners
        perimeter += (north
            && east
            && pos
                .north_east(map)
                .filter(|&p| p.value() == map[index])
                .is_none()) as usize;
        perimeter += (east
            && south
            && pos
                .south_east(map)
                .filter(|&p| p.value() == map[index])
                .is_none()) as usize;
        perimeter += (south
            && west
            && pos
                .south_west(map)
                .filter(|&p| p.value() == map[index])
                .is_none()) as usize;
        perimeter += (west
            && north
            && pos
                .north_west(map)
                .filter(|&p| p.value() == map[index])
                .is_none()) as usize;

        queue.extend(
            pos.neighbors(map)
                .filter(|&p| p.value() == map[start])
                .map(|p| p.index())
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
