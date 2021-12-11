use std::collections::HashSet;
use enum_iterator::IntoEnumIterator;
use rand::seq::SliceRandom;
pub mod sizes;
pub mod color;
mod moves;
pub mod coordinates;
mod display;
pub use color::Color;
pub use sizes::{NB_FACES, NB_SQUARES_CUBE, NB_SQUARES_FACE, NB_SQUARES_SIDE};
pub use moves::Move;
pub use coordinates::{Face, Coordinate1D, Coordinate2D, RotationAxis};

//-----------------------------------------------------------------------------
// Cube

/// A Rubik's cube stored as a flat array of colors
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Cube
{
    pub squares: [Color; NB_SQUARES_CUBE]
}

impl Cube
{
    /// produces a new, solved, Rubik's cube
    /// we use the western color scheme as a reference for the colors
    /// https://www.speedsolving.com/wiki/index.php/Western_Color_Scheme
    pub fn solved() -> Cube
    {
        let mut squares = [Color::Invalid; NB_SQUARES_CUBE];
        let mut shift = 0;
        for color in Color::ALL
        {
            for square in squares.iter_mut().skip(shift).take(NB_SQUARES_FACE)
            {
                *square = color;
            }
            shift += NB_SQUARES_FACE;
        }
        Cube { squares }
    }

    /// returns a vector of all possible solved cube
    /// done by rotating a solved cube until all possibilities are reached
    pub fn all_solved_cubes() -> Vec<Cube>
    {
        let mut cubes = vec![Cube::solved()];
        let mut result = HashSet::new();

        while !cubes.is_empty()
        {
            let mut new_cubes = Vec::new();
            for cube in cubes
            {
                for axis in RotationAxis::into_enum_iter()
                {
                    // NOTE: there is probably a way to remove the clone but no need to bother for so few iterations
                    let new_cube = cube.rotate(axis);
                    let is_new = result.insert(new_cube.clone());
                    if is_new
                    {
                        new_cubes.push(new_cube);
                    }
                }
            }
            cubes = new_cubes;
        }

        result.into_iter().collect()
    }

    /// returns true if a rubik's cube is solved
    pub fn is_solved(&self) -> bool
    {
        // checks that all faces are solved, one after the other
        // no need to check the last face as it will be solved if all other faces are solved
        for index_face in 0..(NB_FACES - 1)
        {
            // extracts the face that we are currently checking
            let start_index = index_face * NB_SQUARES_FACE;
            let end_index = start_index + NB_SQUARES_FACE;
            let face = &self.squares[start_index..end_index];
            // gets the color of the face, all elements should be of this color
            let face_color = face[0];
            // returns early if at least one element is not of the target color
            if face.iter().any(|color| *color != face_color)
            {
                return false;
            }
        }
        true
    }

    /// gets the color at the given 2D coordinates
    pub fn get(&self, face: Face, x: usize, y: usize) -> Color
    {
        let index = Coordinate2D { face, x, y }.to_1D().x;
        self.squares[index]
    }

    /// scrambles the cube a given number of times to produce a new, random, cube
    pub fn scramble(&self, nb_scramble: usize) -> Cube
    {
        let mut rng = rand::thread_rng();
        let mut result = self.clone();
        let moves = Move::all_moves();
        for _i in 0..nb_scramble
        {
            let random_move = moves.choose(&mut rng).unwrap();
            result = result.apply_move(random_move);
        }
        result
    }
}
