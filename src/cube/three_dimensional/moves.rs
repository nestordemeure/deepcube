//! Represents ways to twist a cube
//! note that this implementation is not designed for performance but rather focusses on correctness
//!
//! See this website for the classical notations:
//! http://www.rubiksplace.com/move-notations/
use enum_iterator::IntoEnumIterator;
use super::cube::{Block, Coordinate3D, Cube3D};

/// all the slice of the cube that could move
#[derive(IntoEnumIterator, Copy, Clone)]
enum MoveKind
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
    /// takes a 3D position
    /// returns true if the corresponding block should move given the MoveKind
    fn should_move_block(&self, position: &Coordinate3D<usize>) -> bool
    {
        match self
        {
            MoveKind::Front => position.front_back == 0,
            MoveKind::Side => position.front_back == 1,
            MoveKind::Back => position.front_back == 2,
            MoveKind::Right => position.right_left == 0,
            MoveKind::Middle => position.right_left == 1,
            MoveKind::Left => position.right_left == 2,
            MoveKind::Up => position.top_down == 0,
            MoveKind::Equator => position.top_down == 1,
            MoveKind::Down => position.top_down == 2
        }
    }

    /// applies a single 90° clockwise rotation to a block
    /// using the appropriate axis given the MoveKind
    fn rotate_90degree_clockwise(&self, block: &Block) -> Block
    {
        match self
        {
            MoveKind::Front | MoveKind::Side | MoveKind::Back => block.rotate_front_back(),
            MoveKind::Right | MoveKind::Middle | MoveKind::Left => block.rotate_right_left(),
            MoveKind::Up | MoveKind::Equator | MoveKind::Down => block.rotate_top_down()
        }
    }
}

/// all possible amplitudes for a move
#[derive(IntoEnumIterator, Copy, Clone, PartialEq)]
enum Amplitude
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
    fn nb_90degree_clockwise_rotations(&self) -> usize
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
struct Move
{
    kind: MoveKind,
    amplitude: Amplitude
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

    /// takes a block and produces a new, rotated, block by applying the move
    fn apply_block(&self, block: &Block) -> Block
    {
        let mut block = block.clone();
        if self.kind.should_move_block(&block.position)
        {
            for i in 0..self.amplitude.nb_90degree_clockwise_rotations()
            {
                block = self.kind.rotate_90degree_clockwise(&block);
            }
        }
        block
    }

    /// takes a cube and produces a new, twisted, cube by applying the move
    fn apply(&self, cube: &Cube3D) -> Cube3D
    {
        let blocks = cube.blocks.iter().map(|block| self.apply_block(block)).collect();
        Cube3D { blocks }
    }

    /// takes a cube and produces a new, twisted, cube by applying the move
    /// however, preserves orientation when applying a middle layer rotation
    fn apply_orientation_preserving(&self, cube: &Cube3D) -> Cube3D
    {
        // moves the parallel layers instead of the center layer
        let (kind1, kind2) = match self.kind
        {
            // the middle layer parallel to the Right and Left faces
            MoveKind::Middle => (MoveKind::Right, MoveKind::Left),
            // the middle layer parallel to the Up and Down faces
            MoveKind::Equator => (MoveKind::Up, MoveKind::Down),
            // the middle layer parallel to the Front and Back faces
            MoveKind::Side => (MoveKind::Front, MoveKind::Back),
            // any non middle layer, we can just apply as usual
            _ => return self.apply(cube)
        };
        // converts clockwise amplitude into counterclockwise amplitude
        let amplitude = match self.amplitude
        {
            Amplitude::Clockwise => Amplitude::Counterclockwise,
            Amplitude::Fullturn => Amplitude::Fullturn,
            Amplitude::Counterclockwise => Amplitude::Clockwise
        };
        // applies the two moves
        let move1 = Move { kind: kind1, amplitude };
        let move2 = Move { kind: kind2, amplitude };
        move1.apply(&move2.apply(cube))
    }
}
