#![feature(iterator_flatten, test, try_from)]

extern crate rand;

pub mod board;
pub mod position;
pub mod queen;
pub mod solver;

pub use board::Board;
pub use position::position_types;
use position::position_types::*;

/// Square type, this represent whether or not a square is occupied.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Square {
    Empty = 0,
    Queen = 1,
}

/// This trait specifies methods related to adding queens, finding their
/// positions, and finding their possible moves on the board.
pub trait Queens {
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
