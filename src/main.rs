use backtracking::Solutions;
use position::Position;

mod backtracking;
mod journey;
mod position;

fn main() {
    let solutions = Solutions::new(Position::new(0, 0));

    for (index, journey) in solutions.enumerate().take(200) {
        let num_solution = index + 1;
        println!("{num_solution}: {journey}")
    }
}

#[cfg(test)]
mod tests {
    use crate::{journey::Journey, position::Position};

    #[test]
    fn fill_possible_moves() {
        let journey = Journey::new(Position::new(0, 0));

        let mut possible = Vec::new();
        journey.fill_possible_moves(&mut possible);
        assert_eq!(
            [Position::new(1, 2), Position::new(2, 1)].as_slice(),
            &possible
        )
    }
}
