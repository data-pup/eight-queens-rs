use board::Board;
use std::collections::HashSet;
use {PosCoords, Queens, Solutions};

pub struct Solver {
    _board: Board,
    _is_solved: bool,
}

impl Solver {
    /// Create a new solver object.
    fn _new(_board: Board) -> Solver {
        unimplemented!();
    }

    /// Check if the board is currently a solution to the 8 queens problem.
    fn _check_is_solved(b: &Board) -> bool {
        let q_positions = b.get_queen_positions();
        if q_positions.len() < 8 {
            return false;
        } else if q_positions.len() > 8 {
            panic!("More than 8 queens are on the board.")
        }

        for &curr_pos in q_positions.iter() {
            let _moves = b.get_queen_moves(curr_pos);
            unimplemented!(); // Check if any of the positions contain a queen.
        }

        unimplemented!(); // Fixup.
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
