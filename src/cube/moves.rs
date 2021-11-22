//! Describes ways to twist a cube
//!
//! See this website for the classical notations:
//! http://www.rubiksplace.com/move-notations/
use enum_iterator::IntoEnumIterator;
use super::sizes::NB_SQUARES_CUBE;
use super::coordinates::Coordinate1D;

//-----------------------------------------------------------------------------
// Move description

/// all the slice of the cube that could move
#[derive(IntoEnumIterator, Copy, Clone, Debug)]
pub enum MoveKind
{
    /// the face facing the solver
    Front,
    /// the back face
    Back,
    /// the right face
    Right,
    /// the left face
    Left,
    /// the upper face
    Up,
    /// the face opposite to the upper face
    Down,
    /// the middle layer parallel to the Right and Left faces
    Middle,
    /// the middle layer parallel to the Up and Down faces
    Equator,
    /// the middle layer parallel to the Front and Back faces
    Side
}

/// all possible amplitudes for a move
#[derive(IntoEnumIterator, Copy, Clone, PartialEq, Debug)]
pub enum Amplitude
{
    /// 90° turn clockwise
    Clockwise,
    /// 180° turn
    Fullturn,
    /// 90° turn counter-clockwise
    Counterclockwise
}

impl Amplitude
{
    /// returns the number of 90° clockwise rotations that should be applied to obtain the given amplitude
    pub fn nb_rotations(&self) -> usize
    {
        match self
        {
            Amplitude::Clockwise => 1,
            Amplitude::Fullturn => 2,
            Amplitude::Counterclockwise => 3
        }
    }
}

/// describes all possible moves
#[derive(Copy, Clone, Debug)]
pub struct MoveDescription
{
    pub kind: MoveKind,
    pub amplitude: Amplitude
}

//-----------------------------------------------------------------------------
// Move

/// a way to twist the cube
/// note that this particular representation takes some memory,
/// MoveDescription are more suited if one just want to recall a move
pub struct Move
{
    /// unique identifier for the move
    pub description: MoveDescription,
    /// a permutation table, precomputed to speed up move computation
    pub permutation: [usize; NB_SQUARES_CUBE]
}

impl Move
{
    /// takes a move description and compiles it down to a permutation table
    /// NOTE: this step is too expensive to be run whenever a move needs to be applied, instead it is meant as a preparation step
    fn new(kind: MoveKind, amplitude: Amplitude) -> Move
    {
        // builds the description of the move
        let description = MoveDescription { kind, amplitude };
        // generate the associated permutation table
        let mut permutation: [usize; NB_SQUARES_CUBE] = [0; NB_SQUARES_CUBE];
        for (index, result) in permutation.iter_mut().enumerate()
        {
            // new index obtained once we apply the move
            let new_index = Coordinate1D::new(index).apply_move(&description).x;
            *result = new_index;
        }
        Move { description, permutation }
    }

    /// returns a vector containing all possible moves
    pub fn all_moves() -> Vec<Move>
    {
        MoveKind::into_enum_iter().flat_map(|kind| {
                                      Amplitude::into_enum_iter().map(move |amplitude| {
                                                                     Move::new(kind, amplitude)
                                                                 })
                                  })
                                  .collect()
    }

    /// returns the new coordinate obtained after applying the move
    pub fn apply(&self, coordinate1D: usize) -> usize
    {
        // applies the permutation
        self.permutation[coordinate1D]
    }
}
