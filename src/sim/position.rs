use crate::sim::direction::Direction;

#[derive(Copy, Clone)]
pub(crate) struct Position {
    pub(crate) row: usize,
    pub(crate) col: usize,
}

impl Position {
    pub(crate) fn new(row: usize, col: usize) -> Position {
        Position { row, col }
    }

    fn from(other: &Position, row_d: isize, col_d: isize) -> Option<Position> {
        let new_row = other.row as isize + row_d;
        let new_col = other.col as isize + col_d;
        if new_row < 0 || new_col < 0 {
            None
        } else {
            Some(Position::new(new_row as usize, new_col as usize))
        }
    }

    pub(crate) fn shifted(&self, direction: Direction) -> Option<Position> {
        match direction {
            Direction::North => Position::from(&self, -1, 0),
            Direction::East => Position::from(&self, 0, 1),
            Direction::South => Position::from(&self, 1, 0),
            Direction::West => Position::from(&self, 0, -1),
        }
    }
}