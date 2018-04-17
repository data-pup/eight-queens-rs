use board::Board;
use std::collections::HashSet;
use {PosCoords, Queens, Solutions, UncontestedSpaces};

/// This struct is used to find solutions to the problem, given a board state.
pub struct Solver {
    pub is_solved: bool,
    pub has_conflict: bool,
}

impl Solver {
    /// Create a new solver object.
    pub fn _new(_board: &Board) -> Solver {
        match Solver::has_contested_queens(_board) {
            true => Solver {
                has_conflict: true,
                is_solved: false,
            },
            false => Solver {
                has_conflict: false,
                is_solved: Solver::check_is_solved(_board),
            },
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

    /// This helper method will check whether the board has any contested
    /// queens, that could move to one another's space.
    fn has_contested_queens(b: &Board) -> bool {
        let q_positions = b.get_queen_positions();
        let uncontested = b.get_uncontested_spaces();
        match q_positions.is_empty() {
            true => false,
            false => q_positions
                .iter()
                .map(|p| uncontested.contains(p))
                .fold(true, |result, is_uncontested| result && is_uncontested),
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

#[cfg(test)]
mod solver_tests {
    use super::Solver;
    use Board;

    #[test]
    fn default_board_is_not_a_solution_has_no_conflict() {
        let b = Board::new();
        let s = Solver::_new(&b);
        assert_eq!(s.has_conflict, false, "Empty board has no conflict.");
        assert_eq!(s.is_solved, false);
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
