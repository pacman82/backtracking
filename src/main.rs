use backtracking::Solutions;
use position::Position;

mod position;
mod journey;
mod backtracking;


fn main() {
    let solutions = Solutions::new(Position::new(0,0));
    
    for journey in solutions {
        println!("{journey}")
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
            [Position::new(1,2), Position::new(2,1)].as_slice(),
            &possible
        )
    }
}
