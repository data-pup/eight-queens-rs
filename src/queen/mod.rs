use position_types::{CoordList, PosCoords};
use std::cmp::min;

/// Return a set of all of the contested spaces on the board, given the
/// positions of each queen, and the dimensions of the board.
pub fn get_contested_spaces(queens: Vec<PosCoords>, dims: PosCoords) -> CoordList {
    let mut coords = get_queen_move_sets(queens, dims)
        .into_iter()
        .flatten()
        .collect::<CoordList>();
    coords.sort();
    coords.dedup();
    coords
}

/// Returns a vector of coordinate sets representing each queen's possible moves.
pub fn get_queen_move_sets(queens: Vec<PosCoords>, dims: PosCoords) -> Vec<CoordList> {
    queens
        .into_iter()
        .map(|pos| get_queen_moves(pos, dims))
        .collect()
}

/// Get the coordinates of the possible moves that a queen can
/// potential make. This identifies the squares a queen is contesting.
pub fn get_queen_moves(pos: PosCoords, dims: PosCoords) -> CoordList {
    let mut moves = [
        get_vert_moves(pos, dims),
        get_horiz_moves(pos, dims),
        get_nw_moves(pos, dims),
        get_ne_moves(pos, dims),
        get_sw_moves(pos, dims),
        get_se_moves(pos, dims),
    ].into_iter()
        .flatten()
        .cloned()
        .collect::<CoordList>();
    moves.sort();
    moves.dedup();
    moves
}

/// This function will return a vector of the vertical moves a queen at
/// a given position `pos` can make.
fn get_vert_moves(pos: PosCoords, dims: PosCoords) -> Vec<PosCoords> {
    (0..dims.1).map(|y| (pos.0, y)).collect()
}

/// This function will return a vector of the horizontal moves a queen at
/// a given position `pos` can make.
fn get_horiz_moves(pos: PosCoords, dims: PosCoords) -> Vec<PosCoords> {
    (0..dims.0).map(|x| (x, pos.1)).collect()
}

/// This function will return a set of the possible diagonal moves
/// going up and to the left.
fn get_nw_moves(pos: PosCoords, dims: PosCoords) -> Vec<PosCoords> {
    let dis_to_edge = min(pos.0 + 1, dims.1 - pos.1);
    (0..dis_to_edge)
        .map(|delta| (pos.0 - delta, pos.1 + delta))
        .collect()
}

/// This function will return a set of the possible diagonal moves
/// going up and to the right.
fn get_ne_moves(pos: PosCoords, dims: PosCoords) -> Vec<PosCoords> {
    let dis_to_edge = min(dims.0 - pos.0, dims.1 - pos.1);
    (0..dis_to_edge)
        .map(|delta| (pos.0 + delta, pos.1 + delta))
        .collect()
}

/// This function will return a set of the possible diagonal moves
/// going down and to the left.
fn get_sw_moves(pos: PosCoords, _: PosCoords) -> Vec<PosCoords> {
    let dis_to_edge = min(pos.0 + 1, pos.1 + 1);
    (0..dis_to_edge)
        .map(|delta| (pos.0 - delta, pos.1 - delta))
        .collect()
}

/// This function will return a set of the possible diagonal moves
/// going down and to the right.
fn get_se_moves(pos: PosCoords, dims: PosCoords) -> Vec<PosCoords> {
    let dis_to_edge = min(dims.0 - pos.0, pos.1 + 1);
    (0..dis_to_edge)
        .map(|delta| (pos.0 + delta, pos.1 - delta))
        .collect()
}

#[cfg(test)]
mod queens_tests {
    use position_types::*;
    use super::{get_queen_moves, get_contested_spaces};
    use std::collections::HashSet;
    use Board;

    #[test]
    fn add_queen_works() {
        let queen_positions: &[PosCoords] = &[(0, 0), (0, 7), (7, 0), (7, 7)];
        let board = queen_positions.iter().cloned().collect::<Board>();
        let queen_positions = board.get_queen_positions();
        let expected = queen_positions.to_vec();
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
        let dims = (8, 8);
        let pos = (2, 2);
        let expected: CoordList = vec![
            (0, 0),
            (0, 2),
            (0, 4),
            (1, 1),
            (1, 2),
            (1, 3),
            (2, 0),
            (2, 1),
            (2, 2),
            (2, 3),
            (2, 4),
            (2, 5),
            (2, 6),
            (2, 7),
            (3, 1),
            (3, 2),
            (3, 3),
            (4, 0),
            (4, 2),
            (4, 4),
            (5, 2),
            (5, 5),
            (6, 2),
            (6, 6),
            (7, 2),
            (7, 7),
        ];
        let result = get_queen_moves(pos, dims);
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
        let dims = (8, 8);
        let pos = (7, 7);
        let expected: CoordList = vec![
            (0, 0),
            (0, 7),
            (1, 1),
            (1, 7),
            (2, 2),
            (2, 7),
            (3, 3),
            (3, 7),
            (4, 4),
            (4, 7),
            (5, 5),
            (5, 7),
            (6, 6),
            (6, 7),
            (7, 0),
            (7, 1),
            (7, 2),
            (7, 3),
            (7, 4),
            (7, 5),
            (7, 6),
            (7, 7),
        ];
        let result = get_queen_moves(pos, dims);
        assert_eq!(result, expected);
    }

    ///   01234567
    ///   --------
    /// 7|x      x|
    /// 6|xx    xx|
    /// 5|x x  x x|
    /// 4|x  xx  x|
    /// 3|x  xx  x|
    /// 2|x x  x x|
    /// 1|xx    xx|
    /// 0|QxxxxxxQ|
    ///   --------
    #[test]
    fn get_contested_moves_works_for_two_queens() {
        let queen_positions: &[PosCoords] = &[(0, 0), (7, 0)];
        let dims = (8, 8);
        let expected: CoordList = vec![
            (0, 0),
            (0, 1),
            (0, 2),
            (0, 3),
            (0, 4),
            (0, 5),
            (0, 6),
            (0, 7),

            (1, 0),
            (1, 1),
            (1, 6),

            (2, 0),
            (2, 2),
            (2, 5),

            (3, 0),
            (3, 3),
            (3, 4),

            (4, 0),
            (4, 3),
            (4, 4),

            (5, 0),
            (5, 2),
            (5, 5),

            (6, 0),
            (6, 1),
            (6, 6),

            (7, 0),
            (7, 1),
            (7, 2),
            (7, 3),
            (7, 4),
            (7, 5),
            (7, 6),
            (7, 7),
        ];
        let result = get_contested_spaces(queen_positions.to_vec(), dims);
        assert_eq!(result, expected);
    }
}

#[cfg(test)]
mod get_queen_moves_benches {
    extern crate rand;
    extern crate test;
    use self::test::Bencher;
    use queen::get_queen_moves;
    use rand::Rng;

    #[bench]
    fn get_queen_moves_bench(bencher: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range::<u32>(0, 8);
        let y = rng.gen_range::<u32>(0, 8);
        let pos = (x, y);
        let dims = (8, 8);
        bencher.iter(|| {
            let _ = get_queen_moves(pos, dims);
        });
    }
}
