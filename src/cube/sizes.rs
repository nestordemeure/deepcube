//! Various dimensions of a Rubik's cube

/// Number of squares on the side of a cube
pub const NB_SQUARES_SIDE: usize = 3;

/// Number of squares on the face of a cube
const NB_SQUARES_FACE: usize = NB_SQUARES_SIDE * NB_SQUARES_SIDE;

/// Number of faces on a cube
pub const NB_FACES: usize = 6;

/// Total number of squares on a cube
pub const NB_SQUARES_CUBE: usize = NB_FACES * NB_SQUARES_FACE;
