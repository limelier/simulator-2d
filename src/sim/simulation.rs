use crate::sim::instruction::Instruction;
use crate::sim::board::Board;
use crate::sim::robot::Robot;

pub struct Simulation<'a> {
    pub board: Board,
    robot: Robot,
    instructions: &'a Vec<Instruction>,
}

impl Simulation<'_> {
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