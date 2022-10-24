/// A problem to be tackled with backtracking. Used by the [`Solutions`] iterator which can find
/// solutions for ypes implementing [`Problem`].
pub trait Problem {
    type Decision: Copy;
    type Solution;

    fn next_decisions(&self, possible_moves: &mut Vec<Self::Decision>);
    /// Undo the last decision made. If invoked by the [`Solutions`] iterator `last` is to be
    /// guaranteed, to be the last decision made with [`do`]
    fn undo(&mut self, last: &Self::Decision, history: &[Self::Decision]);
    fn play_move(&mut self, next: Self::Decision);
    fn is_solution(&self, history: &[Self::Decision]) -> Option<Self::Solution>;
}

/// An iterator performing backtracking to find solutions to a problem.
pub struct Solutions<G: Problem> {
    decisions: Vec<G::Decision>,
    open: Vec<Candidate<G::Decision>>,
    /// Keeps track of the decisions, which yielded the current problem state, starting from the
    /// initial state.
    history: Vec<G::Decision>,
    current: G,
}

impl<G: Problem> Solutions<G> {
    pub fn new(init: G) -> Self {
        let mut possible_moves = Vec::new();
        init.next_decisions(&mut possible_moves);
        let open = possible_moves
            .iter()
            .map(|pos| Candidate {
                count: 1,
                mov: *pos,
            })
            .collect();
        Self {
            decisions: possible_moves,
            open,
            history: Vec::new(),
            current: init,
        }
    }
}

struct Candidate<M> {
    /// Counts the number of turns made to get to this candidate. We keep track of this so we can
    /// call undo the appropriate number of types, if we roll back a solution.
    count: i32,
    mov: M,
}

impl<G: Problem> Iterator for Solutions<G> {
    type Item = G::Solution;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(Candidate { count, mov }) = self.open.pop() {
            // Unroll all the moves until our current state is identical with the one which we
            // had once we put that mov into the open list. We want to be one move behind so
            // we need to play the move in order to get the desired state
            for _ in 0..self.history.len() as i32 - count + 1 {
                let last = self.history.pop().unwrap();
                self.current.undo(&last, &self.history);
            }

            // We advance one move deeper into the search tree
            self.current.play_move(mov);
            self.history.push(mov);

            // Emit solution
            if let Some(solution) = self.current.is_solution(&self.history) {
                return Some(solution);
            }

            // Extend search tree
            self.current.next_decisions(&mut self.decisions);
            self.open
                .extend(self.decisions.iter().map(|&position| Candidate {
                    count: count + 1,
                    mov: position,
                }))
        }
        None
    }
}
