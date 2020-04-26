use crate::sim::turn_direction::TurnDirection;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub(crate) enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    pub(crate) fn turned(&self, direction: &TurnDirection) -> Direction {
        match direction {
            TurnDirection::Left => self.turned_left(),
            TurnDirection::Right => self.turned_right(),
        }
    }

    fn turned_left(&self) -> Direction {
        match &self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }

    fn turned_right(&self) -> Direction {
        match &self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}
