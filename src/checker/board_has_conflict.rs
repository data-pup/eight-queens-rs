use queen::get_contested_spaces;
use queen::get_queen_moves;
use {Board, CoordSet, PosCoords};

/// Check if the board has any conflicted queens.
pub fn board_has_conflict(board: Board) -> bool {
    let board_dims = board.dims();
    let get_moves = |pos: &PosCoords| get_queen_moves(*pos, board_dims);
    let queen_positions: CoordSet = board.get_queen_positions();
    let move_sets = queen_positions
        .iter()
        .map(get_moves)
        .collect::<Vec<CoordSet>>();
    let total_moves: usize = move_sets.iter().map(|moves| moves.len()).sum();
    let distinct_moves: usize = get_contested_spaces(queen_positions, board_dims)
        .into_iter()
        .count();
    let has_conflict = total_moves != distinct_moves;
    has_conflict
}

#[cfg(test)]
mod has_conflict_tests {
    use super::board_has_conflict;
    use Board;

    #[test]
    fn default_board_is_not_a_solution_has_no_conflict() {
        let b = Board::new();
        let res = board_has_conflict(b);
        assert_eq!(res, false, "Empty board has no conflict.");
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
        assert!(board_has_conflict(b));
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
        let res = board_has_conflict(b);
        assert_eq!(res, false);
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
        assert!(board_has_conflict(b));
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
        let res = board_has_conflict(b);
        assert_eq!(res, false);
    }
}
