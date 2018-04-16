pub mod board;

pub type Position = (u32, u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Square {
    Empty = 0,
    Queen = 1,
}

trait Queens {
    fn get_queen_positions() -> Vec<Position>;
}
