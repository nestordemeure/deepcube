use serde::{Serialize, Deserialize};
use crate::cube::{Cube, Color, NB_FACES, NB_SQUARES_CUBE};
use crate::cube::coordinates::{Coordinate3D, RotationAxis};
use super::super::permutations::{nb_permutations, decimal_from_permutation, permutation_from_decimal};

/// used to turn a cube into a single, unique and consecutiv, corners code
/// and back again
#[derive(Serialize, Deserialize)]
pub struct CornerEncoder
{
    /// turns a triplet index into a corner index and an orientation index
    #[serde(with = "serde_arrays")]
    corner_and_orientation_of_color_triplet_table: [(u8, usize); Self::NB_COLOR_TRIPLETS],
    /// turns a corner index and an orientation index into a triplet of colors
    #[serde(with = "serde_arrays")]
    color_triplet_of_corner_and_orientation_table: [(Color, Color, Color); Self::NB_LEGAL_COLOR_TRIPLETS],
    /// 1D coordinates of the faces making each corner
    corners_1D_indexes: [(usize, usize, usize); Self::NB_CORNERS],
    /// whether each corner is a top corner or a bottom corner
    corners_is_top: [bool; Self::NB_CORNERS]
}

impl CornerEncoder
{
    //-------------------------------------------------------------------------
    // CONSTANTS

    /// number of corners
    const NB_CORNERS: usize = 8;

    /// number of possible orientations for a corner
    const NB_ORIENTATIONS: usize = 3; // 3 knowing the corner

    /// number of different colors
    const NB_COLORS: usize = NB_FACES;

    /// number of possible triplet of colors
    const NB_COLOR_TRIPLETS: usize = Self::NB_COLORS * Self::NB_COLORS * Self::NB_COLORS;

    /// number of legal, present on actual cubes, triplet of colors
    const NB_LEGAL_COLOR_TRIPLETS: usize = Self::NB_ORIENTATIONS * 2 * Self::NB_CORNERS;

    //-------------------------------------------------------------------------
    // PRECOMPUTATION

    /// turns a triplet of colors into an index
    fn index_of_color_triplet(c1: Color, c2: Color, c3: Color) -> usize
    {
        let i1 = c1 as usize;
        let i2 = c2 as usize;
        let i3 = c3 as usize;
        i1 + Self::NB_COLORS * (i2 + Self::NB_COLORS * i3)
    }

    /// turns a triplet (corner_index, orientation_index, is_top) into an index
    fn index_of_corner_orientation(corner_index: u8, mut orientation_index: usize, is_top: bool) -> usize
    {
        // there are twice as many orientation if we take the top/bottom aspect into account
        if is_top
        {
            orientation_index += Self::NB_ORIENTATIONS;
        }
        (corner_index as usize) + orientation_index * Self::NB_CORNERS
    }

    /// computes a table which associate the index of a color triplet (representing a corner) with a corner index and an orientation
    fn compute_tables(
        )
        -> ([(u8, usize); Self::NB_COLOR_TRIPLETS], [(Color, Color, Color); Self::NB_LEGAL_COLOR_TRIPLETS])
    {
        // all possible triplets of colors making a corner
        let corner_triplets = vec![(Color::Orange, Color::Green, Color::White),
                                   (Color::White, Color::Green, Color::Red),
                                   (Color::Green, Color::Yellow, Color::Red),
                                   (Color::Orange, Color::Yellow, Color::Green),
                                   (Color::Yellow, Color::Blue, Color::Red),
                                   (Color::Red, Color::Blue, Color::White),
                                   (Color::Orange, Color::White, Color::Blue),
                                   (Color::Blue, Color::Yellow, Color::Orange)];

        // builds the table
        let mut t2co = [(0, 0); Self::NB_COLOR_TRIPLETS];
        let mut co2t = [(Color::Invalid, Color::Invalid, Color::Invalid); Self::NB_LEGAL_COLOR_TRIPLETS];
        for (corner_index, (c1, c2, c3)) in corner_triplets.into_iter().enumerate()
        {
            // all possible permutations of the tree colors
            let corner_index = corner_index as u8;

            let index = CornerEncoder::index_of_color_triplet(c1, c2, c3);
            t2co[index] = (corner_index, 0);
            let index = CornerEncoder::index_of_corner_orientation(corner_index, 0, true);
            co2t[index] = (c1, c2, c3);

            let index = CornerEncoder::index_of_color_triplet(c1, c3, c2);
            t2co[index] = (corner_index, 0);
            let index = CornerEncoder::index_of_corner_orientation(corner_index, 0, false);
            co2t[index] = (c1, c3, c2);

            let index = CornerEncoder::index_of_color_triplet(c2, c1, c3);
            t2co[index] = (corner_index, 1);
            let index = CornerEncoder::index_of_corner_orientation(corner_index, 1, true);
            co2t[index] = (c2, c1, c3);

            let index = CornerEncoder::index_of_color_triplet(c2, c3, c1);
            t2co[index] = (corner_index, 1);
            let index = CornerEncoder::index_of_corner_orientation(corner_index, 1, false);
            co2t[index] = (c2, c3, c1);

            let index = CornerEncoder::index_of_color_triplet(c3, c2, c1);
            t2co[index] = (corner_index, 2);
            let index = CornerEncoder::index_of_corner_orientation(corner_index, 2, true);
            co2t[index] = (c3, c2, c1);

            let index = CornerEncoder::index_of_color_triplet(c3, c1, c2);
            t2co[index] = (corner_index, 2);
            let index = CornerEncoder::index_of_corner_orientation(corner_index, 2, false);
            co2t[index] = (c3, c1, c2);
        }

        (t2co, co2t)
    }

    /// list the indexes for all the corners
    /// the faces are given in order left_right, down_up, front_back
    fn compute_corners_1D_indexes() -> ([(usize, usize, usize); Self::NB_CORNERS], [bool; Self::NB_CORNERS])
    {
        // all the corner coordinates
        let coordinates =
            [(0, 0, 0), (2, 0, 0), (0, 2, 0), (0, 0, 2), (2, 2, 0), (2, 0, 2), (0, 2, 2), (2, 2, 2)];

        // whether each corner is on the top or bottom layer
        let is_top = coordinates.map(|(_lr, du, _fb)| du > 1);

        // turns 3D corner coordinates into 1D faces coordinates
        let mut corners_coordinates = [(0, 0, 0); Self::NB_CORNERS];
        for ((lr, du, fb), corner_result) in coordinates.into_iter().zip(corners_coordinates.iter_mut())
        {
            let c_lr = Coordinate3D::new(lr, du, fb, RotationAxis::LeftRight).to_1D().x;
            let c_du = Coordinate3D::new(lr, du, fb, RotationAxis::DownUp).to_1D().x;
            let c_fb = Coordinate3D::new(lr, du, fb, RotationAxis::FrontBack).to_1D().x;
            *corner_result = (c_lr, c_du, c_fb);
        }

        (corners_coordinates, is_top)
    }

    //-------------------------------------------------------------------------
    // ENCODER

    /// creates a new CornerEncoder
    pub fn new() -> CornerEncoder
    {
        let (corner_and_orientation_of_color_triplet_table, color_triplet_of_corner_and_orientation_table) =
            CornerEncoder::compute_tables();
        let (corners_1D_indexes, corners_is_top) = CornerEncoder::compute_corners_1D_indexes();
        CornerEncoder { corner_and_orientation_of_color_triplet_table,
                        color_triplet_of_corner_and_orientation_table,
                        corners_1D_indexes,
                        corners_is_top }
    }

    /// returns the number of (consecutive) corner code that can be produced by the encoder
    pub fn nb_corners_code(&self) -> usize
    {
        nb_permutations(Self::NB_CORNERS) * Self::NB_ORIENTATIONS.pow(Self::NB_CORNERS as u32)
    }

    /// takes a cube
    /// gets all of its corners
    /// turn them into pairs (corner index, orientation index)
    /// converts the orientations in a single values
    /// and the coner index in a permutation in a single value
    /// combines both into a single number
    pub fn corners_code_of_cube(&self, cube: &Cube) -> usize
    {
        let mut total_orientation_index = 0;
        let mut permutation = [0; Self::NB_CORNERS];
        for (i, (i1, i2, i3)) in self.corners_1D_indexes.iter().enumerate()
        {
            let triplet_index = CornerEncoder::index_of_color_triplet(cube.squares[*i1],
                                                                      cube.squares[*i2],
                                                                      cube.squares[*i3]);
            let (corner_index, orientation_index) =
                self.corner_and_orientation_of_color_triplet_table[triplet_index];
            permutation[i] = corner_index;
            total_orientation_index = total_orientation_index * Self::NB_ORIENTATIONS + orientation_index;
        }
        let permutation_index = decimal_from_permutation(&permutation);
        permutation_index + total_orientation_index * nb_permutations(Self::NB_CORNERS)
    }

    /// takes a corner code
    /// split it into permutation_index and orientation_index
    /// deduces the corner index and orientation index for each corners
    /// rebuilds the corresponding colors
    /// rebuilds a cube with the proper corners
    pub fn cube_of_corner_code(&self, corners_code: usize) -> Cube
    {
        // splits corners_code into both pieces of information
        let max_permutation_index = nb_permutations(Self::NB_CORNERS);
        let permutation_index = corners_code % max_permutation_index;
        let mut total_orientation_index = corners_code / max_permutation_index;
        // rebuilds the permutation
        let permutation = permutation_from_decimal(permutation_index, Self::NB_CORNERS);
        // rebuilds the cube corner per corner
        let mut squares = [Color::Invalid; NB_SQUARES_CUBE];
        for (i, (i1, i2, i3)) in self.corners_1D_indexes.iter().enumerate().rev()
        {
            // gets corner_index, orientation_index and is_top back
            let corner_index = permutation[i];
            let orientation_index = total_orientation_index % Self::NB_ORIENTATIONS;
            total_orientation_index /= Self::NB_ORIENTATIONS;
            let is_top = self.corners_is_top[i];
            // gets the color back
            let triplet_index =
                CornerEncoder::index_of_corner_orientation(corner_index, orientation_index, is_top);
            let (c1, c2, c3) = self.color_triplet_of_corner_and_orientation_table[triplet_index];
            // rebuilds the corner
            squares[*i1] = c1;
            squares[*i2] = c2;
            squares[*i3] = c3;
        }

        Cube { squares }
    }
}
