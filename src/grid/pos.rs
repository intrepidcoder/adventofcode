use crate::grid::{DirectedPos, Direction, Grid};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Pos {
    index: usize,
    value: char,
    width: usize,
}

impl Pos {
    pub(crate) fn new(index: usize, value: char, width: usize) -> Self {
        Self {
            index,
            value,
            width,
        }
    }

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

    pub fn directed(self, dir: Direction) -> DirectedPos {
        DirectedPos::new(self, dir)
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
