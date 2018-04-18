mod solution_state;
use self::solution_state::SolutionState;

use board::Board;
use std::collections::HashSet;
use {PosCoords, Solutions};

/// This struct is used to find solutions to the problem, given a board state.
pub struct Solver {
    _soln_state: SolutionState,
}

impl Solver {
    /// Create a new solver object.
    pub fn _new(b: Board) -> Solver {
        Solver {
            _soln_state: SolutionState::from(b),
        }
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
