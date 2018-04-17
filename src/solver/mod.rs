use board::Board;
use std::collections::HashSet;
use {PosCoords, Queens, Solutions};

/// This struct is used to find solutions to the problem, given a board state.
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

#[cfg(test)]
mod solver_tests {
    #[test]
    fn default_board_is_not_a_solution() {
        unimplemented!();
    }

    #[test]
    fn invalid_solution_with_8_queens_is_not_accepted() {
        unimplemented!();
    }

    #[test]
    fn valid_solution_is_accepted() {
        unimplemented!();
    }
}
