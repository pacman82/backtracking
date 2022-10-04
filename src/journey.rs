use std::fmt::{Display, Formatter, self};

use crate::position::Position;

// Dimensions of the chessboard
const ROWS: usize = 8;
const COLUMNS: usize = 8;
const NUM_FIELDS: usize = ROWS * COLUMNS;

#[derive(Clone, Copy)]
pub struct Journey {
    /// Number of fields traveled
    num_visited: usize,
    /// Order of moves visited so far. Only meaningful until `num_visited`.
    moves: [Position; NUM_FIELDS]
}

impl Journey {

    pub fn new(start: Position) -> Self {
        Self {
            num_visited: 1,
            moves: [start; NUM_FIELDS]
        }
    }

    pub fn play_move(&mut self, next: Position) {
        self.moves[self.num_visited] = next;
        self.num_visited += 1;
    }

    pub fn is_solution(&self) -> bool {
        self.num_visited == NUM_FIELDS
    }

    pub fn fill_possible_moves(&self, possible_moves: &mut Vec<Position>) {
        let current = self.moves[self.num_visited - 1];
        current.possible_moves(possible_moves);
        possible_moves.retain(|pos| !self.moves[..self.num_visited].contains(pos))
    }
}

impl Display for Journey {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.moves[0])?;
        for m in &self.moves[1..self.num_visited] {
            write!(f, " {m}")?;
        }
        Ok(())
    }
}