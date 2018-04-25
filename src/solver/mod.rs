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
        // FIXUP: Add the initial state to the visited set.
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

    /// Tick the solver forward one iteration. Attempt to pop an item off of
    /// the state heap, and check if it is a solution. If it is a solution,
    /// add it (and its reflections) to the solutions set. If it is not a
    /// solution, add it (and its reflections) to the visited set, then
    /// calculate the next possible moves.
    pub fn _tick(&mut self) {
        if let Some(queen_positions) = self._state_heap.pop() {
            // FIXUP: Check if queen_positions can be a solution.
            //        If so, check solution validity, otherwise get next moves.
            let board = queen_positions.iter().cloned().collect::<Board>();
            let contested: HashSet<PosCoords> =
                get_contested_spaces(queen_positions, self._dimensions)
                    .iter()
                    .cloned()
                    .collect();
            // FIXUP: Create CoordIter from coord, not board.
            let uncontested: HashSet<PosCoords> = CoordIter::from(board.clone())
                .filter(|pos| !contested.contains(pos))
                .collect();
            let _move_checks = uncontested
                .into_iter()
                .map(|new_queen_pos| {
                    let mut new_board = board.clone();
                    new_board.add_queen(new_queen_pos);
                    let queen_positions = new_board.get_queen_positions();
                    self._visited.insert(queen_positions.clone());
                    let check_result = check_board(new_board);
                    (queen_positions, check_result)
                })
                .collect::<Vec<(Vec<PosCoords>, CheckResult)>>();
            let next_5_best_moves = _move_checks
                .into_iter()
                .rev()
                .take(5)
                .map(|(pos_vec, _)| pos_vec);
            self._state_heap.extend(next_5_best_moves);
            // FIXUP: Add reflections / state to visited.
            // FIXUP: Check result sorting? This may need to use sort_by_key etc.
        }
    }

    pub fn solve(&mut self) -> HashSet<CoordList> {
        while !self.is_done() {
            self._tick();
        }
        self._solutions.clone()
    }
}

#[cfg(test)]
mod tick_bench {
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
            let _ = s._tick();
        });
    }

    #[bench]
    fn time_get_next_moves_for_board_with_two_queens(bencher: &mut Bencher) {
        let b: Board = get_n_random_coords(2).into_iter().collect();
        let mut s = Solver::new(b);
        bencher.iter(|| {
            let _ = s._tick();
        });
    }

    #[bench]
    fn time_get_next_moves_for_board_with_four_queens(bencher: &mut Bencher) {
        let b: Board = get_n_random_coords(4).into_iter().collect();
        let mut s = Solver::new(b);
        bencher.iter(|| {
            let _ = s._tick();
        });
    }

    #[bench]
    fn time_get_next_moves_for_board_with_seven_queens(bencher: &mut Bencher) {
        let b: Board = get_n_random_coords(7).into_iter().collect();
        let mut s = Solver::new(b);
        bencher.iter(|| {
            let _ = s._tick();
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

#[cfg(test)]
mod solve_tests {
    use super::Solver;
    use std::collections::HashSet;
    use {Board, CoordList};
    /// Test the `solve` method, starting at a position with 7 queens
    /// on the board. One queen placed at (5, 7) will solve the problem.
    #[test]
    fn test_correct_solution_is_found_for_7_queen_pos() {
        let b: Board = [(2, 0), (4, 1), (1, 2), (7, 3), (0, 4), (6, 5), (3, 6)]
            .iter()
            .cloned()
            .collect();
        let mut solver = Solver::new(b);
        let soln_set = solver.solve();
        let expected_soln_coords: HashSet<CoordList> = vec![
            [
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
                .collect::<CoordList>(),
        ].iter()
            .cloned()
            .collect();
        assert_eq!(soln_set, expected_soln_coords);
    }
}

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

    // #[bench]
    // fn time_get_solution_from_3_queen_pos(bencher: &mut Bencher) {
    //     let b: Board = [(2, 0), (4, 1), (1, 2)].iter().cloned().collect();
    //     let mut solver = Solver::new(b);
    //     bencher.iter(|| {
    //         solver.solve();
    //     });
    // }

    // #[bench]
    // fn time_get_solution_from_empty_board(bencher: &mut Bencher) {
    //     let b: Board = Board::new();
    //     let mut solver = Solver::new(b);
    //     bencher.iter(|| {
    //         solver.solve();
    //     });
    // }
}
