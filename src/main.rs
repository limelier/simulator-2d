use std::fmt::{Formatter, Debug};
use std::fmt;

fn main() {
    let instructions = vec![
        Instruction::Place,
        Instruction::Move,
        Instruction::Place,
        Instruction::Move,
        Instruction::Turn(TurnDirection::Right),
        Instruction::Move,
        Instruction::Place,
        Instruction::Move,
        Instruction::Place
    ];

    let mut simulation = Simulation::new(3, &instructions);
    simulation.run();

    println!("{:?}", simulation.board);
}

#[derive(Copy, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn turned(&self, direction: &TurnDirection) -> Direction {
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

enum TurnDirection {
    Left,
    Right,
}

enum Instruction {
    Turn(TurnDirection),
    Place,
    Move,
}

#[derive(Clone)]
struct Board {
    tiles: Vec<Vec<bool>>
}

impl Board {
    /// # Panics
    /// Will panic if size is 0.
    fn new(size: usize) -> Board {
        if size == 0 {
            panic!("Cannot create board of size 0.");
        }

        let tiles = vec![vec![false; size]; size];
        Board { tiles }
    }

    /// If the position is in the board, set it to `true`.
    fn set(&mut self, pos: Position, value: bool) {
        let Position { row, col } = pos;
        if row < self.size() && col < self.size() {
            self.tiles[row][col] = value;
        }
    }

    fn size(&self) -> usize {
        self.tiles.len()
    }

    fn in_bounds(&self, pos: &Position) -> bool {
        let size = self.size();
        pos.col < size && pos.row <= size
    }
}

impl ToString for Board {
    fn to_string(&self) -> String {
        let strings: Vec<String> = self.tiles.iter().map(|row| -> String {
            let strings: Vec<&str> = row.iter().map(|val| {
                match val {
                    false => "-",
                    true => "#",
                }
            }
            ).collect();
            strings.join("")
        }
        ).collect();
        strings.join("\n")
    }
}

impl Debug for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Copy, Clone)]
struct Position {
    row: usize,
    col: usize,
}

impl Position {
    fn new(row: usize, col: usize) -> Position {
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

    fn shifted(&self, direction: Direction) -> Option<Position> {
        match direction {
            Direction::North => Position::from(&self, -1, 0),
            Direction::East => Position::from(&self, 0, 1),
            Direction::South => Position::from(&self, 1, 0),
            Direction::West => Position::from(&self, 0, -1),
        }
    }
}

struct Robot {
    direction: Direction,
    position: Position,
}

impl Robot {
    fn new() -> Robot {
        Robot {
            direction: Direction::East,
            position: Position::new(0, 0),
        }
    }
}

struct Simulation<'a> {
    board: Board,
    robot: Robot,
    instructions: &'a Vec<Instruction>,
}

impl Simulation<'_> {
    fn new(size: usize, instructions: &Vec<Instruction>) -> Simulation {
        let board = Board::new(size);
        let robot = Robot::new();

        Simulation { board, robot, instructions }
    }

    fn run(&mut self) {
        for instruction in self.instructions {
            self.run_instruction(instruction);
        }
    }

    fn run_instruction(&mut self, instruction: &Instruction) {
        let robot = &mut self.robot;
        match instruction {
            Instruction::Move => {
                let new_pos = robot.position.shifted(robot.direction);
                if let Some(new_pos) = new_pos {
                    robot.position = new_pos;
                }
            }
            Instruction::Turn(dir) => {
                robot.direction = robot.direction.turned(dir);
            }
            Instruction::Place => {
                self.board.set(robot.position, true)
            }
        }
    }
}