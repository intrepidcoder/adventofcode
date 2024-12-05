use std::io;

#[derive(Default)]
struct Sum {
    value: usize,
}

impl Extend<usize> for Sum {
    fn extend<T: IntoIterator<Item = usize>>(&mut self, iter: T) {
        for elem in iter {
            self.value += elem;
        }
    }
}

fn main() {
    let (grid, forward_counts): (Vec<Vec<char>>, Sum) = io::stdin()
        .lines()
        .map(|line| {
            let line = line.expect("IO error");
            let count = line.matches("XMAS").count() + line.matches("SAMX").count();
            (line.chars().collect(), count)
        })
        .unzip();

    let mut result: usize = forward_counts.value;

    for x in 0..grid.len() {
        for y in 0..grid[0].len() {
            // Vertical
            result += (0..4)
                .flat_map(|i| grid.get(x + i).and_then(|row| row.get(y)))
                .copied()
                .eq("XMAS".chars()) as usize;
            result += (0..4)
                .flat_map(|i| grid.get(x + i).and_then(|row| row.get(y)))
                .copied()
                .eq("SAMX".chars()) as usize;
            // Diagonal down
            result += (0..4)
                .flat_map(|i| grid.get(x + i).and_then(|row| row.get(y + i)))
                .copied()
                .eq("XMAS".chars()) as usize;
            result += (0..4)
                .flat_map(|i| grid.get(x + i).and_then(|row| row.get(y + i)))
                .copied()
                .eq("SAMX".chars()) as usize;
            // Diagonal up
            result += (0..4)
                .flat_map(|i| grid.get(x + i).and_then(|row| row.get(y - i)))
                .copied()
                .eq("XMAS".chars()) as usize;
            result += (0..4)
                .flat_map(|i| grid.get(x + i).and_then(|row| row.get(y - i)))
                .copied()
                .eq("SAMX".chars()) as usize;
        }
    }
    println!("{result}");
}
