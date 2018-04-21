#![feature(iterator_flatten, test, try_from)]

extern crate rand;

use std::collections::HashSet;

pub mod board;
pub mod solver;

pub use board::Board;

/// Position index. Note: This may be made private?
pub type PosIndex = usize;
/// Position coordinates. Note: These are in (x, y) format.
pub type PosCoords = (u32, u32);
// A set of position coordinates representing the current queen positions.
pub type CoordSet = HashSet<PosCoords>;

/// Position errors. Thrown if a coordinate access attempt is out of bounds.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PosError {
    OutOfBounds,
}

/// Square type, this represent whether or not a square is occupied.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Square {
    Empty = 0,
    Queen = 1,
}

/// This trait specifies methods related to adding queens, finding their
/// positions, and finding their possible moves on the board.
pub trait Queens {
    fn add_queen(&mut self, row: u32, col: u32);
    fn get_queen_positions(&self) -> CoordSet;
    fn get_queen_moves(&self, pos: PosCoords) -> CoordSet;
    fn get_uncontested_spaces(&self) -> CoordSet;
}

/// This trait is used to identify reflections of a given state.
pub trait Reflection {
    fn get_horizontal_reflection(&self) -> Board;
    fn get_vertical_reflection(&self) -> Board;
    fn get_inverse(&self) -> Board;
}

/// Solutions trait, this specifies the method to be called to calculate
/// solutions to the problem, and return them as a set.
pub trait Solutions {
    fn get_solutions(&self) -> Option<Vec<CoordSet>>;
}

/// Module used to import the different position types, and the error enum
/// that can be returned in the event of an invalid coordinate pair.
mod position_types {
    pub use PosCoords;
    pub use PosError;
    pub use PosIndex;
}
