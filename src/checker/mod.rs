mod board_has_conflict;
mod check_result;

pub use self::check_result::CheckResult;
use self::board_has_conflict::board_has_conflict;

use Board;

/// Check the state of the board. Returns a `CheckResult` object, containing
/// information about whether the given positions contain a conflict, whether
/// the given positions represent a solution to the eight problem, etc.
pub fn check_board(board: Board) -> CheckResult {
    let num_queens = board.get_queen_positions().len() as u8;
    let has_conflict = board_has_conflict(board);
    let is_solved = num_queens == 8 && !has_conflict;
    CheckResult {
        is_solved,
        has_conflict,
        num_queens,
    }
}

#[cfg(test)]
mod check_result_tests {
    use super::check_board;
    use super::CheckResult;
    use Board;

    #[test]
    fn default_board_is_not_a_solution_has_no_conflict() {
        let b = Board::new();
        let sol = check_board(b);
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
        let sol = check_board(b);
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
        let sol = check_board(b);
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
        let sol = check_board(b);
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
        let sol = check_board(b);
        assert_eq!(sol.has_conflict, false);
        assert_eq!(sol.is_solved, true);
    }
}
