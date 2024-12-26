use std::{
    fmt::{self, Display, Write},
    io::{self, BufRead},
    ops::Deref,
    str::FromStr,
};

mod directed_pos;
mod pos;
pub use directed_pos::DirectedPos;
pub use directed_pos::Direction;
pub use pos::Pos;

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
        Pos::new(index, self.grid[index], self.width)
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

impl FromStr for Grid {
    type Err = GridParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut grid = Vec::new();
        let mut width = 0;
        let mut height = 0;

        for line in s.trim().split_terminator('\n') {
            grid.extend(line.chars());
            if width == 0 {
                width = line.len();
            } else if line.len() != width {
                return Err(GridParseError::UnevenWidths);
            }
            height += 1;
        }

        Ok(Self {
            grid,
            width,
            height,
        })
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum GridParseError {
    UnevenWidths,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get() {
        let grid: Grid = "abc\ndef\nghi\njkl".parse().unwrap();
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
        let grid: Grid = "abc\ndef\nghi\njkl".parse().unwrap();
        assert_eq!(grid.len(), 12);
        assert!(grid.iter().copied().eq("abcdefghijkl".chars()));

        for i in 0..grid.len() {
            let (r, c) = grid.coords(i);
            assert_eq!(Some(grid[i]), grid.get(r, c));
        }
    }

    #[test]
    fn test_set_index() {
        let mut grid: Grid = "abc\ndef\nghi\njkl".parse().unwrap();
        for i in 0..grid.len() {
            assert_ne!(grid[i], '.');
            grid.set_index(i, '.');
            assert_eq!(grid[i], '.');
        }
    }

    #[test]
    fn test_from_str() {
        assert!("abc\ndef\nghi\njkl".parse::<Grid>().is_ok());
        assert!("abc\nde\nghi\njkl"
            .parse::<Grid>()
            .is_err_and(|e| e == GridParseError::UnevenWidths));
    }
}
