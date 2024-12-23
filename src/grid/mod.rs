use std::{
    fmt::{self, Display, Write},
    io::{self, BufRead},
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
        Grid::read_from_buf(&mut io::stdin().lock())
    }

    pub fn read_from_buf(input: &mut impl BufRead) -> Self {
        let mut grid = Vec::new();
        let mut width = 0;
        let mut height = 0;
        let lines = input
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

    /// Get a grid element (r, c) where r is the zero-based row index counted from the top and c is
    /// the zero-based column index counted from the left.
    pub fn get_coords(&self, r: usize, c: usize) -> Option<Pos> {
        if !self.in_bounds(r, c) {
            None
        } else {
            Some(self.pos(r * self.width + c))
        }
    }

    fn in_bounds(&self, r: usize, c: usize) -> bool {
        (0..self.height).contains(&r) && (0..self.width).contains(&c)
    }

    /// Set a grid element by index.
    pub fn set_index(&mut self, index: usize, val: char) {
        self.grid[index] = val;
    }

    /// Set a grid element by pos.
    pub fn set(&mut self, pos: Pos, val: char) {
        self.set_index(pos.index(), val);
    }

    /// Find first index of c in the grid
    pub fn find_char(&self, ch: char) -> Option<Pos> {
        self.grid
            .iter()
            .enumerate()
            .find(|&(_, c)| *c == ch)
            .map(|(i, _)| self.pos(i))
    }

    pub fn pos(&self, index: usize) -> Pos {
        Pos {
            index,
            value: self.grid[index],
            width: self.width,
        }
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Pos {
    index: usize,
    value: char,
    width: usize,
}

impl Pos {
    pub fn index(&self) -> usize {
        self.index
    }

    pub fn value(&self) -> char {
        self.value
    }

    pub fn row(&self) -> usize {
        self.index / self.width
    }

    pub fn col(&self) -> usize {
        self.index % self.width
    }

    pub fn north(&self, grid: &Grid) -> Option<Pos> {
        self.row()
            .checked_sub(1)
            .and_then(|r| grid.get_coords(r, self.col()))
    }

    pub fn north_east(&self, grid: &Grid) -> Option<Pos> {
        self.row()
            .checked_sub(1)
            .and_then(|r| grid.get_coords(r, self.col() + 1))
    }

    pub fn east(&self, grid: &Grid) -> Option<Pos> {
        grid.get_coords(self.row(), self.col() + 1)
    }

    pub fn south_east(&self, grid: &Grid) -> Option<Pos> {
        grid.get_coords(self.row() + 1, self.col() + 1)
    }

    pub fn south(&self, grid: &Grid) -> Option<Pos> {
        grid.get_coords(self.row() + 1, self.col())
    }

    pub fn south_west(&self, grid: &Grid) -> Option<Pos> {
        self.col()
            .checked_sub(1)
            .and_then(|c| grid.get_coords(self.row() + 1, c))
    }

    pub fn west(&self, grid: &Grid) -> Option<Pos> {
        self.col()
            .checked_sub(1)
            .and_then(|c| grid.get_coords(self.row(), c))
    }

    pub fn north_west(&self, grid: &Grid) -> Option<Pos> {
        self.row()
            .checked_sub(1)
            .zip(self.col().checked_sub(1))
            .and_then(|(r, c)| grid.get_coords(r, c))
    }

    pub fn neighbors(&self, grid: &Grid) -> NeighborsIter {
        NeighborsIter([
            self.north(grid),
            self.east(grid),
            self.south(grid),
            self.west(grid),
            None,
            None,
            None,
            None,
        ])
    }

    pub fn neighbors_diag(&self, grid: &Grid) -> NeighborsIter {
        NeighborsIter([
            self.north_east(grid),
            self.south_east(grid),
            self.south_west(grid),
            self.north_west(grid),
            None,
            None,
            None,
            None,
        ])
    }

    pub fn neighbors_all(&self, grid: &Grid) -> NeighborsIter {
        NeighborsIter([
            self.north(grid),
            self.north_east(grid),
            self.east(grid),
            self.south_east(grid),
            self.south(grid),
            self.south_west(grid),
            self.west(grid),
            self.north_west(grid),
        ])
    }
}

pub struct NeighborsIter([Option<Pos>; 8]);

impl Iterator for NeighborsIter {
    type Item = Pos;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.iter_mut().find_map(|opt| opt.take())
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
    fn test_set_index() {
        let mut grid = Grid::new("abc\ndef\nghi\njkl".to_string());
        for i in 0..grid.len() {
            assert_ne!(grid[i], '.');
            grid.set_index(i, '.');
            assert_eq!(grid[i], '.');
        }
    }

    #[test]
    fn test_directions() {
        let grid = Grid::new("012\n345\n678".to_string());
        let pos = grid.pos(4);
        assert_eq!(pos.north(&grid).unwrap().index(), 1);
        assert_eq!(pos.north_east(&grid).unwrap().index(), 2);
        assert_eq!(pos.east(&grid).unwrap().index(), 5);
        assert_eq!(pos.south_east(&grid).unwrap().index(), 8);
        assert_eq!(pos.south(&grid).unwrap().index(), 7);
        assert_eq!(pos.south_west(&grid).unwrap().index(), 6);
        assert_eq!(pos.west(&grid).unwrap().index(), 3);
        assert_eq!(pos.north_west(&grid).unwrap().index(), 0);

        let pos = grid.pos(0);
        assert_eq!(pos.north(&grid), None);
        assert_eq!(pos.east(&grid).unwrap().index(), 1);
        assert_eq!(pos.south(&grid).unwrap().index(), 3);
        assert_eq!(pos.west(&grid), None);
    }

    #[test]
    fn test_neighbors() {
        let grid = Grid::new("012\n345\n678".to_string());
        let mut neighbors = Vec::new();
        neighbors.extend(grid.pos(4).neighbors(&grid).map(|p| p.index()));
        assert_eq!(neighbors, vec![1, 5, 7, 3]);
        neighbors.clear();

        neighbors.extend(grid.pos(0).neighbors(&grid).map(|p| p.index()));
        assert_eq!(neighbors, vec![1, 3]);
        neighbors.clear();

        neighbors.extend(grid.pos(8).neighbors(&grid).map(|p| p.index()));
        assert_eq!(neighbors, vec![5, 7]);
        neighbors.clear();
    }

    #[test]
    fn test_neighbors_diag() {
        let grid = Grid::new("012\n345\n678".to_string());
        let mut neighbors = Vec::new();
        neighbors.extend(grid.pos(4).neighbors_diag(&grid).map(|p| p.index()));
        assert_eq!(neighbors, vec![2, 8, 6, 0]);
        neighbors.clear();

        neighbors.extend(grid.pos(0).neighbors_diag(&grid).map(|p| p.index()));
        assert_eq!(neighbors, vec![4]);
        neighbors.clear();

        neighbors.extend(grid.pos(8).neighbors_diag(&grid).map(|p| p.index()));
        assert_eq!(neighbors, vec![4]);
        neighbors.clear();

        neighbors.extend(grid.pos(1).neighbors_diag(&grid).map(|p| p.index()));
        assert_eq!(neighbors, vec![5, 3]);
        neighbors.clear();

        neighbors.extend(grid.pos(5).neighbors_diag(&grid).map(|p| p.index()));
        assert_eq!(neighbors, vec![7, 1]);
        neighbors.clear();
    }
}
