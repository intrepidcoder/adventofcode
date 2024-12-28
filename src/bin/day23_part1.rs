use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    io::{self, BufRead},
};

fn main() {
    println!("{}", solve(io::stdin().lock()));
}

#[derive(Debug)]
struct Node {
    first: char,
    adj: HashSet<usize>,
}

fn solve(input: impl BufRead) -> usize {
    let mut id_counter = 0;
    let mut nodes: Vec<Node> = Vec::new();
    let mut nodes_ids: HashMap<(char, char), usize> = HashMap::new();

    let edges: Vec<_> = input
        .lines()
        .map(|line| {
            line.unwrap()
                .split('-')
                .map(|s| s.chars().collect_tuple::<(char, char)>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .map(|(u, v)| {
            let u_id = *nodes_ids.entry(u).or_insert_with(|| {
                let node = Node {
                    first: u.0,
                    adj: HashSet::new(),
                };
                nodes.push(node);
                id_counter += 1;
                id_counter - 1
            });
            let v_id = *nodes_ids.entry(v).or_insert_with(|| {
                let node = Node {
                    first: v.0,
                    adj: HashSet::new(),
                };
                nodes.push(node);
                id_counter += 1;
                id_counter - 1
            });
            nodes[u_id].adj.insert(v_id);
            nodes[v_id].adj.insert(u_id);
            (u_id, v_id)
        })
        .collect();

    edges
        .into_iter()
        .flat_map(|(u, v)| {
            nodes[u]
                .adj
                .intersection(&nodes[v].adj)
                .map(move |&w| (u, v, w))
        })
        .filter(|&(u, v, w)| {
            nodes[u].first == 't' || nodes[v].first == 't' || nodes[w].first == 't'
        })
        .count()
        / 3
}

#[cfg(test)]
mod test {
    use super::solve;
    use std::{
        fs::File,
        io::{self, BufReader},
    };

    fn open(path: &str) -> io::Result<BufReader<File>> {
        let file = File::open(path)?;
        Ok(BufReader::new(file))
    }

    #[test]
    fn test_example() {
        let input = open("input/day23/example.txt").expect("Missing file");
        assert_eq!(7, solve(input));
    }

    #[test]
    fn test_input() {
        let input = open("input/day23/input.txt").expect("Missing file");
        assert_eq!(1467, solve(input));
    }
}
