//! Flat representation for a Rubik's cube
use crate::cube::Cube;
use super::super::color::Color;
use super::super::sizes::NB_SQUARES_CUBE;

/// A Rubik's cube stored as a flat array of colors
#[derive(Clone, Debug)]
pub struct FlatCube
{
    pub squares: [Color; NB_SQUARES_CUBE]
}

impl FlatCube
{
    /// takes a FlatCube and returns a Cube
    fn unflatten(&self) -> Cube
    {
        // the cube in which we will store the result
        let mut result = Cube::solved();
        // insures that the vector starts sorted
        debug_assert!(result.blocks.is_sorted_by_key(|block| block.position));
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

    /*/// produces a new, solved, Rubik's cube
    fn solved() -> FlatCube
    {
        let mut squares: [Color; NB_SQUARES_CUBE] = [Color::Invalid; NB_SQUARES_CUBE];
        for i in 0..SIZE_CUBE
        {
            squares[i] = Color::ALL[i / SIZE_FACE];
        }
        FlatCube { squares }
    }*/

    /*/// takes a cube and produces a new cube with color switched such that the center of the first face is of the first color, etc
    /// this let us ignore orientation further in the code
    /// NOTE: it is not equivalent to rotating the cube into a standard orientation
    fn normalize_orientation(&self) -> Cube
    {
        // builds a mapping to turn colors into expected colors
        // uses the fact that we know that the center of each face will be of a different color
        let mut color_mapping = [Color::Invalid; NB_FACES];
        for face in 0..NB_FACES
        {
            let color_center_face = self.get(face, SIZE_SIDE / 2, SIZE_SIDE / 2);
            let expected_color = Color::ALL[face];
            color_mapping[color_center_face as usize] = expected_color;
        }
        // maps the squares
        let mut squares: [Color; SIZE_CUBE] = [Color::Invalid; SIZE_CUBE];
        for i in 0..SIZE_CUBE
        {
            let color = self.squares[i];
            squares[i] = color_mapping[color as usize]
        }
        // returns the new square
        Cube { squares }
    }*/
}
