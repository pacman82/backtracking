use std::fmt::{self, Display, Formatter};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct Position {
    row: i8,
    column: i8,
}

impl Position {
    pub fn from_index(index: usize) -> Self {
        Self {
            row: index as i8 / 8,
            column: index as i8 % 8,
        }
    }

    pub fn possible_moves(&self, possible_moves: &mut Vec<Position>) {
        let moves = [
            (-2, -1),
            (-2, 1),
            (-1, -2),
            (-1, 2),
            (1, -2),
            (1, 2),
            (2, -1),
            (2, 1),
        ];
        possible_moves.clear()
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let letter = (self.row as u8 + b'A') as char;
        let digit = (self.column as u8) + 1;
        write!(f, "{letter}{digit}")
    }
}

pub struct Journey {
    /// Stores the index of the field from which it had been visited or `OPEN` in order to
    /// signal that this field has not been visited yet. A self referencing field resembles the
    /// starting position.
    visited: [usize; 8 * 8],
    /// Current position of the knight
    current: Position,
}

impl Journey {
    const OPEN: usize = 8 * 8;

    pub fn new() -> Self {
        let mut visited = [Self::OPEN; 8 * 8];
        visited[0] = 0;
        Self {
            visited,
            current: Position::from_index(0),
        }
    }

    pub fn knights_position(&self) -> Position {
        self.current
    }

    pub fn fill_possible_moves(&self, possible: &mut Vec<Position>) {
        self.current.possible_moves(possible)
    }
}

impl Default for Journey {
    fn default() -> Self {
        Self::new()
    }
}

fn main() {
    let journey = Journey::new();
    println!("Starting position is: {}", journey.knights_position());
}

#[cfg(test)]
mod tests {
    use crate::{Journey, Position};

    #[test]
    fn print_position() {
        assert_eq!("A1", Position::from_index(0).to_string());
    }

    #[test]
    fn start_in_upper_right_corner() {
        let journey = Journey::new();
        assert_eq!(Position::from_index(0), journey.knights_position());
    }

    #[test]
    fn fill_possible_moves() {
        let journey = Journey::new();
        let mut possible = Vec::new();
        journey.fill_possible_moves(&mut possible);
        // assert_eq!([Position(10), Position(17)].as_slice(), &possible)
    }
}
