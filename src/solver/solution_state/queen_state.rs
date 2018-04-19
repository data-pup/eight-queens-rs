use std::collections::HashSet;
use PosCoords;

/// This struct represents a queen's state. This struct stores a queen's
/// current position and a set of the coordinates it can move to.
pub struct QueenState {
    pub pos: PosCoords,
    pub moves: HashSet<PosCoords>,
}
