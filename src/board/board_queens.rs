use super::Board;
use position_types::*;
use std::cmp::min;
use std::collections::HashSet;
use {CoordSet, Queens};
use queen::get_contested_spaces;

impl Queens for Board {
    /// Get a set of the uncontested spaces on the board. This identifies the
    /// positions at which a new queen can be added to the board.
    fn get_uncontested_spaces(&self) -> CoordSet {
        let contested_spaces = get_contested_spaces(
            self.get_queen_positions(),
            self.dims());
        let mut uncontested = CoordSet::new();
        for y in 0..self.height {
            for x in 0..self.width {
                let pos = (x, y);
                if !contested_spaces.contains(&pos) {
                    uncontested.insert(pos);
                }
            }
        }
        uncontested
    }
}

#[cfg(test)]
mod board_queens_tests {
    use super::Board;
    use position_types::*;
    use std::collections::HashSet;
    use Queens;

    #[test]
    fn add_queen_works() {
        let queen_positions: &[PosCoords] = &[(0, 0), (0, 7), (7, 0), (7, 7)];
        let board = queen_positions.iter().cloned().collect::<Board>();
        let queen_positions = board.get_queen_positions();
        let expected = queen_positions.iter().cloned().collect::<CoordSet>();
        assert_eq!(queen_positions, expected);
    }

    ///   01234567
    ///   --------
    /// 7|  x    x|
    /// 6|  x   x |
    /// 5|  x  x  |
    /// 4|x x x   |
    /// 3| xxx    |
    /// 2|xxQxxxxx|
    /// 1| xxx    |
    /// 0|x x x   |
    ///   --------
    #[test]
    fn get_queen_moves_works_from_2_2() {
        let b = Board::new();
        let pos = (2, 2);
        let expected = [
            (0, 0),
            (2, 0),
            (4, 0),
            (1, 1),
            (2, 1),
            (3, 1),
            (0, 2),
            (1, 2),
            (2, 2),
            (3, 2),
            (4, 2),
            (5, 2),
            (6, 2),
            (7, 2),
            (1, 3),
            (2, 3),
            (3, 3),
            (0, 4),
            (2, 4),
            (4, 4),
            (2, 5),
            (5, 5),
            (2, 6),
            (6, 6),
            (2, 7),
            (7, 7),
        ].iter()
            .cloned()
            .collect();
        let result = b.get_queen_moves(pos);
        assert_eq!(result, expected);
    }

    ///   01234567
    ///   --------
    /// 7|xxxxxxxQ|
    /// 6|      xx|
    /// 5|     x x|
    /// 4|    x  x|
    /// 3|   x   x|
    /// 2|  x    x|
    /// 1| x     x|
    /// 0|x      x|
    ///   --------
    #[test]
    fn get_queen_moves_works_from_7_7() {
        let b = Board::new();
        let pos = (7, 7);
        let expected = [
            (0, 0),
            (1, 1),
            (2, 2),
            (3, 3),
            (4, 4),
            (5, 5),
            (6, 6),
            (7, 0),
            (7, 1),
            (7, 2),
            (7, 3),
            (7, 4),
            (7, 5),
            (7, 6),
            (0, 7),
            (1, 7),
            (2, 7),
            (3, 7),
            (4, 7),
            (5, 7),
            (6, 7),
            (7, 7),
        ].iter()
            .cloned()
            .collect();
        let result = b.get_queen_moves(pos);
        assert_eq!(
            result,
            expected,
            "Difference: {:?}",
            expected.difference(&result)
        );
    }

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
    fn get_uncontested_moves_works_for_queens_in_corners() {
        let queen_positions: &[PosCoords] = &[(0, 0), (0, 7), (7, 0), (7, 7)];
        let board = queen_positions.iter().cloned().collect::<Board>();
        let expected: HashSet<PosCoords> = [
            (2, 6),
            (3, 6),
            (4, 6),
            (5, 6),
            (1, 5),
            (3, 5),
            (4, 5),
            (6, 5),
            (1, 4),
            (2, 4),
            (5, 4),
            (6, 4),
            (1, 3),
            (2, 3),
            (5, 3),
            (6, 3),
            (1, 2),
            (3, 2),
            (4, 2),
            (6, 2),
            (2, 1),
            (3, 1),
            (4, 1),
            (5, 1),
        ].iter()
            .cloned()
            .collect();
        let results = board.get_uncontested_spaces();
        assert_eq!(results, expected);
    }
}

#[cfg(test)]
mod get_queen_moves_benches {
    extern crate rand;
    extern crate test;
    use self::test::Bencher;
    use super::Board;
    use rand::Rng;
    use Queens;

    #[bench]
    fn get_queen_moves(bencher: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range::<u32>(0, 8);
        let y = rng.gen_range::<u32>(0, 8);
        let pos = (x, y);
        let b = Board::new();
        bencher.iter(|| {
            let _ = b.get_queen_moves(pos);
        });
    }
}

#[cfg(test)]
mod get_uncontested_spaces_benches {
    extern crate rand;
    extern crate test;
    use self::test::Bencher;
    use super::Board;
    use rand::Rng;
    use {PosCoords, Queens};

    #[bench]
    fn get_uncontested_spaces_with_empty_board_bench(bencher: &mut Bencher) {
        let board: Board = Board::new();
        bencher.iter(|| {
            let _ = board.get_uncontested_spaces();
        });
    }

    #[bench]
    fn get_uncontested_spaces_with_two_queens_bench(bencher: &mut Bencher) {
        let board: Board = get_n_random_coords(2).into_iter().collect();
        bencher.iter(|| {
            let _ = board.get_uncontested_spaces();
        });
    }

    #[bench]
    fn get_uncontested_spaces_with_four_queens_bench(bencher: &mut Bencher) {
        let board: Board = get_n_random_coords(4).into_iter().collect();
        bencher.iter(|| {
            let _ = board.get_uncontested_spaces();
        });
    }

    #[bench]
    fn get_uncontested_spaces_with_seven_queens_bench(bencher: &mut Bencher) {
        let board: Board = get_n_random_coords(7).into_iter().collect();
        bencher.iter(|| {
            let _ = board.get_uncontested_spaces();
        });
    }

    fn get_n_random_coords(n: usize) -> Vec<PosCoords> {
        let mut rng = rand::thread_rng();
        let mut y_range = (0..8).collect::<Vec<u32>>();
        let mut x_range = (0..8).collect::<Vec<u32>>();
        rng.shuffle(&mut x_range);
        rng.shuffle(&mut y_range);
        let x_coords: Vec<u32> = x_range.into_iter().take(n).collect();
        let y_coords: Vec<u32> = y_range.into_iter().take(n).collect();
        (0..n).map(|i| (x_coords[i], y_coords[i])).collect()
    }
}
