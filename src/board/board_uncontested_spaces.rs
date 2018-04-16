use std::collections::HashSet;

use UncontestedSpaces;
use position_types::*;
use super::Board;

impl UncontestedSpaces for Board {
    fn get_uncontested_spaces(&self) -> HashSet<PosCoords> {
        unimplemented!();
    }
}

#[cfg(test)]
mod board_uncontested_spaces_tests {
    use std::collections::HashSet;
    use {UncontestedSpaces, Queens};
    use position_types::*;
    use super::Board;

    /// This function tests uncontested spaces for the following state:
    /// NOTE: Q's are queens, x's are contested spaces.
    ///   01234567
    ///   --------
    /// 7|QxxxxxxQ|
    /// 6|xx    xx|
    /// 5|x x  x x|
    /// 4|x  xx  x|
    /// 3|x  xx  x|
    /// 2|x x  x x|
    /// 1|xx    xx|
    /// 0|QxxxxxxQ|
    ///   --------
    #[test]
    fn queens_in_corners() {
        let mut b = Board::new();
        for curr in [(0, 0), (0, 7), (7, 0), (7, 7)].iter() {
            let &(col, row) = curr;
            b.add_queen(row, col);
        }
        let expected: HashSet<PosCoords> = [
            (2, 6), (3, 6), (4, 6), (5, 6),
            (1, 5), (3, 5), (4, 5), (6, 5),
            (1, 4), (2, 4), (5, 4), (6, 4),
            (1, 3), (2, 3), (5, 3), (6, 3),
            (1, 2), (3, 2), (4, 2), (6, 2),
            (2, 1), (3, 1), (4, 1), (5, 1),
        ].iter().cloned().collect();
        let results = b.get_uncontested_spaces();
        assert_eq!(results, expected);
    }
}
