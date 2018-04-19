mod solution_state;
use self::solution_state::SolutionState;

use board::Board;
use std::collections::HashSet;
use {PosCoords, Solutions};

/// This struct is used to find solutions to the problem, given a board state.
pub struct Solver {
    _curr_board: Board,
    _soln_state: SolutionState,
    _next_moves: Vec<SolutionState>,
}

impl Solver {
    /// Create a new solver object.
    pub fn _new(b: Board) -> Solver {
        Solver {
            _curr_board: b.clone(),
            _soln_state: SolutionState::from(b),
            _next_moves: vec![],
        }
    }

    /// Create a vector of the next possible moves.
    /// TODO: Return in descending order sorted by number of remaining
    /// uncontested squares on the board?
    fn _get_next_moves(_b: &Board) -> Vec<Board> {
        unimplemented!();
    }
}

/// Implement the Solutions trait for the solver class. This will return a set
/// of solutions.
impl Solutions for Solver {
    /// Calculate solutions to the problem.
    fn get_solutions(&self) -> HashSet<PosCoords> {
        unimplemented!();
    }
}
