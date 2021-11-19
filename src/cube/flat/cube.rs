//! Flat representation for a Rubik's cube
use crate::cube::Cube;
use super::super::color::Color;
use super::super::sizes::NB_SQUARES_CUBE;
use super::moves::FlatMove;

/// A Rubik's cube stored as a flat array of colors
#[derive(Clone, Debug)]
pub struct FlatCube
{
    pub squares: [Color; NB_SQUARES_CUBE]
}

impl FlatCube
{
    /// produces a new, solved, Rubik's cube
    /// note that this operation isn't as cheap as one might intuit
    fn solved() -> FlatCube
    {
        Cube::solved().flatten()
    }

    /// takes a FlatCube and returns a Cube
    fn unflatten(&self) -> Cube
    {
        // the cube in which we will store the result
        let mut result = Cube::solved();
        // insures that the vector starts sorted
        debug_assert!(result.blocks.is_sorted_by_key(|block| block.position));
        // puts the squares back in the 3D cube
        let squares = self.squares;
        let mut index_square = 0;
        for colors in result.blocks.iter_mut().map(|block| &mut block.color)
        {
            let rl = &mut colors.right_left;
            if *rl != Color::Invalid
            {
                *rl = squares[index_square];
                index_square += 1;
            }
            let td = &mut colors.top_down;
            if *td != Color::Invalid
            {
                *td = squares[index_square];
                index_square += 1;
            }
            let fb = &mut colors.front_back;
            if *fb != Color::Invalid
            {
                *fb = squares[index_square];
                index_square += 1;
            }
        }
        // insures that we used all the squares
        debug_assert!(index_square == NB_SQUARES_CUBE);
        result
    }

    /// takes a move and produces a new, twisted, cube by applying the move
    /// cube[i] is replaced by cube[index[i]]
    fn apply_move(&self, m: &FlatMove) -> FlatCube
    {
        // applies the permutation
        let mut squares: [Color; NB_SQUARES_CUBE] = [Color::Invalid; NB_SQUARES_CUBE];
        for i in 0..NB_SQUARES_CUBE
        {
            squares[i] = self.squares[m.permutation[i]];
        }
        // returns the new square
        FlatCube { squares }
    }
}
