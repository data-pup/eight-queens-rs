mod board_has_conflict;
mod get_queen_states;
mod queen_state;

use self::board_has_conflict::has_contested_queens;
use self::get_queen_states::get_queen_states;
use self::queen_state::QueenState;

use Board;

/// Represents a given solution state.
#[derive(Clone, Debug, PartialEq)]
pub struct SolutionState {
    pub is_solved: bool,
    pub has_conflict: bool,
    pub num_queens: u8,
}

/// Create a SolutionState object using a Board state.
impl From<Board> for SolutionState {
    fn from(b: Board) -> SolutionState {
        let queen_states = get_queen_states(&b);
        let num_queens = queen_states.len() as u8;
        if has_contested_queens(&queen_states) {
            SolutionState {
                has_conflict: true,
                is_solved: false,
                num_queens,
            }
        } else {
            let is_solved = match num_queens {
                n if n < 8 => false,
                n if n == 8 => true,
                _ => panic!("More than eight queens on board"),
            };
            SolutionState {
                has_conflict: false,
                is_solved,
                num_queens,
            }
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

    ///   01234567
    ///   --------
    /// 7|        |
    /// 6|        |
    /// 5|        |
    /// 4|        |
    /// 3|        |
    /// 2|        |
    /// 1|        |
    /// 0|QQ      |
    ///   --------
    #[test]
    fn board_with_2_adjacent_queens_has_conflict() {
        let b: Board = [(0, 0), (1, 0)].iter().cloned().collect();
        let sol = SolutionState::from(b);
        assert_eq!(sol.has_conflict, true);
        assert_eq!(sol.is_solved, false);
    }

    ///   01234567
    ///   --------
    /// 7|x x    x|
    /// 6|x  x  xx|
    /// 5|x   xx x|
    /// 4|x   xx x|
    /// 3|x  x  xx|
    /// 2|xxxxxxxQ|
    /// 1|xx    xx|
    /// 0|Qxxxxxxx|
    ///   --------
    #[test]
    fn board_with_two_safe_queens_has_no_conflict() {
        let b: Board = [(0, 0), (7, 2)].iter().cloned().collect();
        let sol = SolutionState::from(b);
        assert_eq!(sol.has_conflict, false);
        assert_eq!(sol.is_solved, false);
    }

    ///   01234567
    ///   --------
    /// 7|     Q  |
    /// 6|     Q  |
    /// 5|      Q |
    /// 4|Q       |
    /// 3|       Q|
    /// 2| Q      |
    /// 1|    Q   |
    /// 0|  Q     |
    ///   --------
    #[test]
    fn invalid_solution_with_8_queens_is_not_accepted() {
        let b: Board = [
            (2, 0),
            (4, 1),
            (1, 2),
            (7, 3),
            (0, 4),
            (6, 5),
            (5, 6),
            (5, 7),
        ].iter()
            .cloned()
            .collect();
        let sol = SolutionState::from(b);
        assert_eq!(sol.has_conflict, true);
        assert_eq!(sol.is_solved, false);
    }

    ///   01234567
    ///   --------
    /// 7|     Q  |
    /// 6|   Q    |
    /// 5|      Q |
    /// 4|Q       |
    /// 3|       Q|
    /// 2| Q      |
    /// 1|    Q   |
    /// 0|  Q     |
    ///   --------
    #[test]
    fn valid_solution_is_accepted() {
        let b: Board = [
            (2, 0),
            (4, 1),
            (1, 2),
            (7, 3),
            (0, 4),
            (6, 5),
            (3, 6),
            (5, 7),
        ].iter()
            .cloned()
            .collect();
        let sol = SolutionState::from(b);
        assert_eq!(sol.has_conflict, false);
        assert_eq!(sol.is_solved, true);
    }
}
