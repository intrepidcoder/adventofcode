use std::{io, ops::Deref};

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
        for line in io::stdin().lines() {
            let line = line.expect("IO error");

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

    pub fn neighbors(&self, index: usize) -> Neighbors {
        let (row, col) = self.coords(index);
        Neighbors {
            north: row
                .checked_sub(1)
                .filter(|&r| self.in_bounds(r, col))
                .map(|r| r * self.width + col),
            west: Some((row, col + 1))
                .filter(|&(r, c)| self.in_bounds(r, c))
                .map(|(r, c)| r * self.width + c),
            south: Some((row + 1, col))
                .filter(|&(r, c)| self.in_bounds(r, c))
                .map(|(r, c)| r * self.width + c),
            east: col
                .checked_sub(1)
                .filter(|&c| self.in_bounds(row, c))
                .map(|c| row * self.width + c),
        }
    }
}

impl Deref for Grid {
    type Target = [char];
    fn deref(&self) -> &Self::Target {
        self.grid.as_slice()
    }
}

pub struct Neighbors {
    north: Option<usize>,
    west: Option<usize>,
    south: Option<usize>,
    east: Option<usize>,
}

impl Iterator for Neighbors {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        self.north.take().or_else(|| {
            self.west
                .take()
                .or_else(|| self.south.take().or_else(|| self.east.take()))
        })
    }
}

#[cfg(test)]
mod test {
    use super::Grid;

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
    }
}
