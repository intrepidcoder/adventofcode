use std::{
    fmt::{self, Display, Write},
    io,
    ops::Deref,
};

#[derive(Debug)]
pub struct Grid {
    grid: Vec<char>,
    width: usize,
    height: usize,
}

impl Grid {
    pub fn new(s: String) -> Self {
        let mut grid = Vec::new();
        let mut width = 0;
        let mut height = 0;

        for line in s.split_terminator('\n') {
            grid.extend(line.chars());
            if width == 0 {
                width = grid.len();
            }
            height += 1;
        }

        Self {
            grid,
            width,
            height,
        }
    }

    pub fn read_from_input() -> Self {
        let mut grid = Vec::new();
        let mut width = 0;
        let mut height = 0;
        let lines = io::stdin()
            .lines()
            .map(|line| line.expect("IO error"))
            .take_while(|line| !line.is_empty());
        for line in lines {
            grid.extend(line.chars());
            if width == 0 {
                width = grid.len();
            }
            height += 1;
        }

        Self {
            grid,
            width,
            height,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn coords(&self, index: usize) -> (usize, usize) {
        (index / self.width, index % self.width)
    }

    /// Get a grid element (r, c) where r is the zero-based row index counted from the top and c is
    /// the zero-based column index counted from the left.
    pub fn get(&self, r: usize, c: usize) -> Option<char> {
        if !self.in_bounds(r, c) {
            None
        } else {
            Some(self.grid[r * self.width + c])
        }
    }

    fn in_bounds(&self, r: usize, c: usize) -> bool {
        (0..self.height).contains(&r) && (0..self.width).contains(&c)
    }

    /// Set a grid element by index.
    pub fn set(&mut self, index: usize, val: char) {
        self.grid[index] = val;
    }

    /// Find first index of c in the grid
    pub fn find_char(&self, ch: char) -> Option<usize> {
        self.grid
            .iter()
            .enumerate()
            .find(|&(_, c)| *c == ch)
            .map(|(i, _)| i)
    }

    pub fn neighbors(&self, index: usize) -> impl Neighbors {
        let (row, col) = self.coords(index);
        NeighborsIter([
            // north
            row.checked_sub(1)
                .filter(|&r| self.in_bounds(r, col))
                .map(|r| r * self.width + col),
            // east
            Some((row, col + 1))
                .filter(|&(r, c)| self.in_bounds(r, c))
                .map(|(r, c)| r * self.width + c),
            // south
            Some((row + 1, col))
                .filter(|&(r, c)| self.in_bounds(r, c))
                .map(|(r, c)| r * self.width + c),
            // west
            col.checked_sub(1)
                .filter(|&c| self.in_bounds(row, c))
                .map(|c| row * self.width + c),
        ])
    }

    /// Returns an Iterator over the 4 diagonal neighbors of a grid location
    pub fn neighbors_diag(&self, index: usize) -> impl NeighborsDiag {
        let (row, col) = self.coords(index);
        NeighborsIter([
            // north east
            row.checked_sub(1)
                .filter(|&r| self.in_bounds(r, col + 1))
                .map(|r| r * self.width + (col + 1)),
            // south east
            Some((row + 1, col + 1))
                .filter(|&(r, c)| self.in_bounds(r, c))
                .map(|(r, c)| r * self.width + c),
            // south west
            col.checked_sub(1)
                .filter(|&c| self.in_bounds(row + 1, c))
                .map(|c| (row + 1) * self.width + c),
            // north west
            row.checked_sub(1)
                .zip(col.checked_sub(1))
                .filter(|&(r, c)| self.in_bounds(r, c))
                .map(|(r, c)| r * self.width + c),
        ])
    }
}

impl Deref for Grid {
    type Target = [char];
    fn deref(&self) -> &Self::Target {
        self.grid.as_slice()
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.grid.chunks_exact(self.width).try_for_each(|line| {
            let buffer: String = line.iter().collect();
            f.write_str(&buffer).and_then(|_| f.write_char('\n'))
        })
    }
}

pub trait Neighbors: Iterator<Item = usize> {
    fn north(&self) -> Option<usize>;
    fn east(&self) -> Option<usize>;
    fn south(&self) -> Option<usize>;
    fn west(&self) -> Option<usize>;
}

pub trait NeighborsDiag: Iterator<Item = usize> {
    fn north_east(&self) -> Option<usize>;
    fn south_east(&self) -> Option<usize>;
    fn south_west(&self) -> Option<usize>;
    fn north_west(&self) -> Option<usize>;
}

pub struct NeighborsIter([Option<usize>; 4]);

impl Neighbors for NeighborsIter {
    fn north(&self) -> Option<usize> {
        self.0[0]
    }

    fn east(&self) -> Option<usize> {
        self.0[1]
    }

    fn south(&self) -> Option<usize> {
        self.0[2]
    }

    fn west(&self) -> Option<usize> {
        self.0[3]
    }
}

impl Iterator for NeighborsIter {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.iter_mut().find_map(|opt| opt.take())
    }
}

impl NeighborsDiag for NeighborsIter {
    fn north_east(&self) -> Option<usize> {
        self.0[0]
    }

    fn south_east(&self) -> Option<usize> {
        self.0[1]
    }

    fn south_west(&self) -> Option<usize> {
        self.0[2]
    }

    fn north_west(&self) -> Option<usize> {
        self.0[3]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get() {
        let s = "abc\ndef\nghi\njkl".to_string();
        let grid = Grid::new(s);
        for (i, c) in ('a'..='l').enumerate() {
            assert_eq!(
                grid.get(i / 3, i % 3),
                Some(c),
                "Mismatch of {c} at index {i}"
            );
        }
    }

    #[test]
    fn test_deref() {
        let s = "abc\ndef\nghi\njkl".to_string();
        let grid = Grid::new(s);
        assert_eq!(grid.len(), 12);
        assert!(grid.iter().copied().eq("abcdefghijkl".chars()));

        for i in 0..grid.len() {
            let (r, c) = grid.coords(i);
            assert_eq!(Some(grid[i]), grid.get(r, c));
        }
    }

    #[test]
    fn test_neighbors() {
        let grid = Grid::new("012\n345\n678".to_string());
        let mut neighbors = Vec::new();
        neighbors.extend(grid.neighbors(4));
        assert_eq!(neighbors, vec![1, 5, 7, 3]);
        neighbors.clear();

        neighbors.extend(grid.neighbors(0));
        assert_eq!(neighbors, vec![1, 3]);
        neighbors.clear();

        neighbors.extend(grid.neighbors(8));
        assert_eq!(neighbors, vec![5, 7]);
        neighbors.clear();

        assert_eq!(grid.neighbors(4).north(), Some(1));
        assert_eq!(grid.neighbors(4).east(), Some(5));
        assert_eq!(grid.neighbors(4).south(), Some(7));
        assert_eq!(grid.neighbors(4).west(), Some(3));
    }

    #[test]
    fn test_neighbors_diag() {
        let grid = Grid::new("012\n345\n678".to_string());
        let mut neighbors = Vec::new();
        neighbors.extend(grid.neighbors_diag(4));
        assert_eq!(neighbors, vec![2, 8, 6, 0]);
        neighbors.clear();

        neighbors.extend(grid.neighbors_diag(0));
        assert_eq!(neighbors, vec![4]);
        neighbors.clear();

        neighbors.extend(grid.neighbors_diag(8));
        assert_eq!(neighbors, vec![4]);
        neighbors.clear();

        neighbors.extend(grid.neighbors_diag(1));
        assert_eq!(neighbors, vec![5, 3]);
        neighbors.clear();

        neighbors.extend(grid.neighbors_diag(5));
        assert_eq!(neighbors, vec![7, 1]);
        neighbors.clear();

        assert_eq!(grid.neighbors_diag(4).north_east(), Some(2));
        assert_eq!(grid.neighbors_diag(4).south_east(), Some(8));
        assert_eq!(grid.neighbors_diag(4).south_west(), Some(6));
        assert_eq!(grid.neighbors_diag(4).north_west(), Some(0));
    }
}
