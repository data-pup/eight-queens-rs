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
    use std::iter::empty;
    use {CoordSet, Queens};

    #[test]
    fn create_board_from_empty_iter() {
        let b: Board = empty().into_iter().collect();
        let results = b.get_queen_positions();
        let expected: CoordSet = CoordSet::new();
        assert_eq!(results, expected);
        assert_eq!(b.height, 8);
        assert_eq!(b.width, 8);
    }

    #[test]
    fn create_board_with_two_queens() {
        let b: Board = [(0, 0), (0, 1)].iter().cloned().collect();
        let expected: CoordSet = [(0, 0), (0, 1)].iter().cloned().collect();
        assert_eq!(b.get_queen_positions(), expected);
    }
}

#[cfg(test)]
mod board_from_pos_iter_benches {
    extern crate rand;
    extern crate test;
    use self::test::Bencher;
    use super::Board;
    use rand::Rng;
    use std::iter::empty;
    use PosCoords;

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

    #[bench]
    fn time_creating_board_with_eight_random_queens_from_iter(bencher: &mut Bencher) {
        let coords = generate_random_coordinates();
        bencher.iter(|| {
            let _: Board = coords.iter().cloned().collect();
        });
    }

    fn generate_random_coordinates() -> Vec<PosCoords> {
        let mut x_coords: Vec<u32> = (0..8).collect();
        let mut y_coords: Vec<u32> = (0..8).collect();
        let mut rng = rand::thread_rng();
        rng.shuffle(&mut x_coords);
        rng.shuffle(&mut y_coords);
        (0..8)
            .map(|i| {
                let row = y_coords[i];
                let col = x_coords[i];
                (col, row)
            })
            .collect()
    }
}
