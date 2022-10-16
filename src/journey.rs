use std::fmt::{self, Display, Formatter};

use crate::position::Position;

// Dimensions of the chessboard
const ROWS: usize = 8;
const COLUMNS: usize = 8;
const NUM_FIELDS: usize = ROWS * COLUMNS;

#[derive(Clone, Copy, Debug)]
pub struct Journey {
    /// Number of fields traveled
    num_visited: usize,
    /// For fast lookup, wether a position has been visited or not.
    visited: [bool; NUM_FIELDS],
    /// Order of moves visited so far. Only meaningful until `num_visited`.
    moves: [Position; NUM_FIELDS],
}

impl Journey {
    pub fn new(start: Position) -> Self {
        let mut visited = [false; NUM_FIELDS];
        visited[start.as_index()] = true;
        Self {
            num_visited: 1,
            visited,
            moves: [start; NUM_FIELDS],
        }
    }

    pub fn play_move(&mut self, next: Position) {
        self.moves[self.num_visited] = next;
        self.visited[next.as_index()] = true;
        self.num_visited += 1;
    }

    pub fn undo(&mut self) {
        self.num_visited -= 1;
        self.visited[self.moves[self.num_visited].as_index()] = false;
    }

    pub fn is_solution(&self) -> bool {
        self.num_visited == NUM_FIELDS
    }

    pub fn fill_possible_moves(&self, possible_moves: &mut Vec<Position>) {
        let current = self.moves[self.num_visited - 1];
        current.reachable_fields(possible_moves);
        possible_moves.retain(|pos| !self.visited[pos.as_index()])
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
