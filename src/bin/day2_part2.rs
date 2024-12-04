use std::io;

#[derive(Debug)]
struct State {
    max: i64,
    min: i64,
    all_positive: bool,
    all_negative: bool,
}

fn main() {
    let result: i64 = io::stdin()
        .lines()
        .map(|line| {
            let line = line.expect("IO Error");
            let levels: Vec<i64> = line.split(' ').flat_map(|s| s.parse()).collect();
            (0..levels.len())
                .map(|skip_index| {
                    let initial_state = State {
                        max: 0,
                        min: 0,
                        all_positive: true,
                        all_negative: true,
                    };
                    let omit_iter = levels[0..skip_index]
                        .iter()
                        .chain(levels[skip_index + 1..levels.len()].iter());

                    omit_iter
                        .clone()
                        .zip(omit_iter.skip(1))
                        .map(|(a, b)| b - a)
                        .fold(initial_state, |state, diff| State {
                            max: state.max.max(diff),
                            min: state.min.min(diff),
                            all_positive: state.all_positive && diff > 0,
                            all_negative: state.all_negative && diff < 0,
                        })
                })
                .map(|state| {
                    (state.max <= 3
                        && state.min >= -3
                        && (state.all_positive || state.all_negative)) as i64
                })
                .max()
                .unwrap()
        })
        .sum();
    println!("{result}");
}
