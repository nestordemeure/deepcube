use rand::seq::SliceRandom;
use ansi_term::{Style, Colour::Black};
pub mod sizes;
mod color;
mod moves;
mod coordinates;
pub use color::Color;
pub use sizes::{NB_FACES, NB_SQUARES_CUBE, NB_SQUARES_FACE, NB_SQUARES_SIDE};
pub use moves::Move;
use coordinates::{Face, Coordinate2D};

//-----------------------------------------------------------------------------
// Cube

/// A Rubik's cube stored as a flat array of colors
#[derive(Clone)]
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

    /// takes a move and produces a new, twisted, cube by applying the move
    pub fn apply_move(&self, m: &Move) -> Cube
    {
        let mut squares = [Color::Invalid; NB_SQUARES_CUBE];
        for (index, color) in self.squares.iter().cloned().enumerate()
        {
            squares[m.apply(index)] = color;
        }
        // insures that we have covered all colors
        debug_assert!(squares.iter().all(|c| *c != Color::Invalid));
        Cube { squares }
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

    /// turns the cube into an array of references to faces ordered according to the default color order
    /// this is useful for transformation (such as hashing)
    /// as it insures that the transformation is not impacted by the orientation of the cube
    fn get_oriented_faces(&self) -> [&[Color]; NB_FACES]
    {
        let mut faces: [&[Color]; NB_FACES] = [&self.squares[..]; NB_FACES];
        for index_face in 0..NB_FACES
        {
            // extracts the face
            let start_index = index_face * NB_SQUARES_FACE;
            let end_index = start_index + NB_SQUARES_FACE;
            let face = &self.squares[start_index..end_index];
            // gets the color of the center element of the face
            let color_center = face[NB_SQUARES_SIDE + NB_SQUARES_SIDE / 2];
            // saves the face, indexing it according to its center color
            faces[color_center as usize] = face;
        }
        faces
    }

    /// returns a pair of array
    /// the first one is the colors of the 4 corners of all the faces
    /// the second one is the colors of the 4 middles of all the faces
    /// both are extracted after putting the cube in a normalized orientation
    /// this information is used by some heuristics
    pub fn get_corners_middles(&self) -> ([Color; NB_FACES * 4], [Color; NB_FACES * 4])
    {
        let mut result_corners = [Color::Invalid; NB_FACES * 4];
        let mut result_middles = [Color::Invalid; NB_FACES * 4];
        for index_face in 0..NB_FACES
        {
            // extracts the face
            let start_index = index_face * NB_SQUARES_FACE;
            let end_index = start_index + NB_SQUARES_FACE;
            let face = &self.squares[start_index..end_index];
            // gets the color of the center of the face, its identifier
            let center_color_index = face[4] as usize;
            // gets the results corresponding to this identifier
            let result_corners_face = &mut result_corners[center_color_index * 4..];
            let result_middles_face = &mut result_middles[center_color_index * 4..];
            // stores the four corners
            result_corners_face[0] = face[0];
            result_corners_face[1] = face[2];
            result_corners_face[2] = face[6];
            result_corners_face[3] = face[8];
            // stores the four middles
            result_middles_face[0] = face[1];
            result_middles_face[1] = face[3];
            result_middles_face[2] = face[5];
            result_middles_face[3] = face[7];
        }
        (result_corners, result_middles)
    }

    /// scrambles the cube a given number of times to produce a new, random, cube
    fn scramble(&self, nb_scramble: usize) -> Cube
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

    /// displays the cube in the shell
    pub fn display(&self)
    {
        // displays the square at a given 2D coordinate
        let display_square = |face: Face, x: usize, y: usize| {
            let color = self.get(face, x, y).to_shell_color();
            let text = if (x == 1) && (y == 1)
            {
                format!("{} ", face.to_single_letter_string())
            }
            else
            {
                "  ".to_string()
            };
            let colored_text = Style::new().on(color).fg(Black).bold().paint(text);
            print!("{}", colored_text);
        };
        // displays top squares
        for y in (0..NB_SQUARES_SIDE).rev()
        {
            print!("      "); // empty line
            for x in 0..NB_SQUARES_SIDE
            {
                display_square(Face::Up, x, y);
            }
            println!();
        }
        // displays middle squares
        for y in (0..NB_SQUARES_SIDE).rev()
        {
            for face in [Face::Left, Face::Front, Face::Right, Face::Back]
            {
                for x in 0..NB_SQUARES_SIDE
                {
                    display_square(face, x, y);
                }
            }
            println!();
        }
        // displays bottom squares
        for y in (0..NB_SQUARES_SIDE).rev()
        {
            print!("      "); // empty line
            for x in 0..NB_SQUARES_SIDE
            {
                display_square(Face::Down, x, y);
            }
            println!();
        }
    }
}
