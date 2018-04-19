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
    use std::iter::empty;
    use Queens;

    #[test]
    fn create_board_from_empty_iter() {
        let b: Board = empty().into_iter().collect();
        let results = b.get_queen_positions();
        let expected: HashSet<PosCoords> = HashSet::new();
        assert_eq!(results, expected);
        assert_eq!(b.height, 8);
        assert_eq!(b.width, 8);
    }

    #[test]
    fn create_board_with_two_queens() {
        let b: Board = [(0, 0), (0, 1)].iter().cloned().collect();
        let expected: HashSet<PosCoords> = [(0, 0), (0, 1)].iter().cloned().collect();
        assert_eq!(b.get_queen_positions(), expected);
    }
}

#[cfg(test)]
mod board_from_pos_iter_benches {
    #![feature(test)]
    extern crate test;
    use self::test::{Bencher, black_box};
    use std::iter::empty;
    use super::Board;

    #[bench]
    fn time_create_board_from_empty_iter(bencher: &mut Bencher) {
        bencher.iter(|| {
            let _: Board = empty().into_iter().collect();
        });
    }

    #[bench]
    fn time_creating_board_with_two_queens_from_iter(bencher: &mut Bencher) {
        bencher.iter(|| {
            let _: Board = [(0, 0), (0, 1)].iter().cloned().collect();
        });
    }
}
