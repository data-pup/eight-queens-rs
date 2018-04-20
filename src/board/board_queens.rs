use super::Board;
use position_types::*;
use std::cmp::min;
use std::collections::HashSet;
use {Queens, Square};

impl Queens for Board {
    /// Add a queen to the board at the given position.
    fn add_queen(&mut self, row: u32, col: u32) {
        let i = self.get_pos_index(row, col);
        self.squares[i] = Square::Queen;
    }

    /// Get the positions of the queens.
    fn get_queen_positions(&self) -> HashSet<PosCoords> {
        self.squares
            .iter()
            .enumerate()
            .filter(|(_, &s)| s == Square::Queen)
            .map(|(i, _)| self.get_index_pos(i))
            .collect::<HashSet<PosCoords>>()
    }

    /// Get the coordinates of the possible moves that a queen can
    /// potential make. This identifies the squares a queen is contesting.
    fn get_queen_moves(&self, pos: PosCoords) -> HashSet<PosCoords> {
        [
            &self.get_n_moves(pos),
            &self.get_s_moves(pos),
            &self.get_w_moves(pos),
            &self.get_e_moves(pos),
            &self.get_nw_moves(pos),
            &self.get_ne_moves(pos),
            &self.get_sw_moves(pos),
            &self.get_se_moves(pos),
        ].iter()
            .fold(HashSet::new(), |res, dir| res.union(dir).cloned().collect())
    }

    /// Get a set of the uncontested spaces on the board. This identifies the
    /// positions at which a new queen can be added to the board.
    fn get_uncontested_spaces(&self) -> HashSet<PosCoords> {
        let all_spaces = self.squares
            .iter()
            .enumerate()
            .map(|(i, _)| self.get_index_pos(i))
            .collect::<HashSet<PosCoords>>();
        let contested_spaces = self.get_queen_positions()
            .iter()
            .map(|&p| self.get_queen_moves(p))
            .fold(HashSet::new(), |result, curr| {
                result.union(&curr).cloned().collect()
            });
        all_spaces.difference(&contested_spaces).cloned().collect()
    }
}

// This block stores helper functions for finding the moves in a given
// direction. These functions each return a hash set of coordinates.
impl Board {
    /// This function will return a set of the possible upward moves.
    fn get_n_moves(&self, pos: PosCoords) -> HashSet<PosCoords> {
        (pos.1..self.height).map(|y| (pos.0, y)).collect()
    }

    /// This function will return a set of the possible downward moves.
    fn get_s_moves(&self, pos: PosCoords) -> HashSet<PosCoords> {
        (0..pos.1 + 1).map(|y| (pos.0, y)).collect()
    }

    /// This function will return a set of the possible leftward moves.
    fn get_w_moves(&self, pos: PosCoords) -> HashSet<PosCoords> {
        (0..pos.0 + 1).map(|x| (x, pos.1)).collect()
    }

    /// This function will return a set of the possible rightward moves.
    fn get_e_moves(&self, pos: PosCoords) -> HashSet<PosCoords> {
        (pos.0..self.width).map(|x| (x, pos.1)).collect()
    }

    /// This function will return a set of the possible diagonal moves
    /// going up and to the left.
    fn get_nw_moves(&self, pos: PosCoords) -> HashSet<PosCoords> {
        let dis_to_edge = min(pos.0 + 1, self.height - pos.1);
        (0..dis_to_edge)
            .map(|delta| (pos.0 - delta, pos.1 + delta))
            .collect()
    }

    /// This function will return a set of the possible diagonal moves
    /// going up and to the right.
    fn get_ne_moves(&self, pos: PosCoords) -> HashSet<PosCoords> {
        let dis_to_edge = min(self.width - pos.0, self.height - pos.1);
        (0..dis_to_edge)
            .map(|delta| (pos.0 + delta, pos.1 + delta))
            .collect()
    }

    /// This function will return a set of the possible diagonal moves
    /// going down and to the left.
    fn get_sw_moves(&self, pos: PosCoords) -> HashSet<PosCoords> {
        let dis_to_edge = min(pos.0 + 1, pos.1 + 1);
        (0..dis_to_edge)
            .map(|delta| (pos.0 - delta, pos.1 - delta))
            .collect()
    }

    /// This function will return a set of the possible diagonal moves
    /// going down and to the right.
    fn get_se_moves(&self, pos: PosCoords) -> HashSet<PosCoords> {
        let dis_to_edge = min(self.width - pos.0, pos.1 + 1);
        (0..dis_to_edge)
            .map(|delta| (pos.0 + delta, pos.1 - delta))
            .collect()
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
        let mut b = Board::new();
        for curr in [(0, 0), (0, 7), (7, 0), (7, 7)].iter() {
            let &(col, row) = curr;
            b.add_queen(row, col);
        }
        let queen_positions = b.get_queen_positions();
        let expected: HashSet<PosCoords> =
            [(0, 0), (0, 7), (7, 0), (7, 7)].iter().cloned().collect();
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

    #[test]
    fn get_n_moves_works_from_origin() {
        let b = Board::new();
        let pos = (0, 0);
        let expected = [
            (0, 0),
            (0, 1),
            (0, 2),
            (0, 3),
            (0, 4),
            (0, 5),
            (0, 6),
            (0, 7),
        ].iter()
            .cloned()
            .collect();
        let result = b.get_n_moves(pos);
        assert_eq!(result, expected);
    }

    ///   01234567
    ///   --------
    /// 7|        |
    /// 6|        |
    /// 5|        |
    /// 4|        |
    /// 3|x       |
    /// 2| x      |
    /// 1|  x     |
    /// 0|   Q    |
    ///   --------
    #[test]
    fn get_nw_moves_works_from_3_0() {
        let b = Board::new();
        let pos = (3, 0);
        let expected = [(3, 0), (2, 1), (1, 2), (0, 3)].iter().cloned().collect();
        let result = b.get_nw_moves(pos);
        assert_eq!(result, expected);
    }

    #[test]
    fn get_nw_moves_works_from_0_0() {
        let b = Board::new();
        let pos = (0, 0);
        let expected = [(0, 0)].iter().cloned().collect();
        let result = b.get_nw_moves(pos);
        assert_eq!(result, expected);
    }

    #[test]
    fn get_n_moves_returns_curr_pos_from_top_row() {
        let b = Board::new();
        let pos = (0, 7);
        let expected = [(0, 7)].iter().cloned().collect();
        let result = b.get_n_moves(pos);
        assert_eq!(result, expected);
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
    fn queens_in_corners() {
        let mut b = Board::new();
        for curr in [(0, 0), (0, 7), (7, 0), (7, 7)].iter() {
            let &(col, row) = curr;
            b.add_queen(row, col);
        }
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
        let results = b.get_uncontested_spaces();
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
    fn get_queen_n_moves(bencher: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range::<u32>(0, 8);
        let y = rng.gen_range::<u32>(0, 8);
        let pos = (x, y);
        let b = Board::new();
        bencher.iter(|| {
            let _ = b.get_n_moves(pos);
        });
    }

    #[bench]
    fn get_queen_ne_moves(bencher: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range::<u32>(0, 8);
        let y = rng.gen_range::<u32>(0, 8);
        let pos = (x, y);
        let b = Board::new();
        bencher.iter(|| {
            let _ = b.get_ne_moves(pos);
        });
    }

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
