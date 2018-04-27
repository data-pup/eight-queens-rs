#![feature(iterator_flatten, test, try_from)]

extern crate rand;

pub mod board;
pub mod checker;
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
