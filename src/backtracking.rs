use std::vec;

use crate::{journey::Journey, position::Position};

pub struct Solutions {
    possible_moves: Vec<Position>,
    open: Vec<Candidate>,
    current: Journey,
}

impl Solutions {
    pub fn new(start: Position) -> Self {
        Self {
            possible_moves: Vec::new(),
            open: vec![Candidate{ count: 0, mov: start, journey: Journey::new(start)}],
            current: Journey::new(start),
        }
    }
}

struct Candidate {
    count: u32,
    mov: Position,
    journey: Journey,
}

impl Iterator for Solutions {
    type Item = Journey;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(Candidate { count, mov, journey} ) = self.open.pop() {
            if journey.is_solution() {
                return Some(journey);
            }

            journey.fill_possible_moves(&mut self.possible_moves);
            self.open
                .extend(self.possible_moves.iter().map(|&position| {
                    let mut new = journey;
                    new.play_move(position);
                    Candidate { count: count + 1, mov: position, journey: new }
                }))
        }
        None
    }
}
