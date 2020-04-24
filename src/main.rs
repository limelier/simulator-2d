use std::fmt::{Formatter, Debug};
use std::fmt;

fn main() {
    let mut board = Board::new(3);
    board.set(1, 1, true);
    println!("{:?}", board);
}

enum Direction {
    North,
    South,
    East,
    West,
}

struct Board {
    tiles: Vec<Vec<bool>>
}

impl Board {
    fn new(size: usize) -> Board {
        let tiles = vec![vec![false; size]; size];
        Board { tiles }
    }

    fn set(&mut self, line: usize, col: usize, value: bool) {
        self.tiles[line][col] = value;
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