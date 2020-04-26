use crate::sim::instruction::Instruction;
use crate::board::Board;
use crate::sim::robot::Robot;

pub struct Simulation<'a> {
    pub board: Board,
    robot: Robot,
    instructions: &'a Vec<Instruction>,
}

impl Simulation<'_> {
    /// Creates a new simulation, with a blank board of the given size, a robot in the top-left
    /// corner facing east, and the given list of instructions.
    pub fn new(size: usize, instructions: &Vec<Instruction>) -> Simulation {
        let board = Board::new(size);
        let robot = Robot::new();

        Simulation { board, robot, instructions }
    }

    pub fn run(&mut self) {
        for instruction in self.instructions {
            self.run_instruction(instruction);
        }
    }

    pub fn run_instruction(&mut self, instruction: &Instruction) {
        let robot = &mut self.robot;
        match instruction {
            Instruction::Move => {
                let new_pos = robot.position.shifted(robot.direction);
                if let Some(new_pos) = new_pos {
                    if self.board.in_bounds(&new_pos) {
                        robot.position = new_pos;
                    }
                }
            }
            Instruction::Turn(dir) => {
                robot.direction = robot.direction.turned(dir);
            }
            Instruction::Place => {
                self.board.set(robot.position, true);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::board::{Position, Direction, TurnDirection};

    #[test]
    fn init_works_as_expected() {
        let instructions = vec![];
        let sim = Simulation::new(5, &instructions);
        assert_eq!(5, sim.board.size());
        assert_eq!(Position::new(0, 0), sim.robot.position);
        assert_eq!(Direction::East, sim.robot.direction);
    }

    #[test]
    fn move_instruction_executes() {
        let instructions = vec![
            Instruction::Move
        ];
        let mut sim = Simulation::new(2, &instructions);
        sim.run();
        assert_eq!(Position::new(0, 1), sim.robot.position);
    }

    #[test]
    fn move_instruction_respects_bounds() {
        let instructions = vec![
            Instruction::Move,
            Instruction::Move,
            Instruction::Turn(TurnDirection::Left),
            Instruction::Move,
        ];
        let mut sim = Simulation::new(2, &instructions);
        sim.run();
        assert_eq!(Position::new(0, 1), sim.robot.position);
    }

    #[test]
    fn turn_instruction_executes() {
        let instructions = vec![
            Instruction::Turn(TurnDirection::Right)
        ];
        let mut sim = Simulation::new(1, &instructions);
        sim.run();
        assert_eq!(Direction::South, sim.robot.direction);
    }

    #[test]
    fn place_instruction_executes() {
        let instructions = vec![
            Instruction::Place,
        ];
        let mut sim = Simulation::new(1, &instructions);
        sim.run();
        assert_eq!(true, sim.board.check(0, 0));
    }
}