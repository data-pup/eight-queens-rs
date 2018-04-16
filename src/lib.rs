pub mod board;

pub type Position = (u32, u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Square {
    Empty = 0,
    Queen = 1,
}

pub trait Queens {
    fn add_queen(&mut self, row: u32, col: u32);
    fn get_queen_positions(&self) -> Vec<Position>;
}

pub trait UncontestedSpaces {
    fn get_uncontested_spaces(&self) -> Vec<Position>;
}
