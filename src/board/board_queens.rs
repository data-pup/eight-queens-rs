use super::Board;
use queen::get_contested_spaces;
use {CoordSet, Queens};

impl Queens for Board {
    /// Get a set of the uncontested spaces on the board. This identifies the
    /// positions at which a new queen can be added to the board.
    fn get_uncontested_spaces(&self) -> CoordSet {
        let contested_spaces = get_contested_spaces(self.get_queen_positions(), self.dims());
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
