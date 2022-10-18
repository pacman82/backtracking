use std::fmt::{self, Display, Formatter};

use crate::{
    backtracking::Game,
    board::{Board, NUM_FIELDS},
    position::Position,
};

#[derive(Clone, Debug)]
pub struct Journey {
    board: Board,
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
            board: Board::new(),
            num_visited: 1,
            visited,
            moves: [start; NUM_FIELDS],
        }
    }
}

impl Display for Solution {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0[0])?;
        for m in &self.0[1..NUM_FIELDS] {
            write!(f, " {m}")?;
        }
        Ok(())
    }
}

impl Game for Journey {
    type Move = Position;
    type Solution = Solution;

    fn fill_possible_moves(&self, possible_moves: &mut Vec<Position>) {
        let current = self.moves[self.num_visited - 1];
        self.board.reachable_fields(current, possible_moves);
        possible_moves.retain(|pos| !self.visited[pos.as_index()])
    }

    fn undo(&mut self) {
        self.num_visited -= 1;
        self.visited[self.moves[self.num_visited].as_index()] = false;
    }

    fn play_move(&mut self, next: Position) {
        self.moves[self.num_visited] = next;
        self.visited[next.as_index()] = true;
        self.num_visited += 1;
    }

    fn is_solution(&self) -> Option<Solution> {
        if self.num_visited == NUM_FIELDS {
            Some(Solution(self.moves))
        } else {
            None
        }
    }
}

pub struct Solution([Position; NUM_FIELDS]);
