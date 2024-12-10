use std::{
    collections::BTreeSet,
    io::{self, Read},
    iter,
};

fn main() {
    let mut disk_map = String::new();
    io::stdin()
        .lock()
        .read_to_string(&mut disk_map)
        .expect("Error reading input");

    let mut blocks = vec![];
    let mut free_queues: Vec<BTreeSet<usize>> =
        iter::repeat_n(0, 10).map(|_| BTreeSet::new()).collect();
    let mut pos = 0;

    for (i, c) in disk_map.trim().char_indices() {
        let length = c.to_digit(10).unwrap() as usize;
        if i & 1 == 0 {
            blocks.push((pos, length, (i / 2)));
        } else {
            free_queues[length].insert(pos);
        }
        pos += length
    }

    let result: usize = blocks
        .into_iter()
        .rev()
        .map(|(pos, length, id)| {
            if let Some((earlier_pos, len)) = (length..10)
                .filter_map(|len| {
                    free_queues[len]
                        .first()
                        .filter(|&&p| p < pos)
                        .map(|&p| (p, len))
                })
                .min()
            {
                free_queues[len].pop_first();
                if len - length > 0 {
                    free_queues[len - length].insert(earlier_pos + length);
                }

                id * (earlier_pos..earlier_pos + length).sum::<usize>()
            } else {
                id * (pos..pos + length).sum::<usize>()
            }
        })
        .sum();

    println!("{result}");
}
