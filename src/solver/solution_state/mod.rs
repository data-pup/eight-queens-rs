mod board_has_conflict;
mod board_is_solved;

use self::board_has_conflict::has_contested_queens;
use self::board_is_solved::check_is_solved;

use std::collections::HashSet;

use {Board, PosCoords, Queens};

/// Represents a given solution state.
#[derive(Debug, PartialEq)]
pub struct SolutionState {
    pub is_solved: bool,
    pub has_conflict: bool,
    pub num_queens: u8,
    pub uncontested: HashSet<PosCoords>,
}

impl From<Board> for SolutionState {
    fn from(b: Board) -> SolutionState {
        let queen_positions = b.get_queen_positions();
        let num_queens = queen_positions.len() as u8;
        let uncontested = b.get_uncontested_spaces();
        match has_contested_queens(&queen_positions, &uncontested) {
            true => SolutionState {
                has_conflict: true,
                is_solved: false,
                num_queens,
                uncontested,
            },
            false => SolutionState {
                has_conflict: false,
                is_solved: check_is_solved(&b),
                num_queens,
                uncontested,
            },
        }
    }
}

#[cfg(test)]
mod solution_state_tests {
    use super::SolutionState;
    use Board;

    #[test]
    fn default_board_is_not_a_solution_has_no_conflict() {
        let b = Board::new();
        let sol = SolutionState::from(b);
        assert_eq!(sol.has_conflict, false, "Empty board has no conflict.");
        assert_eq!(sol.is_solved, false);
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
