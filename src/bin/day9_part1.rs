use std::io::{self, Read};

#[derive(Debug)]
enum Block {
    File { length: usize, id: usize },
    Free { length: usize },
}

fn main() {
    let mut disk_map = String::new();
    io::stdin()
        .lock()
        .read_to_string(&mut disk_map)
        .expect("Error reading input");

    let mut blocks: Vec<_> = disk_map
        .trim()
        .char_indices()
        .map(|(i, c)| match i & 1 {
            0 => Block::File {
                length: c.to_digit(10).unwrap() as usize,
                id: (i / 2),
            },
            1 => Block::Free {
                length: c.to_digit(10).unwrap() as usize,
            },
            _ => unreachable!(),
        })
        .collect();

    let mut start = 0;
    let mut end = blocks.len() - 1;
    let mut pos: usize = 0;
    let mut result: usize = 0;

    while start < end {
        let (prefix, suffix) = blocks.split_at_mut(end);
        match prefix[start] {
            Block::File { length, id } => {
                result += id * (pos..pos + length).sum::<usize>();
                pos += length;
                start += 1;
            }
            Block::Free { length: 0 } => start += 1,
            Block::Free {
                length: ref mut start_length,
            } => match suffix[0] {
                Block::File {
                    length: ref mut end_length,
                    id: end_id,
                } => {
                    let length: usize = *start_length.min(end_length);
                    result += end_id * (pos..pos + length).sum::<usize>();
                    *end_length -= length;
                    *start_length -= length;
                    pos += length;
                    if *start_length == 0 {
                        start += 1;
                    }
                    if *end_length == 0 {
                        end -= 1;
                    }
                }
                Block::Free { .. } => end -= 1,
            },
        }
    }
    if start == end {
        if let Block::File { length, id } = blocks[start] {
            result += id * (pos..pos + length).sum::<usize>();
        }
    }

    println!("{result}");
}
