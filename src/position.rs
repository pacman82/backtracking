use std::fmt::{self, Display, Formatter};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct Position {
    row: i8,
    column: i8,
}

impl Position {

    pub fn new(row: i8, column: i8) -> Self {
        Self {
            row, column
        }
    }

    /// All possible moves, taking into account the position in the board
    pub fn possible_moves(&self, possible_moves: &mut Vec<Position>) {
        // Possible Moves of the knight
        const MOVES: [(i8, i8); 8] = [
            (-2, -1),
            (-2, 1),
            (-1, -2),
            (-1, 2),
            (1, -2),
            (1, 2),
            (2, -1),
            (2, 1),
        ];

        possible_moves.clear();
        possible_moves.extend(
            MOVES
                .iter()
                .map(|(delta_row, delta_column)| (self.row - delta_row, self.column - delta_column))
                .filter(|&(r, c)| (0..8).contains(&r) && (0..8).contains(&c))
                .map(|(row, column)| Position { row, column }),
        );
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let letter = (self.row as u8 + b'A') as char;
        let digit = (self.column as u8) + 1;
        write!(f, "{letter}{digit}")
    }
}

#[cfg(test)]
mod tests {
    use super::Position;

    #[test]
    fn print_position() {
        assert_eq!("A1", Position::new(0, 0).to_string());
    }
}