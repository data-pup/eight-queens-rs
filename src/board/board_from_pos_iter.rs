use super::Board;
use position_types::*;
use std::iter::FromIterator;
use Queens;

impl FromIterator<PosCoords> for Board {
    fn from_iter<I: IntoIterator<Item = PosCoords>>(positions: I) -> Board {
        let mut board = Board::new();
        for (col, row) in positions {
            board.add_queen(row, col);
        }
        board
    }
}

#[cfg(test)]
mod board_from_pos_iter_tests {
    use super::Board;
    use position_types::*;
    use std::collections::HashSet;
    use Queens;

    #[test]
    fn create_default_board() {
        let positions: Vec<PosCoords> = vec![];
        let b: Board = positions.into_iter().collect();
        let results = b.get_queen_positions();
        let expected: HashSet<PosCoords> = HashSet::new();
        assert_eq!(results, expected);
        assert_eq!(b.height, 8);
        assert_eq!(b.width, 8);
    }

    #[test]
    fn create_board_with_one_queen_at_origin() {
        let b: Board = [(0, 0), (0, 1)].iter().cloned().collect();
        let expected: HashSet<PosCoords> = [(0, 0), (0, 1)].iter().cloned().collect();
        assert_eq!(b.get_queen_positions(), expected);
    }
}
