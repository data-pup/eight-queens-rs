use position_types::{CoordSet, PosCoords};
use std::cmp::min;

/// Get the coordinates of the possible moves that a queen can
/// potential make. This identifies the squares a queen is contesting.
pub fn get_queen_moves(pos: PosCoords, dims: PosCoords) -> CoordSet {
    [
        get_vert_moves(pos, dims),
        get_horiz_moves(pos, dims),
        get_nw_moves(pos, dims),
        get_ne_moves(pos, dims),
        get_sw_moves(pos, dims),
        get_se_moves(pos, dims),
    ].iter()
        .flatten()
        .cloned()
        .collect()
}

/// This function will return a vector of the vertical moves a queen at
/// a given position `pos` can make.
fn get_vert_moves(pos: PosCoords, dims: PosCoords) -> Vec<PosCoords> {
    (0..dims.1).map(|y| (pos.0, y)).collect()
}

/// This function will return a vector of the horizontal moves a queen at
/// a given position `pos` can make.
fn get_horiz_moves(pos: PosCoords, dims: PosCoords) -> Vec<PosCoords> {
    (0..dims.0).map(|x| (x, pos.1)).collect()
}

/// This function will return a set of the possible diagonal moves
/// going up and to the left.
fn get_nw_moves(pos: PosCoords, dims: PosCoords) -> Vec<PosCoords> {
    let dis_to_edge = min(pos.0 + 1, dims.1 - pos.1);
    (0..dis_to_edge)
        .map(|delta| (pos.0 - delta, pos.1 + delta))
        .collect()
}

/// This function will return a set of the possible diagonal moves
/// going up and to the right.
fn get_ne_moves(pos: PosCoords, dims: PosCoords) -> Vec<PosCoords> {
    let dis_to_edge = min(dims.0 - pos.0, dims.1 - pos.1);
    (0..dis_to_edge)
        .map(|delta| (pos.0 + delta, pos.1 + delta))
        .collect()
}

/// This function will return a set of the possible diagonal moves
/// going down and to the left.
fn get_sw_moves(pos: PosCoords, _: PosCoords) -> Vec<PosCoords> {
    let dis_to_edge = min(pos.0 + 1, pos.1 + 1);
    (0..dis_to_edge)
        .map(|delta| (pos.0 - delta, pos.1 - delta))
        .collect()
}

/// This function will return a set of the possible diagonal moves
/// going down and to the right.
fn get_se_moves(pos: PosCoords, dims: PosCoords) -> Vec<PosCoords> {
    let dis_to_edge = min(dims.0 - pos.0, pos.1 + 1);
    (0..dis_to_edge)
        .map(|delta| (pos.0 + delta, pos.1 - delta))
        .collect()
}
