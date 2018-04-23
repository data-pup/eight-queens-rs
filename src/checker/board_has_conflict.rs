use queen::get_queen_move_sets;
use {Board, CoordSet, PosCoords};

/// Check if the board has any conflicted queens.
pub fn board_has_conflict(board: Board) -> bool {
    let dims: PosCoords = board.dims();
    let queens: CoordSet = board.get_queen_positions();
    let move_sets = get_queen_move_sets(queens.clone(), dims);
    let temp: usize = queens
        .into_iter()
        .map(|pos| {
            let n = move_sets
                .iter()
                .map(|movements| movements.contains(&pos))
                .filter(|&in_space| in_space == true)
                .count();
            n
        })
        .filter(|&num_contesting_space| num_contesting_space != 1)
        .count();
    let has_conflict = temp != 0;
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
