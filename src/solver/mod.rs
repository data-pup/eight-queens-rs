use std::collections::{BinaryHeap, HashSet};

mod update_visited;

use checker::{check_board, CheckResult};
use position::CoordIter;
use queen::get_contested_spaces;
use {Board, CoordList, PosCoords};

/// This struct is used to find solutions to the problem, given a board state.
#[derive(Clone, Debug)]
pub struct Solver {
    _state_heap: BinaryHeap<CoordList>,
    _visited: HashSet<CoordList>,
    _solutions: HashSet<CoordList>,
    _dimensions: PosCoords,
}

impl Solver {
    /// Create a new solver object.
    pub fn new(board: Board) -> Solver {
        let mut _state_heap = BinaryHeap::new();
        _state_heap.push(vec![]);
        Solver {
            _state_heap,
            _visited: HashSet::new(),
            _solutions: HashSet::new(),
            _dimensions: board.dims(),
        }
    }

    /// Returns true if the solver is done examining moves.
    pub fn is_done(&self) -> bool {
        self._state_heap.is_empty()
    }

    /// Create a vector of the next possible moves.
    pub fn _tick(&mut self) {
        // TODO:
        // ?. Add the current position list, and its reflections, to some cache.
        // 1. Get the uncontested spaces on the board.
        // 2. Map the uncontested spaces into solution check results.
        // 3. Sort them descending according to which has the most free spaces.
        // 4. Take (n) next moves, and get the solutions for those states.
        if let Some(queen_positions) = self._state_heap.pop() {
            let board = queen_positions.iter().cloned().collect::<Board>();
            let contested: HashSet<PosCoords> =
                get_contested_spaces(queen_positions, self._dimensions)
                    .iter()
                    .cloned()
                    .collect();
            let uncontested: HashSet<PosCoords> = CoordIter::from(board.clone())
                .filter(|pos| !contested.contains(pos))
                .collect();
            let move_checks = uncontested.into_iter().map(|new_queen_pos| {
                let mut new_board = board.clone();
                new_board.add_queen(new_queen_pos);
                check_board(new_board)
            });
        }

        unimplemented!();
    }

    pub fn solve(&mut self) -> HashSet<CoordList> {
        while !self.is_done() {
            self._tick();
        }
        self._solutions.clone()
    }
}

#[cfg(test)]
mod get_next_moves_bench {
    extern crate rand;
    extern crate test;
    use self::test::Bencher;
    use super::Solver;
    use rand::Rng;
    use {Board, PosCoords};

    #[bench]
    fn time_get_next_moves_for_empty_board(bencher: &mut Bencher) {
        let b = Board::new();
        let mut s = Solver::new(b);
        bencher.iter(|| {
            let _ = s.solve();
        });
    }

    #[bench]
    fn time_get_next_moves_for_board_with_two_queens(bencher: &mut Bencher) {
        let b: Board = get_n_random_coords(2).into_iter().collect();
        let mut s = Solver::new(b);
        bencher.iter(|| {
            let _ = s.solve();
        });
    }

    #[bench]
    fn time_get_next_moves_for_board_with_four_queens(bencher: &mut Bencher) {
        let b: Board = get_n_random_coords(4).into_iter().collect();
        let mut s = Solver::new(b);
        bencher.iter(|| {
            let _ = s.solve();
        });
    }

    #[bench]
    fn time_get_next_moves_for_board_with_seven_queens(bencher: &mut Bencher) {
        let b: Board = get_n_random_coords(7).into_iter().collect();
        let mut s = Solver::new(b);
        bencher.iter(|| {
            let _ = s.solve();
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

// #[cfg(test)]
// mod solve_tests {
//     use super::Solver;
//     use {Board, CoordList};
//     /// Test the `solve` method, starting at a position with 7 queens
//     /// on the board. One queen placed at (5, 7) will solve the problem.
//     #[test]
//     fn test_correct_solution_is_found_for_7_queen_pos() {
//         let b: Board = [(2, 0), (4, 1), (1, 2), (7, 3), (0, 4), (6, 5), (3, 6)]
//             .iter()
//             .cloned()
//             .collect();
//         let mut solver = Solver::new(b);
//         let soln_set = solver.solve().unwrap();
//         let expected_soln_coords: Vec<CoordList> = vec![
//             [
//                 (2, 0),
//                 (4, 1),
//                 (1, 2),
//                 (7, 3),
//                 (0, 4),
//                 (6, 5),
//                 (3, 6),
//                 (5, 7),
//             ].iter()
//                 .cloned()
//                 .collect(),
//         ];
//         assert_eq!(soln_set, expected_soln_coords);
//     }
// }

#[cfg(test)]
mod solve_benches {
    extern crate test;
    use self::test::Bencher;
    use super::Solver;
    use Board;

    /// Time the `solve` method, starting at a position with 7 queens
    /// on the board. One queen placed at (5, 7) will solve the problem.
    /// (On the board below, the missing queen is labeled 'A')
    ///   01234567
    ///   --------
    /// 7|     A  |
    /// 6|   Q    |
    /// 5|      Q |
    /// 4|Q       |
    /// 3|       Q|
    /// 2| Q      |
    /// 1|    Q   |
    /// 0|  Q     |
    ///   --------
    #[bench]
    fn time_get_solution_from_7_queen_pos(bencher: &mut Bencher) {
        let b: Board = [(2, 0), (4, 1), (1, 2), (7, 3), (0, 4), (6, 5), (3, 6)]
            .iter()
            .cloned()
            .collect();
        let mut solver = Solver::new(b);
        bencher.iter(|| {
            solver.solve();
        });
    }

    /// Time the `solve` method, starting at a position with 7 queens
    /// on the board. Queens placed at (5, 7), (3, 6), and (6, 5) will solve
    /// the problem. (On the board below, the missing queens are labeled 'A')
    /// NOTE: Other solutions may exist from this position.
    ///   01234567
    ///   --------
    /// 7|     A  |
    /// 6|   A    |
    /// 5|      A |
    /// 4|Q       |
    /// 3|       Q|
    /// 2| Q      |
    /// 1|    Q   |
    /// 0|  Q     |
    ///   --------
    #[bench]
    fn time_get_solution_from_5_queen_pos(bencher: &mut Bencher) {
        let b: Board = [(2, 0), (4, 1), (1, 2), (7, 3), (0, 4)]
            .iter()
            .cloned()
            .collect();
        let mut solver = Solver::new(b);
        bencher.iter(|| {
            solver.solve();
        });
    }

    #[bench]
    fn time_get_solution_from_3_queen_pos(bencher: &mut Bencher) {
        let b: Board = [(2, 0), (4, 1), (1, 2)].iter().cloned().collect();
        let mut solver = Solver::new(b);
        bencher.iter(|| {
            solver.solve();
        });
    }

    // #[bench]
    // fn time_get_solution_from_empty_board(bencher: &mut Bencher) {
    //     let b: Board = Board::new();
    //     let mut solver = Solver::new(b);
    //     bencher.iter(|| {
    //         solver.solve();
    //     });
    // }
}
