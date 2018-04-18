use std::collections::HashSet;
use PosCoords;

pub struct QueenState {
    pub pos: PosCoords,
    pub moves: HashSet<PosCoords>,
}
