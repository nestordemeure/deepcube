//! Represents ways to twist a cube
//!
//! We convert classical notations in permutation tables
//!
//! See this website for the classical notations:
//! http://www.rubiksplace.com/move-notations/
use crate::cube::color::Color;

use super::super::moves::{Move, MoveKind, Amplitude};
use super::super::sizes::{NB_SQUARES_CUBE, NB_SQUARES_SIDE};
use super::super::{Cube, Block, Coordinate3D};

//---------------------------------------------------------------------------------------
// IndexCube

/// a 3D cube (vector of blocks) where the colors are replaced by the indices of the square in a flat cube
struct IndexCube
{
    pub blocks: Vec<Block<i32>>
}

impl IndexCube
{
    /// builds a cube where each square contains the corresponding index in the flat representation
    pub fn new() -> IndexCube
    {
        let mut index_square = 0;
        // returns the current square index
        let mut get_index = |has_face| {
            let mut result = -1;
            if has_face
            {
                result = index_square;
                index_square += 1;
            }
            result
        };
        // builds blocks of indices
        let blocks = Cube::solved().blocks
                                   .into_iter()
                                   .map(|block| {
                                       let right_left = get_index(block.color.right_left != Color::Invalid);
                                       let top_down = get_index(block.color.top_down != Color::Invalid);
                                       let front_back = get_index(block.color.front_back != Color::Invalid);
                                       let color = Coordinate3D { right_left, top_down, front_back };
                                       Block { color, position: block.position }
                                   })
                                   .collect();
        IndexCube { blocks }
    }

    /// takes a move and produces a new, twisted, cube by applying the move
    pub fn apply_move(&self, m: Move) -> IndexCube
    {
        // applies the move to all blocks
        let mut blocks: Vec<Block<i32>> = self.blocks.iter().map(|block| block.apply_move(m)).collect();
        // sorts the blocks by position to insure reproducibility
        // this is especially important to make sure flatten always behave identically
        // (one could move the sort into flatten but it would be inelegant and this function is not efficient anyway)
        blocks.sort_unstable_by_key(|block| block.position);
        IndexCube { blocks }
    }

    /// takes a move and produces a new, twisted, cube by applying the move
    /// however, preserves orientation when applying a middle layer rotation
    pub fn apply_move_orientation_preserving(&self, m: Move) -> IndexCube
    {
        // rotates the parallel layers instead of the middle layer
        let (kind1, kind2) = match m.kind
        {
            // the middle layer parallel to the Right and Left faces
            MoveKind::Middle => (MoveKind::Right, MoveKind::Left),
            // the middle layer parallel to the Up and Down faces
            MoveKind::Equator => (MoveKind::Up, MoveKind::Down),
            // the middle layer parallel to the Front and Back faces
            MoveKind::Side => (MoveKind::Front, MoveKind::Back),
            // any non middle layer, we can just apply as usual
            _ => return self.apply_move(m)
        };
        // converts clockwise motion into counterclockwise motion
        let amplitude = match m.amplitude
        {
            Amplitude::Clockwise => Amplitude::Counterclockwise,
            Amplitude::Fullturn => Amplitude::Fullturn,
            Amplitude::Counterclockwise => Amplitude::Clockwise
        };
        // applies the two moves
        let move1 = Move { kind: kind1, amplitude };
        let move2 = Move { kind: kind2, amplitude };
        self.apply_move(move1).apply_move(move2)
    }

    /// flattens an IndexCube into a permutation
    /// the permutation is equivalent to applying all the moves that were applied to the cube
    pub fn to_permutation(&self) -> [usize; NB_SQUARES_CUBE]
    {
        let mut permutation: [usize; NB_SQUARES_CUBE] = [0; NB_SQUARES_CUBE];
        let mut index_square = 0;
        for colors in self.blocks.iter().map(|block| block.color)
        {
            let rl = colors.right_left;
            if rl != -1
            {
                permutation[rl as usize] = index_square;
                index_square += 1;
            }
            let td = colors.top_down;
            if td != -1
            {
                permutation[td as usize] = index_square;
                index_square += 1;
            }
            let fb = colors.front_back;
            if fb != -1
            {
                permutation[fb as usize] = index_square;
                index_square += 1;
            }
        }
        permutation
    }
}

//---------------------------------------------------------------------------------------
// FlatMove

/// move compiled into a permutation table
/// the compilation step is expensive but needs to be run only once
pub struct FlatMove
{
    pub description: Move,
    pub permutation: [usize; NB_SQUARES_CUBE]
}

impl FlatMove
{
    /// returns a vector containing all possible moves
    pub fn all_moves() -> Vec<FlatMove>
    {
        Move::all_moves().into_iter().map(FlatMove::new).collect()
    }

    /// returns a vector containing all possible moves
    /// expressed in an orientation preserving way
    pub fn all_moves_orientation_preserving() -> Vec<FlatMove>
    {
        Move::all_moves().into_iter().map(FlatMove::new_orientation_preserving).collect()
    }

    /// takes a move and compiles it down to a permutation table (a CompiledMove)
    /// NOTE: this step is too expensive to be run whenever a move needs to be applied, instead it is meant as a preparation step
    fn new(description: Move) -> FlatMove
    {
        let permutation = IndexCube::new().apply_move(description).to_permutation();
        FlatMove { description, permutation }
    }

    /// takes a move and compiles it down to a permutation table (a CompiledMove)
    /// however, preserves orientation when applying a middle layer rotation
    /// NOTE: this step is too expensive to be run whenever a move needs to be applied, instead it is meant as a preparation step
    fn new_orientation_preserving(description: Move) -> FlatMove
    {
        let permutation = IndexCube::new().apply_move_orientation_preserving(description).to_permutation();
        FlatMove { description, permutation }
    }
}
