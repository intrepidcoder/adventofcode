use itertools::Itertools;
use std::io;

fn main() {
    const ITERATIONS: isize = 100;
    const WIDTH: isize = 101;
    const HEIGHT: isize = 103;
    let (q1, q2, q3, q4) = io::stdin()
        .lines()
        .flat_map(|line| -> Vec<isize> {
            line.unwrap()
                .split_terminator(&['=', ',', ' ', '\n'][..])
                .filter_map(|s| s.parse().ok())
                .collect()
        })
        .tuples()
        .map(|(px, py, vx, vy)| (px, py, (vx + WIDTH) % WIDTH, (vy + HEIGHT) % HEIGHT))
        .map(|(px, py, vx, vy)| {
            (
                (px + vx * ITERATIONS) % WIDTH,
                (py + vy * ITERATIONS) % HEIGHT,
            )
        })
        .fold(
            (0, 0, 0, 0),
            |(mut q1, mut q2, mut q3, mut q4), (final_x, final_y)| {
                if (0..(WIDTH / 2)).contains(&final_x) {
                    if (0..HEIGHT / 2).contains(&final_y) {
                        q1 += 1;
                    } else if ((HEIGHT / 2 + 1)..HEIGHT).contains(&final_y) {
                        q2 += 1;
                    }
                }
                if (WIDTH / 2 + 1..WIDTH).contains(&final_x) {
                    if (0..HEIGHT / 2).contains(&final_y) {
                        q3 += 1;
                    } else if ((HEIGHT / 2 + 1)..HEIGHT).contains(&final_y) {
                        q4 += 1;
                    }
                }
                (q1, q2, q3, q4)
            },
        );
    println!("{}", q1 * q2 * q3 * q4);
}
