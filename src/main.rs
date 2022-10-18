use backtracking::Solutions;
use journey::Journey;
use position::Position;

mod backtracking;
mod board;
mod journey;
mod position;

fn main() {
    let start = Position::new(0, 0);
    let init = Journey::new(start);
    let solutions = Solutions::new(init);

    for (index, journey) in solutions.enumerate().take(10) {
        let num_solution = index + 1;
        println!("{num_solution}: {journey}")
    }
}

#[cfg(test)]
mod tests {
    use crate::{backtracking::Game, journey::Journey, position::Position};

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
