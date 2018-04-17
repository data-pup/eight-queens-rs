use board::Board;
use std::collections::HashSet;
use {PosCoords, Queens, Solutions, UncontestedSpaces};

/// This struct is used to find solutions to the problem, given a board state.
pub struct Solver {
    is_solved: bool,
    all_queens_safe: bool,
}

impl Solver {
    /// Create a new solver object.
    pub fn _new(_board: &Board) -> Solver {
        let is_solved = Solver::check_is_solved(_board);
        let all_queens_safe = Solver::has_no_contested_queens(_board);
        Solver {
            is_solved,
            all_queens_safe,
        }
    }

    /// Check if the board is currently a solution to the 8 queens problem.
    fn check_is_solved(b: &Board) -> bool {
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

    fn has_no_contested_queens(b: &Board) -> bool {
        let q_positions = b.get_queen_positions();
        let uncontested = b.get_uncontested_spaces();
        q_positions
            .iter()
            .map(|p| uncontested.contains(p))
            .fold(true, |res, curr| res && curr)
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
    use super::Board;

    #[test]
    fn default_board_is_not_a_solution() {
        unimplemented!()
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
