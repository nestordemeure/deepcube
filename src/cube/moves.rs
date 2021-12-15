//! Describes ways to twist a cube
//!
//! See this website for the classical notations:
//! http://www.rubiksplace.com/move-notations/
use enum_iterator::IntoEnumIterator;
use super::Cube;
use super::sizes::NB_SQUARES_CUBE;
use super::coordinates::{Coordinate1D, RotationAxis};
use super::color::Color;

//-----------------------------------------------------------------------------
// Move description

/// all the slice of the cube that could move
#[derive(IntoEnumIterator, Copy, Clone, Debug, PartialEq)]
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

impl MoveKind
{
    pub fn is_center_layer(&self) -> bool
    {
        matches!(self, MoveKind::Equator | MoveKind::Middle | MoveKind::Side)
    }
}

impl std::fmt::Display for MoveKind
{
    /// print a MoveKind as a single letter
    fn fmt(&self, formater: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        let c = match self
        {
            MoveKind::Front => 'F',
            MoveKind::Back => 'B',
            MoveKind::Right => 'R',
            MoveKind::Left => 'L',
            MoveKind::Up => 'U',
            MoveKind::Down => 'D',
            MoveKind::Middle => 'M',
            MoveKind::Equator => 'E',
            MoveKind::Side => 'S'
        };
        write!(formater, "{}", c)
    }
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
#[derive(Copy, Clone, PartialEq)]
pub struct MoveDescription
{
    pub kind: MoveKind,
    pub amplitude: Amplitude
}

impl std::fmt::Debug for MoveDescription
{
    /// displays a move description in standard format
    fn fmt(&self, formater: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        write!(formater, "{}{}", self.kind, self.amplitude.nb_rotations())
    }
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

//-----------------------------------------------------------------------------
// Cube

impl Cube
{
    /// takes a move and produces a new, twisted, cube by applying the move
    pub fn apply_move(&self, m: &Move) -> Cube
    {
        let mut squares = [Color::Invalid; NB_SQUARES_CUBE];
        for (index, color) in self.squares.iter().cloned().enumerate()
        {
            squares[m.apply(index)] = color;
        }
        Cube { squares }
    }

    /// applies a full path to a cube
    /// NOTE: this operation is not designed with efficiency in mind
    pub fn apply_path(&self, path: &[MoveDescription]) -> Cube
    {
        let moves = Move::all_moves();
        let mut cube = self.clone();
        for moveDescription in path
        {
            let m = moves.iter().find(|m| m.description == *moveDescription).unwrap();
            cube = cube.apply_move(m);
        }
        cube
    }

    /// rotates the cube along the given axis
    pub fn rotate(&self, axis: RotationAxis) -> Cube
    {
        let mut squares = [Color::Invalid; NB_SQUARES_CUBE];
        for (index, color) in self.squares.iter().cloned().enumerate()
        {
            let new_index = Coordinate1D::new(index).rotate(axis).x;
            squares[new_index] = color;
        }
        Cube { squares }
    }
}
