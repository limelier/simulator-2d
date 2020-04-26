use crate::board::{Board, Position};
use std::{fs, io};

fn board_from_lines(lines: Vec<&str>) -> Board {
    let size = lines[0].len();
    let mut board = Board::new(size);

    for (row, line) in lines.iter().enumerate() {
        for (col, char) in line.as_bytes().iter().enumerate() {
            if *char == b'#' {
                board.set(Position::new(row, col), true);
            }
        }
    }

    board
}

fn lines_from_file(filename: &str) -> io::Result<Vec<String>> {
    Ok(
        fs::read_to_string(filename)?
            .lines()
            .map(|str| str.to_string() )
            .collect()
    )
}

fn board_from_file(filename: &str) -> io::Result<Board> {
    let lines = lines_from_file(&filename)?;

    Ok (
        board_from_lines(lines.iter().map(|x| x.as_str()).collect())
    )
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::board::{Board, Position};

    #[test]
    fn board_from_str_works() {
        let mut expected_board = Board::new(3);
        expected_board.set(Position::new(0, 0), true);
        expected_board.set(Position::new(0, 2), true);
        expected_board.set(Position::new(1, 1), true);
        expected_board.set(Position::new(2, 0), true);
        expected_board.set(Position::new(2, 2), true);

        let actual_board = board_from_lines(vec![
            "#-#",
            "-#-",
            "#-#",
        ]);

        assert_eq!(expected_board, actual_board);
    }
}