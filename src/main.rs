mod backtracking;
mod knights_journey;
mod sudoku;

use std::io::stdout;

use backtracking::Solutions;
use knights_journey::{Journey, Position};

fn main() {
    let start = Position::new(0, 0);
    let init = Journey::new(start);
    let solutions = Solutions::new(init);

    for (index, journey) in solutions.enumerate().take(1) {
        let num_solution = index + 1;
        println!("{num_solution}: {journey}")
    }

    let start = sudoku::Sudoku::new();
    let solutions = Solutions::new(start);

    for (index, journey) in solutions.enumerate().take(1) {
        let num_solution = index + 1;
        println!("{num_solution}:");
        journey.print_to(&mut stdout()).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        backtracking::Game,
        knights_journey::{Journey, Position},
    };

    #[test]
    fn fill_possible_moves() {
        let journey = Journey::new(Position::new(0, 0));

        let mut possible = Vec::new();
        journey.fill_possible_moves(&mut possible);
        assert_eq!(
            [Position::new(2, 1), Position::new(1, 2)].as_slice(),
            &possible
        )
    }
}
