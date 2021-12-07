use crate::cube::{Cube, Color, NB_FACES, NB_SQUARES_CUBE, NB_SQUARES_FACE};

/// 4 color per face compressed into an integer
/// used to minimize memory use in heuristic tables
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MiniCube
{
    // NB_FACES^(nb_colors*4) fits just right in 64 bits
    data: u64
}

impl MiniCube
{
    /// takes an array and changes its colors
    /// building a mapping on the fly
    /// each color encountered is associated with a color in order
    /// NOTE: with this normalization step all final states are equal
    ///       however, it can produce some states that would be impossible with a normal cube
    fn recolor_array(colors: &mut [Color; NB_FACES * 4])
    {
        // mapping from old colors to new colors
        let mut new_colors = [Color::Invalid; NB_FACES];
        // index of the next color to use
        let mut current_color = 0;
        // square by square, goes from old colors to new colors
        for color in colors.iter_mut()
        {
            let color_index = *color as usize;
            match new_colors[color_index]
            {
                Color::Invalid =>
                {
                    let new_color = Color::ALL[current_color];
                    current_color += 1;
                    new_colors[color_index] = new_color;
                    *color = new_color;
                }
                new_color =>
                {
                    *color = new_color;
                }
            }
        }
    }

    /// turns an array into a unique integer
    fn from_array(mut colors: [Color; NB_FACES * 4]) -> MiniCube
    {
        // normalizes colors
        MiniCube::recolor_array(&mut colors);
        // converts colors into a single integer
        let nb_colors = NB_FACES as u64;
        let data = colors.iter()
                         .map(|color| *color as u64) // converts colors into integers
                         .fold(0, |acc, n| acc * nb_colors + n); // combines the integers
        MiniCube { data }
    }

    /// turns the integer back into an array
    /// NOTE: this is not a perfect bijection due to the color normalization done at the previous step
    fn to_array(self) -> [Color; NB_FACES * 4]
    {
        let nb_colors = NB_FACES as u64;
        let mut result = [Color::Invalid; NB_FACES * 4];
        let mut data = self.data;

        for color in result.iter_mut().rev()
        {
            let color_index = (data % nb_colors) as usize;
            *color = Color::ALL[color_index];
            data /= nb_colors;
        }

        result
    }

    /// extracts the colors of the 4 corners of all the faces
    pub fn from_corners(cube: &Cube) -> MiniCube
    {
        let mut result = [Color::Invalid; NB_FACES * 4];

        for index_face in 0..NB_FACES
        {
            // extracts the face
            let start_index = index_face * NB_SQUARES_FACE;
            let end_index = start_index + NB_SQUARES_FACE;
            let face = &cube.squares[start_index..end_index];
            // gets the results corresponding to the face
            let result_face = &mut result[index_face * 4..];
            // stores the four corners
            result_face[0] = face[0];
            result_face[1] = face[2];
            result_face[2] = face[6];
            result_face[3] = face[8];
        }

        MiniCube::from_array(result)
    }

    /// takes a minicube and turns it into a full cube filling the corners with the colors
    /// all other colors are left Invalid
    pub fn to_corners(self) -> Cube
    {
        let corners = self.to_array();
        let mut squares = [Color::Invalid; NB_SQUARES_CUBE];

        for index_face in 0..NB_FACES
        {
            // extracts the face
            let start_index = index_face * NB_SQUARES_FACE;
            let end_index = start_index + NB_SQUARES_FACE;
            let face = &mut squares[start_index..end_index];
            // gets the corners corresponding to the face
            let corners_face = &corners[index_face * 4..];
            // put the four corners back in the face
            face[0] = corners_face[0];
            face[2] = corners_face[1];
            face[6] = corners_face[2];
            face[8] = corners_face[3];
        }

        Cube { squares }
    }

    /// extracts the colors of the 4 middle sides of all the faces
    pub fn from_middles(cube: &Cube) -> MiniCube
    {
        let mut result = [Color::Invalid; NB_FACES * 4];
        for index_face in 0..NB_FACES
        {
            // extracts the face
            let start_index = index_face * NB_SQUARES_FACE;
            let end_index = start_index + NB_SQUARES_FACE;
            let face = &cube.squares[start_index..end_index];
            // gets the results corresponding to the face
            let result_face = &mut result[index_face * 4..];
            // stores the four middles
            result_face[0] = face[1];
            result_face[1] = face[3];
            result_face[2] = face[5];
            result_face[3] = face[7];
        }

        MiniCube::from_array(result)
    }

    /// takes a minicube and turns it into a full cube filling the corners with the colors
    /// all other colors are left Invalid
    pub fn to_middles(self) -> Cube
    {
        let corners = self.to_array();
        let mut squares = [Color::Invalid; NB_SQUARES_CUBE];

        for index_face in 0..NB_FACES
        {
            // extracts the face
            let start_index = index_face * NB_SQUARES_FACE;
            let end_index = start_index + NB_SQUARES_FACE;
            let face = &mut squares[start_index..end_index];
            // gets the middles corresponding to the face
            let corners_face = &corners[index_face * 4..];
            // put the four middles back in the face
            face[1] = corners_face[0];
            face[3] = corners_face[1];
            face[5] = corners_face[2];
            face[7] = corners_face[3];
        }

        Cube { squares }
    }
}

/*
the conversion into corners should include some color normalization to reduce the search space due to its invariances
a corner can be uniquely identified by its triplet of colors (ignoring orientation information)
the we color as we go strategy might be viable: with this strategy all solutions cubes are equivalent
*/
