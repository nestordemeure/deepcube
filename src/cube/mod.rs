use std::collections::BTreeSet;
use enum_iterator::IntoEnumIterator;
use rand::seq::SliceRandom;
pub mod sizes;
pub mod color;
pub mod moves;
pub mod coordinates;
mod display;
pub use color::{Color, NB_COLORS};
pub use sizes::{NB_FACES, NB_SQUARES_CUBE, NB_SQUARES_FACE, NB_SQUARES_SIDE};
pub use moves::Move;
pub use coordinates::{Face, Coordinate1D, Coordinate2D, RotationAxis};

//-----------------------------------------------------------------------------
// Cube

/// A Rubik's cube stored as a flat array of colors
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Cube
{
    pub squares: [Color; NB_SQUARES_CUBE]
}

/// type used as a unique identifier for cubes
type CubeIdentifier = u128;

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
        let mut result = BTreeSet::new();

        while !cubes.is_empty()
        {
            let mut new_cubes = Vec::new();
            for cube in cubes
            {
                // NOTE: there is probably a way to remove the clone but no need to bother for so few iterations
                let is_new = result.insert(cube.clone());
                if is_new
                {
                    for axis in RotationAxis::into_enum_iter()
                    {
                        let new_cube = cube.rotate(axis);
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
            if face.iter().skip(1).any(|color| *color != face_color)
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
    pub fn scramble(self, nb_scramble: usize) -> Cube
    {
        let mut rng = rand::thread_rng();
        let mut result = self;
        let moves = Move::all_moves();
        for _i in 0..nb_scramble
        {
            let random_move = moves.choose(&mut rng).unwrap();
            result = result.apply_move(random_move);
        }
        result
    }

    /// converts the cube into a unique identifier
    /// we use the colors of the center squares to put the cube in standard orientation
    /// we do not encode the center square as it is always of the same color
    /// we do not encode the last face as it is entirely deifned by the other faces
    pub fn to_identifier(&self) -> CubeIdentifier
    {
        // constants with the proper types
        let nb_colors_u128 = NB_COLORS as u128;
        let nb_squares_per_face = (NB_SQUARES_FACE - 1) as u32; // we ignore the center square
        let index_face_ignored = (NB_FACES - 1) as u32; // the last face is not encoded

        // looping on all faces
        let mut result: u128 = 0;
        for index_face in 0..NB_FACES
        {
            // extracts the face that we are currently checking
            let start_index = index_face * NB_SQUARES_FACE;
            let end_index = start_index + NB_SQUARES_FACE;
            let face = &self.squares[start_index..end_index];

            // gets the index of the face using its center square
            let rotated_index_face = face[4] as u32;
            // we do not encode the last face as it can be deduced from the other faces
            if rotated_index_face == index_face_ignored
            {
                continue;
            }

            // computes a code uniquely identifying the face
            let mut result_face = 0;
            // 4 squares before center square
            for i in 0..4
            {
                let color_index = face[i] as usize;
                result_face = result_face * NB_COLORS + color_index;
            }
            // 4 squares after center square
            for i in 5..NB_SQUARES_FACE
            {
                let color_index = face[i] as usize;
                result_face = result_face * NB_COLORS + color_index;
            }

            // computes the multiplication factor uniquely identifying the color of the center square
            let face_shift = nb_colors_u128.pow(nb_squares_per_face * rotated_index_face);

            // assemble
            result += (result_face as u128) * face_shift;
        }
        result
    }
}
