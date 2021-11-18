//! Describes a Rubik's cube

mod color;
use color::{Color, NB_COLORS};
mod moves;

/// A Rubik's cube
#[derive(Clone, Debug)]
pub struct Cube
{
    squares: [Color; SIZE_CUBE]
}

impl Cube
{
    /// produces a new, solved, Rubik's cube
    fn solved() -> Cube
    {
        let mut squares: [Color; SIZE_CUBE] = [Color::Invalid; SIZE_CUBE];
        for i in 0..SIZE_CUBE
        {
            squares[i] = Color::ALL[i / SIZE_FACE];
        }
        Cube { squares }
    }

    /// takes a cube and produces a new cube with color switched such that the center of the first face is of the first color, etc
    /// this let us ignore orientation further in the code
    /// NOTE: it is not equivalent to rotate the cube into a standard orientation
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
    }

    /// takes a face, row and column number, outputs a flat index
    pub fn flat_index(face: usize, row: usize, col: usize) -> usize
    {
        debug_assert!(face < SIZE_FACE);
        debug_assert!(row < SIZE_SIDE);
        debug_assert!(col < SIZE_SIDE);
        face * SIZE_FACE + row * SIZE_SIDE + col
    }

    /// gets the color of the square at the given index
    /// NOTE: the indexing used here is less effective than working directly on the flat array
    pub fn get(&self, face: usize, row: usize, col: usize) -> Color
    {
        self.squares[Cube::flat_index(face, row, col)]
    }
}
