use std::fmt::{self, Display, Formatter};

use backtracking::{Problem, Solutions};

fn main() {
    let board = NQueens::new(8);
    for solution in Solutions::new(board) {
        println!("{solution}")
    }
}

#[derive(Clone)]
struct NQueens {
    n: u32,
    /// Position of queens in each row
    queens: Vec<QueenAt>,
}

impl NQueens {
    fn new(n: u32) -> Self {
        Self {
            n,
            queens: Vec::new(),
        }
    }
}

impl Display for NQueens {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut queens = self.queens.clone();
        queens.sort_by_key(|q| q.row);

        let repeat_point = |f: &mut Formatter, n| {
            for _ in 0..n {
                write!(f, ".")?;
            }
            Ok(())
        };

        for queen in queens {
            repeat_point(f, queen.column)?;
            write!(f, "Q")?;
            repeat_point(f, self.n - queen.column - 1)?;
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Clone, Copy)]
struct QueenAt {
    row: u32,
    column: u32,
}

impl QueenAt {
    /// True if the two queens are not allowed at the board at the same time.
    fn conflicts(self, other: QueenAt) -> bool {
        self.row == other.row
            || self.column == other.column
            || self.row.abs_diff(other.row) == self.column.abs_diff(other.column)
    }
}

impl Problem for NQueens {
    type Posibility = QueenAt;
    type Solution = NQueens;

    fn next_decisions(&self, possible_moves: &mut Vec<QueenAt>) {
        if self.queens.len() == self.n as usize {
            return;
        }
        // Give all possible position for the top empty row
        let possibilities = (0..self.n).map(|col| QueenAt {
            row: self.queens.len() as u32,
            column: col,
        }).filter(|candidate| self.queens.iter().all(|q| !q.conflicts(*candidate)));
        possible_moves.extend(possibilities);
    }

    fn undo(&mut self, _last: &Self::Posibility, _history: &[Self::Posibility]) {
        self.queens.pop();
    }

    fn decide(&mut self, next: QueenAt) {
        self.queens.push(next);
    }

    fn is_solution(&self, _history: &[Self::Posibility]) -> Option<Self::Solution> {
        if self.queens.len() == self.n as usize {
            Some(self.clone())
        } else {
            None
        }
    }
}
