use super::QueenState;
use Queens;

pub fn get_queen_states(queens: &Queens) -> Vec<QueenState> {
    queens
        .get_queen_positions()
        .iter()
        .map(|&pos| QueenState {
            pos: pos,
            moves: queens.get_queen_moves(pos),
        })
        .collect()
}

#[cfg(test)]
mod get_queen_states_benches {
    extern crate rand;
    extern crate test;
    use rand::Rng;
    use self::test::Bencher;
    use {Board, PosCoords};
    use super::get_queen_states;

    #[bench]
    fn get_queen_states_empty_board(bencher: &mut Bencher) {
        let b = Board::new();
        bencher.iter(|| {
            let _ = get_queen_states(&b);
        });
    }

    #[bench]
    fn get_queen_states_four_queens(bencher: &mut Bencher) {
        let b: Board = get_n_random_coords(4).into_iter().collect();
        bencher.iter(|| {
            let _ = get_queen_states(&b);
        });
    }

    #[bench]
    fn get_queen_states_seven_queens(bencher: &mut Bencher) {
        let b: Board = get_n_random_coords(7).into_iter().collect::<Board>();
        bencher.iter(|| {
            let _ = get_queen_states(&b);
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
