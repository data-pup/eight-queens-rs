pub mod board;

pub type PosIndex = usize;
pub type PosCoords = (u32, u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PosError {
    InvalidRow,
    InvalidColumn,
    InvalidIndex,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Square {
    Empty = 0,
    Queen = 1,
}

pub trait Queens {
    fn add_queen(&mut self, row: u32, col: u32);
    fn get_queen_positions(&self) -> Vec<PosCoords>;
}

pub trait UncontestedSpaces {
    fn get_uncontested_spaces(&self) -> Vec<PosCoords>;
}

mod position_types {
    pub use PosIndex;
    pub use PosCoords;
    pub use PosError;
}
