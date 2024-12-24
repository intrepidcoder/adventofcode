use crate::grid::{Grid, Pos};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DirectedPos {
    pos: Pos,
    dir: Direction,
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

impl DirectedPos {
    pub fn new(pos: Pos, dir: Direction) -> Self {
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

#[cfg(test)]
mod test {
    use super::*;

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
}
