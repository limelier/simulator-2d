use crate::sim::turn_direction::TurnDirection;

pub enum Instruction {
    Turn(TurnDirection),
    Place,
    Move,
}

