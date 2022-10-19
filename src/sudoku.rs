use std::io::{self, Write};

use crate::backtracking::Game;

pub struct Sudoku {
    /// All 9 by 9 fields, in top to bottom, left to right order. `0` represents empty. Other valid
    /// values are 1..=9
    fields: [u8; 9*9],
}

impl Sudoku {
    pub fn new() -> Self {
        let fields = [0u8; 9*9];
        Self::from_bytes(fields)
    }

    pub fn from_bytes(bytes: [u8; 9*9]) -> Self {
        if bytes.iter().any(|&n| n > 9) {
            panic!("Only values from 0 to 9 are valid.")
        }
        Self { fields: bytes }
    }

    pub fn from_slice(bytes: &[u8]) -> Self {
        Self::from_bytes(bytes.try_into().unwrap())
    }

    pub fn print_to(&self, to: &mut impl Write) -> io::Result<()> {
        for index in 0..self.fields.len() {
            // New row beginnig?
            if index % 9 == 0 && index != 0 {
                writeln!(to)?;
            }
            match self.fields[index] {
                0 => write!(to, "X")?,
                n @ 1..=9 => write!(to, "{n}")?,
                _ => unreachable!()
            };
        }
        writeln!(to)?;
        Ok(())
    }
}

impl Default for Sudoku {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Copy)]
pub struct WriteDigit {
    index: u8,
    digit: u8,
}

impl Game for Sudoku {
    type Move = WriteDigit;

    type Solution = Sudoku;

    fn fill_possible_moves(&self, possible_moves: &mut Vec<Self::Move>) {
        todo!()
    }

    fn undo(&mut self) {
        todo!()
    }

    fn play_move(&mut self, next: Self::Move) {
        todo!()
    }

    fn is_solution(&self) -> Option<Self::Solution> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::Sudoku;

    #[test]
    fn print_empty_sudoku() {
        let mut out = Vec::new();
        let game = Sudoku::new();

        game.print_to(&mut out).unwrap();


        let expect =
            "XXXXXXXXX\n\
            XXXXXXXXX\n\
            XXXXXXXXX\n\
            XXXXXXXXX\n\
            XXXXXXXXX\n\
            XXXXXXXXX\n\
            XXXXXXXXX\n\
            XXXXXXXXX\n\
            XXXXXXXXX\n\
        ";
        assert_eq!(expect, std::str::from_utf8(&out).unwrap());
    }
}