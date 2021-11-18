//! Describes ways to twist a cube
//!
//! See this website for the classical notations:
//! http://www.rubiksplace.com/move-notations/
use enum_iterator::IntoEnumIterator;

/// all the slice of the cube that could move
#[derive(IntoEnumIterator, Copy, Clone)]
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
#[derive(IntoEnumIterator, Copy, Clone, PartialEq)]
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
#[derive(Copy, Clone)]
pub struct Move
{
    pub kind: MoveKind,
    pub amplitude: Amplitude
}

impl Move
{
    /// returns a vector containing all possible moves
    fn all_moves() -> Vec<Move>
    {
        MoveKind::into_enum_iter().flat_map(|kind| {
                                      Amplitude::into_enum_iter().map(move |amplitude| Move { kind,
                                                                                              amplitude })
                                  })
                                  .collect()
    }
}
