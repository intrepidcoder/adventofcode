use crate::grid::{Grid, Pos};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DirectedPos {
    pos: Pos,
    dir: Direction,
}

impl DirectedPos {
    pub(crate) fn new(pos: Pos, dir: Direction) -> Self {
        Self { pos, dir }
    }

    pub fn pos(&self) -> Pos {
        self.pos
    }

    pub fn direction(&self) -> Direction {
        self.dir
    }

    pub fn index(&self) -> usize {
        self.pos.index()
    }

    pub fn value(&self) -> char {
        self.pos.value()
    }

    pub fn row(&self) -> usize {
        self.pos.row()
    }

    pub fn col(&self) -> usize {
        self.pos.col()
    }

    /// Rotate direction by 90 degrees clockwise
    pub fn cw(self) -> Self {
        Self {
            pos: self.pos,
            dir: self.dir.cw(),
        }
    }

    /// Rotate direction by 90 degrees counter-clockwise
    pub fn ccw(self) -> Self {
        Self {
            pos: self.pos,
            dir: self.dir.ccw(),
        }
    }

    /// Rotate direction by 45 degrees clockwise
    pub fn cw45(self) -> Self {
        Self {
            pos: self.pos,
            dir: self.dir.cw45(),
        }
    }

    /// Rotate direction by 45 degrees counter-clockwise
    pub fn ccw45(self) -> Self {
        Self {
            pos: self.pos,
            dir: self.dir.ccw45(),
        }
    }

    /// Rotate direction by 180 degrees
    pub fn reverse(self) -> Self {
        Self {
            pos: self.pos,
            dir: self.dir.reverse(),
        }
    }

    pub fn advance(&self, grid: &Grid) -> Option<Self> {
        match self.dir {
            Direction::North => self.north(grid),
            Direction::NorthEast => self.north_east(grid),
            Direction::East => self.east(grid),
            Direction::SouthEast => self.south_east(grid),
            Direction::South => self.south(grid),
            Direction::SouthWest => self.south_west(grid),
            Direction::West => self.west(grid),
            Direction::NorthWest => self.north_west(grid),
        }
    }

    pub fn advance_iter(self, grid: &Grid) -> AdvanceIter {
        AdvanceIter {
            grid,
            pos: Some(self),
        }
    }

    pub fn north(&self, grid: &Grid) -> Option<DirectedPos> {
        self.pos.north(grid).map(|p| Self::new(p, self.dir))
    }

    pub fn north_east(&self, grid: &Grid) -> Option<DirectedPos> {
        self.pos.north_east(grid).map(|p| Self::new(p, self.dir))
    }

    pub fn east(&self, grid: &Grid) -> Option<DirectedPos> {
        self.pos.east(grid).map(|p| Self::new(p, self.dir))
    }

    pub fn south_east(&self, grid: &Grid) -> Option<DirectedPos> {
        self.pos.south_east(grid).map(|p| Self::new(p, self.dir))
    }

    pub fn south(&self, grid: &Grid) -> Option<DirectedPos> {
        self.pos.south(grid).map(|p| Self::new(p, self.dir))
    }

    pub fn south_west(&self, grid: &Grid) -> Option<DirectedPos> {
        self.pos.south_west(grid).map(|p| Self::new(p, self.dir))
    }

    pub fn west(&self, grid: &Grid) -> Option<DirectedPos> {
        self.pos.west(grid).map(|p| Self::new(p, self.dir))
    }

    pub fn north_west(&self, grid: &Grid) -> Option<DirectedPos> {
        self.pos.north_west(grid).map(|p| Self::new(p, self.dir))
    }
}

pub struct AdvanceIter<'grid> {
    grid: &'grid Grid,
    pos: Option<DirectedPos>,
}

impl Iterator for AdvanceIter<'_> {
    type Item = DirectedPos;
    fn next(&mut self) -> Option<Self::Item> {
        self.pos = self.pos.and_then(|p| p.advance(self.grid));
        self.pos
    }
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    North = 0,
    NorthEast = 1,
    East = 2,
    SouthEast = 3,
    South = 4,
    SouthWest = 5,
    West = 6,
    NorthWest = 7,
}

impl Direction {
    /// Rotate by 90 degrees clockwise
    pub fn cw(self) -> Self {
        Self::cast((self as u8) + 2)
    }

    /// Rotate by 90 degrees counter-clockwise
    pub fn ccw(self) -> Self {
        Self::cast((self as u8) + 6)
    }

    /// Rotate by 45 degrees clockwise
    pub fn cw45(self) -> Self {
        Self::cast((self as u8) + 1)
    }

    /// Rotate by 45 degrees counter-clockwise
    pub fn ccw45(self) -> Self {
        Self::cast((self as u8) + 7)
    }

    /// Rotate by 180 degrees
    pub fn reverse(self) -> Self {
        Self::cast((self as u8) + 4)
    }

    fn cast(val: u8) -> Self {
        let mask = 7;

        match val & mask {
            0 => Self::North,
            1 => Self::NorthEast,
            2 => Self::East,
            3 => Self::SouthEast,
            4 => Self::South,
            5 => Self::SouthWest,
            6 => Self::West,
            7 => Self::NorthWest,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::iter;

    #[test]
    fn test_pos() {
        let grid = Grid::new("012\n345\n678".to_string());
        let pos = DirectedPos::new(grid.pos(4), Direction::North);
        assert_eq!(pos, grid.pos(4).directed(Direction::North));
        assert_eq!(pos.direction(), Direction::North);
        assert_eq!(pos.index(), 4);
        assert_eq!(
            pos.north(&grid).unwrap(),
            DirectedPos::new(grid.pos(1), Direction::North)
        );
    }

    #[test]
    fn test_rotations() {
        let grid = Grid::new("012\n345\n678".to_string());
        let first_pos = DirectedPos::new(grid.pos(4), Direction::North);

        for pos in iter::successors(Some(first_pos), |p| Some(p.cw45())).take(8) {
            assert_eq!(pos, pos.cw().ccw());
            assert_eq!(pos, pos.cw().ccw45().ccw45());
            assert_eq!(pos, pos.ccw().cw45().cw45());
            assert_eq!(pos.cw().cw(), pos.reverse());
        }
    }

    #[test]
    fn test_advance() {
        let grid = Grid::new("012\n345\n678".to_string());
        let mut pos = DirectedPos::new(grid.pos(0), Direction::East);
        pos = pos.advance(&grid).unwrap();
        assert_eq!(pos.index(), 1);
        assert_eq!(pos.direction(), Direction::East);
        pos = pos.cw().advance(&grid).unwrap();
        assert_eq!(pos.index(), 4);
        assert_eq!(pos.direction(), Direction::South);
        pos = pos.cw45().advance(&grid).unwrap();
        assert_eq!(pos.index(), 6);
        assert_eq!(pos.direction(), Direction::SouthWest);
        assert_eq!(pos.advance(&grid), None);
    }

    #[test]
    fn test_advance_iter() {
        let grid = Grid::new("012\n345\n678".to_string());
        let pos = grid.pos(0).directed(Direction::East);
        let mut actual = Vec::new();
        actual.extend(pos.advance_iter(&grid).map(|p| p.index()));
        assert_eq!(actual, vec![1, 2]);
        actual.clear();

        let pos = grid.pos(2).directed(Direction::SouthWest);
        actual.extend(pos.advance_iter(&grid).map(|p| p.index()));
        assert_eq!(actual, vec![4, 6]);
        actual.clear();
    }
}
