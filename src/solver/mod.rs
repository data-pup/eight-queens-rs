use std::collections::HashSet;

use checker::{check_board, CheckResult};
use position::CoordIter;
use queen::get_contested_spaces;
use {Board, CoordList, PosCoords};

/// This struct is used to find solutions to the problem, given a board state.
#[derive(Clone, Debug)]
pub struct Solver {
    solutions: HashSet<CoordList>,
    state_heap: Vec<CoordList>,
    visited: HashSet<Board>,
    dimensions: PosCoords,
}

impl Solver {
    // Create a new solver object.
    pub fn new() -> Solver {
        let mut state_heap = Vec::new();
        let default_board = CoordList::new();
        state_heap.push(default_board);
        Solver {
            state_heap,
            visited: HashSet::new(),
            solutions: HashSet::new(),
            dimensions: (8, 8),
        }
    }

    /// Returns true if the solver is done examining moves.
    pub fn is_done(&self) -> bool {
        self.state_heap.is_empty()
    }

    /// Returns true if the solver has found at least one solution.
    pub fn solution_exists(&self) -> bool {
        !self.solutions.is_empty()
    }

    /// Tick the solver forward one iteration. Attempt to pop an item off of
    /// the state heap, and check if it is a solution. If it is a solution,
    /// add it (and its reflections) to the solutions set. If it is not a
    /// solution, add it (and its reflections) to the visited set, then
    /// calculate the next possible moves.
    pub fn tick(&mut self) {
        if let Some(queen_positions) = self.state_heap.pop() {
            let board = queen_positions.iter().cloned().collect::<Board>();
            self.add_state_and_reflections_to_visited(&board);
            let state_check = check_board(board.clone());
            if state_check.is_solved {
                self.solutions.insert(queen_positions);
                return;
            } else {
                let next_best_moves = self.get_next_moves(queen_positions);
                self.state_heap.extend(next_best_moves);
            }
        }
    }

    /// Progress until the next solution is found.
    pub fn get_next_solution(&mut self) {
        let initial_soln_count = self.solutions.len();
        while self.solutions.len() == initial_soln_count {
            self.tick();
        }
    }

    /// Find all of the solutions to the eight queen problem.
    pub fn solve(&mut self) -> HashSet<CoordList> {
        while !self.is_done() {
            self.tick();
        }
        self.solutions.clone()
    }

    /// Get the next best moves from the board state, given as a list of
    /// position coordinates.
    fn get_next_moves(&self, queen_positions: CoordList) -> Vec<CoordList> {
        let board = queen_positions.iter().cloned().collect::<Board>();
        let contested: HashSet<PosCoords> = get_contested_spaces(queen_positions, self.dimensions)
            .iter()
            .cloned()
            .collect();
        let uncontested: HashSet<PosCoords> = CoordIter::from(board.dims())
            .filter(|pos| !contested.contains(pos))
            .collect();
        let mut _move_checks = uncontested
            .into_iter()
            .map(|new_queen_pos| {
                let mut new_board = board.clone();
                new_board.add_queen(new_queen_pos);
                let queen_positions = new_board.get_queen_positions();
                let check_result = check_board(new_board);
                (queen_positions, check_result)
            })
            .collect::<Vec<(Vec<PosCoords>, CheckResult)>>();
        _move_checks.sort_by_key(|elem| elem.1.clone());
        let next_best_moves = _move_checks
            .into_iter()
            .rev()
            .take(20)
            .map(|(pos_vec, _)| pos_vec)
            .collect();
        next_best_moves
    }

    /// Add a board and its equivalent reflections to the visisted table.
    fn add_state_and_reflections_to_visited(&mut self, board: &Board) {
        self.visited.insert(board.clone());
        board.get_reflections().into_iter().for_each(|board| {
            self.visited.insert(board);
        });
    }
}

impl From<Board> for Solver {
    fn from(board: Board) -> Solver {
        let mut state_heap = Vec::new();
        state_heap.push(board.get_queen_positions());
        Solver {
            state_heap,
            visited: HashSet::new(),
            solutions: HashSet::new(),
            dimensions: board.dims(),
        }
    }
}

#[cfg(test)]
mod cummulative_tick_bench {
    extern crate rand;
    extern crate test;
    use self::test::Bencher;
    use super::Solver;
    use rand::Rng;
    use {Board, PosCoords};

    #[bench]
    fn time_1_tick_for_empty_board(bencher: &mut Bencher) {
        let mut s = Solver::new();
        bencher.iter(|| tick_n_times(&mut s, 1));
    }

    #[bench]
    fn time_2_tick_for_empty_board(bencher: &mut Bencher) {
        let mut s = Solver::new();
        bencher.iter(|| tick_n_times(&mut s, 2));
    }

    #[bench]
    fn time_4_tick_for_empty_board(bencher: &mut Bencher) {
        let mut s = Solver::new();
        bencher.iter(|| tick_n_times(&mut s, 4));
    }

    #[bench]
    fn time_8_tick_for_empty_board(bencher: &mut Bencher) {
        let mut s = Solver::new();
        bencher.iter(|| tick_n_times(&mut s, 8));
    }

    #[bench]
    fn time_32_tick_for_empty_board(bencher: &mut Bencher) {
        let mut s = Solver::new();
        bencher.iter(|| tick_n_times(&mut s, 32));
    }

    #[bench]
    fn time_256_tick_for_empty_board(bencher: &mut Bencher) {
        let mut s = Solver::new();
        bencher.iter(|| tick_n_times(&mut s, 256));
    }

    #[bench]
    fn time_1024_tick_for_empty_board(bencher: &mut Bencher) {
        let mut s = Solver::new();
        bencher.iter(|| tick_n_times(&mut s, 1024));
    }

    #[bench]
    fn time_tick_for_board_with_two_queens(bencher: &mut Bencher) {
        let b: Board = get_n_random_coords(2).into_iter().collect();
        let mut s = Solver::from(b);
        bencher.iter(|| {
            let _ = s.tick();
        });
    }

    #[bench]
    fn time_tick_for_board_with_four_queens(bencher: &mut Bencher) {
        let b: Board = get_n_random_coords(4).into_iter().collect();
        let mut s = Solver::from(b);
        bencher.iter(|| {
            let _ = s.tick();
        });
    }

    #[bench]
    fn time_tick_for_board_with_seven_queens(bencher: &mut Bencher) {
        let b: Board = get_n_random_coords(7).into_iter().collect();
        let mut s = Solver::from(b);
        bencher.iter(|| {
            let _ = s.tick();
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

    fn tick_n_times(solver: &mut Solver, n: u32) {
        for _ in 0..n {
            solver.tick();
        }
    }
}

#[cfg(test)]
mod single_tick_bench {
    extern crate rand;
    extern crate test;
    use self::test::Bencher;
    use super::Solver;

    #[bench]
    fn time_4th_tick(bencher: &mut Bencher) {
        let mut s = Solver::new();
        tick_n_times(&mut s, 3);
        bencher.iter(|| {
            let _ = s.tick();
        });
    }

    #[bench]
    fn time_32nd_tick(bencher: &mut Bencher) {
        let mut s = Solver::new();
        tick_n_times(&mut s, 31);
        bencher.iter(|| {
            let _ = s.tick();
        });
    }

    #[bench]
    fn time_256th_tick(bencher: &mut Bencher) {
        let mut s = Solver::new();
        tick_n_times(&mut s, 255);
        bencher.iter(|| {
            let _ = s.tick();
        });
    }

    fn tick_n_times(solver: &mut Solver, n: u32) {
        for _ in 0..n {
            solver.tick();
        }
    }
}

// #[cfg(test)]
// mod get_next_solutions_tests {
//     extern crate test;
//     use self::test::Bencher;
//     use super::Solver;

//     #[bench]
//     fn time_until_1_solution(bencher: &mut Bencher) {
//         let mut s = Solver::new();
//         bencher.iter(|| {
//             get_n_solutions(&mut s, 1);
//         });
//     }

//     fn get_n_solutions(solver: &mut Solver, n: u32) {
//         for _ in 0..n {
//             solver.get_next_solution();
//         }
//     }
// }

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
        let mut solver = Solver::from(b);
        let soln_set = solver.solve();
        let expected_soln_coords: HashSet<CoordList> = vec![
            [
                (0, 4),
                (1, 2),
                (2, 0),
                (3, 6),
                (4, 1),
                (5, 7),
                (6, 5),
                (7, 3),
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
        let mut solver = Solver::from(b);
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
        let mut solver = Solver::from(b);
        bencher.iter(|| {
            solver.solve();
        });
    }

    #[bench]
    fn time_get_solution_from_3_queen_pos(bencher: &mut Bencher) {
        let b: Board = [(2, 0), (4, 1), (1, 2)].iter().cloned().collect();
        let mut solver = Solver::from(b);
        bencher.iter(|| {
            solver.solve();
        });
    }

    // #[bench]
    // fn time_get_solution_from_empty_board(bencher: &mut Bencher) {
    //     let mut solver = Solver::new();
    //     bencher.iter(|| {
    //         solver.solve();
    //     });
    // }
}
