use crate::board::{Position, Direction};

pub(crate) struct Robot {
    pub(crate) direction: Direction,
    pub(crate) position: Position,
}

impl Robot {
    pub(crate) fn new() -> Robot {
        Robot {
            direction: Direction::East,
            position: Position::new(0, 0),
        }
    }
}