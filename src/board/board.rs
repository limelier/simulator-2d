use std::fmt::{Debug, Formatter};
use std::fmt;
use crate::board::Position;

#[derive(Clone)]
pub struct Board {
    tiles: Vec<Vec<bool>>
}

impl Board {
    /// # Panics
    /// Will panic if size is 0.
    pub(crate) fn new(size: usize) -> Board {
        if size == 0 {
            panic!("Cannot create board of size 0.");
        }

        let tiles = vec![vec![false; size]; size];
        Board { tiles }
    }

    /// If the position is in the board, set it to `true`.
    pub(crate) fn set(&mut self, pos: Position, value: bool) {
        let Position { row, col } = pos;
        if row < self.size() && col < self.size() {
            self.tiles[row][col] = value;
        }
    }

    pub(crate) fn size(&self) -> usize {
        self.tiles.len()
    }

    pub(crate) fn in_bounds(&self, pos: &Position) -> bool {
        let size = self.size();
        pos.col < size && pos.row <= size
    }
}

#[cfg(test)]
impl Board {
    pub(crate) fn check(&self, row: usize, col: usize) -> bool {
        self.tiles[row][col]
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
