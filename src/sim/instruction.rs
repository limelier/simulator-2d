use crate::board::TurnDirection;

pub enum Instruction {
    Turn(TurnDirection),
    Place,
    Move,
}

