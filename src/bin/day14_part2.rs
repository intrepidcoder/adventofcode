use itertools::Itertools;
use std::io;

const WIDTH: usize = 101;
const HEIGHT: usize = 103;

fn print_robots(robots: &[(usize, usize, usize, usize)], iterations: usize) {
    let mut grid = vec!['.'; WIDTH * HEIGHT];
    robots
        .iter()
        .map(|(px, py, vx, vy)| {
            (
                (px + vx * iterations) % WIDTH,
                (py + vy * iterations) % HEIGHT,
            )
        })
        .for_each(|(x, y)| grid[x + y * WIDTH] = '#');
    grid.chunks_exact(WIDTH)
        .for_each(|chunk| println!("{}", chunk.iter().collect::<String>()));
}

fn main() {
    const I_WIDTH: isize = WIDTH as isize;
    const I_HEIGHT: isize = HEIGHT as isize;
    let robots: Vec<_> = io::stdin()
        .lines()
        .flat_map(|line| -> Vec<isize> {
            line.unwrap()
                .split_terminator(&['=', ',', ' ', '\n'][..])
                .filter_map(|s| s.parse().ok())
                .collect()
        })
        .tuples()
        .map(|(px, py, vx, vy)| {
            (
                px as usize,
                py as usize,
                ((vx + I_WIDTH) % I_WIDTH) as usize,
                ((vy + I_HEIGHT) % I_HEIGHT) as usize,
            )
        })
        .collect();

    // Look for the iteration that minimizes the sum of squared distances from the center
    // of mass in both x and y directions to find a cluster.
    let (result, _) = (1..WIDTH * HEIGHT)
        .map(|i| {
            let center_of_mass_x: usize = robots
                .iter()
                .map(|(px, py, vx, vy)| ((px + vx * i) % WIDTH, (py + vy * i) % HEIGHT))
                .map(|(x, _)| x)
                .sum::<usize>()
                / robots.len();
            let moment_x: usize = robots
                .iter()
                .map(|(px, py, vx, vy)| ((px + vx * i) % WIDTH, (py + vy * i) % HEIGHT))
                .map(|(x, _): (usize, _)| {
                    x.abs_diff(center_of_mass_x) * x.abs_diff(center_of_mass_x)
                })
                .sum();
            let center_of_mass_y: usize = robots
                .iter()
                .map(|(px, py, vx, vy)| ((px + vx * i) % WIDTH, (py + vy * i) % HEIGHT))
                .map(|(_, y)| y)
                .sum::<usize>()
                / robots.len();
            let moment_y: usize = robots
                .iter()
                .map(|(px, py, vx, vy)| ((px + vx * i) % WIDTH, (py + vy * i) % HEIGHT))
                .map(|(_, y): (_, usize)| {
                    y.abs_diff(center_of_mass_y) * y.abs_diff(center_of_mass_y)
                })
                .sum();
            (i, moment_x + moment_y)
        })
        .min_by_key(|&(_, m)| m)
        .unwrap();

    // Sanity check the result since the solution is heuristic.
    print_robots(&robots, result);

    println!("{result}");
}
