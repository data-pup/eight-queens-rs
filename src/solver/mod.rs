mod update_visited;

use checker::{check_board, CheckResult};
use {Board, CoordSet, PosCoords, Solutions};

/// FIXUP: This will be the new function, the solver struct will not be public.
pub fn get_solution() {
    unimplemented!();
}

/// This struct is used to find solutions to the problem, given a board state.
#[derive(Clone, Debug)]
pub struct Solver {
    _curr_board: Board,
    _soln_state: CheckResult,
}

impl Solver {
    /// Create a new solver object.
    pub fn new(board: Board) -> Solver {
        Solver {
            _curr_board: board.clone(),
            _soln_state: check_board(board),
        }
    }

    /// Create a vector of the next possible moves.
    /// TODO: Return in descending order sorted by number of remaining
    /// uncontested squares on the board?
    fn _get_next_moves(&self) -> Vec<Solver> {
        unimplemented!();
        // self._curr_board
        //     .get_uncontested_spaces()
        //     .iter()
        //     .map(|&pos| {
        //         let mut next_state = self._curr_board.clone();
        //         next_state.add_queen(pos);
        //         next_state
        //     })
        //     .map(Solver::new)
        //     .filter(|s| !s._soln_state.has_conflict)
        //     .collect::<Vec<Solver>>()
    }
}

/// Implement the Solutions trait for the solver class. This will return a set
/// of solutions.
impl Solutions for Solver {
    /// Calculate solutions to the problem.
    fn get_solutions(&self) -> Option<Vec<Vec<PosCoords>>> {
        if self._soln_state.has_conflict {
            None
        } else if self._soln_state.is_solved {
            let soln = vec![self._curr_board.get_queen_positions()];
            Some(soln)
        } else {
            let solns = self._get_next_moves()
                .iter()
                .filter_map(|next_move| next_move.get_solutions())
                .flatten()
                .collect::<Vec<Vec<PosCoords>>>();
            Some(solns)
        }
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
        let s = Solver::new(b);
        bencher.iter(|| {
            let _ = s._get_next_moves();
        });
    }

    #[bench]
    fn time_get_next_moves_for_board_with_two_queens(bencher: &mut Bencher) {
        let b: Board = get_n_random_coords(2).into_iter().collect();
        let s = Solver::new(b);
        bencher.iter(|| {
            let _ = s._get_next_moves();
        });
    }

    #[bench]
    fn time_get_next_moves_for_board_with_four_queens(bencher: &mut Bencher) {
        let b: Board = get_n_random_coords(4).into_iter().collect();
        let s = Solver::new(b);
        bencher.iter(|| {
            let _ = s._get_next_moves();
        });
    }

    #[bench]
    fn time_get_next_moves_for_board_with_seven_queens(bencher: &mut Bencher) {
        let b: Board = get_n_random_coords(7).into_iter().collect();
        let s = Solver::new(b);
        bencher.iter(|| {
            let _ = s._get_next_moves();
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
mod get_solutions_tests {
    use super::Solver;
    use {Board, CoordSet, Solutions};

    /// Time the `get_solutions` method, starting at a position with 7 queens
    /// on the board. One queen placed at (5, 7) will solve the problem.
    #[test]
    fn test_correct_solution_is_found_for_7_queen_pos() {
        let b: Board = [(2, 0), (4, 1), (1, 2), (7, 3), (0, 4), (6, 5), (3, 6)]
            .iter()
            .cloned()
            .collect();
        let solver = Solver::new(b);
        let soln_set = solver.get_solutions().unwrap();
        let expected_soln_coords: Vec<CoordSet> = vec![
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
                .collect(),
        ];
        assert_eq!(soln_set, expected_soln_coords);
    }
}

#[cfg(test)]
mod get_solutions_benches {
    extern crate test;
    use self::test::Bencher;
    use super::Solver;
    use {Board, Solutions};

    /// Time the `get_solutions` method, starting at a position with 7 queens
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
        let solver = Solver::new(b);
        bencher.iter(|| {
            solver.get_solutions();
        });
    }

    /// Time the `get_solutions` method, starting at a position with 7 queens
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
        let solver = Solver::new(b);
        bencher.iter(|| {
            solver.get_solutions();
        });
    }

    #[bench]
    fn time_get_solution_from_3_queen_pos(bencher: &mut Bencher) {
        let b: Board = [(2, 0), (4, 1), (1, 2)].iter().cloned().collect();
        let solver = Solver::new(b);
        bencher.iter(|| {
            solver.get_solutions();
        });
    }

    // #[bench]
    // fn time_get_solution_from_empty_board(bencher: &mut Bencher) {
    //     let b: Board = Board::new();
    //     let solver = Solver::new(b);
    //     bencher.iter(|| {
    //         solver.get_solutions();
    //     });
    // }
}
