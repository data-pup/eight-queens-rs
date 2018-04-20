mod solution_state;
use self::solution_state::SolutionState;

use board::Board;
use {CoordSet, Queens, Solutions};

/// This struct is used to find solutions to the problem, given a board state.
#[derive(Clone, Debug)]
pub struct Solver {
    _curr_board: Board,
    _soln_state: SolutionState,
}

impl Solver {
    /// Create a new solver object.
    pub fn _new(b: Board) -> Solver {
        Solver {
            _curr_board: b.clone(),
            _soln_state: SolutionState::from(b),
        }
    }

    /// Create a vector of the next possible moves.
    /// TODO: Return in descending order sorted by number of remaining
    /// uncontested squares on the board?
    fn _get_next_moves(&self) -> Vec<Solver> {
        self._curr_board
            .get_uncontested_spaces()
            .iter()
            .map(|&(col, row)| {
                let mut next_state = self._curr_board.clone();
                next_state.add_queen(row, col);
                next_state
            })
            .map(Solver::_new)
            .filter(|s| !s._soln_state.has_conflict)
            .collect::<Vec<Solver>>()
    }
}

/// Implement the Solutions trait for the solver class. This will return a set
/// of solutions.
impl Solutions for Solver {
    /// Calculate solutions to the problem.
    fn get_solutions(&self) -> Option<Vec<CoordSet>> {
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
                .collect::<Vec<CoordSet>>();
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
        let s = Solver::_new(b);
        bencher.iter(|| {
            let _ = s._get_next_moves();
        });
    }

    #[bench]
    fn time_get_next_moves_for_board_with_two_queens(bencher: &mut Bencher) {
        let b: Board = get_n_random_coords(2).into_iter().collect();
        let s = Solver::_new(b);
        bencher.iter(|| {
            let _ = s._get_next_moves();
        });
    }

    #[bench]
    fn time_get_next_moves_for_board_with_four_queens(bencher: &mut Bencher) {
        let b: Board = get_n_random_coords(4).into_iter().collect();
        let s = Solver::_new(b);
        bencher.iter(|| {
            let _ = s._get_next_moves();
        });
    }

    #[bench]
    fn time_get_next_moves_for_board_with_seven_queens(bencher: &mut Bencher) {
        let b: Board = get_n_random_coords(7).into_iter().collect();
        let s = Solver::_new(b);
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
