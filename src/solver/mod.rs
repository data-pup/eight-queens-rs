mod solution_state;
use self::solution_state::SolutionState;

use board::Board;
use std::collections::HashSet;
use {Queens, PosCoords, Solutions};

/// This struct is used to find solutions to the problem, given a board state.
pub struct Solver {
    _curr_board: Board,
    _soln_state: SolutionState,
}

impl Solver {
    /// Create a new solver object.
    pub fn _new(b: Board) -> Solver {
        Solver {
            _curr_board: b.clone(),
            _soln_state: SolutionState::from(b),
        }
    }

    /// Create a vector of the next possible moves.
    /// TODO: Return in descending order sorted by number of remaining
    /// uncontested squares on the board?
    fn _get_next_moves(b: &Board) -> Vec<Solver> {
        let uncontested = b.get_uncontested_spaces();
        uncontested.iter()
            .map(|&(col, row)| {
                let mut next_state = b.clone();
                next_state.add_queen(row, col);
                next_state
            })
            .map(Solver::_new)
            .collect::<Vec<Solver>>()
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
