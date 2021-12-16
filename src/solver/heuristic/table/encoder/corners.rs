use serde::{Serialize, Deserialize};
use crate::cube::{Cube, Color, NB_FACES};
use crate::cube::coordinates::{Coordinate3D, RotationAxis};
use super::super::permutations::{nb_permutations, decimal_from_permutation};
use super::Encoder;

/// used to turn a cube into a single, unique and consecutiv, corners code
/// and back again
#[derive(Serialize, Deserialize)]
pub struct CornerEncoder
{
    /// turns a triplet index into a corner index and an orientation index
    #[serde(with = "serde_arrays")]
    corner_and_orientation_of_color_triplet_table: [(u8, usize); Self::NB_COLOR_TRIPLETS],
    /// 1D coordinates of the faces making each corner
    corners_1D_indexes: [(usize, usize, usize); Self::NB_CORNERS]
}

impl Encoder for CornerEncoder
{
    /// initializes the encoder
    fn new() -> Self
    {
        let corner_and_orientation_of_color_triplet_table = CornerEncoder::compute_table_corner_of_triplet();
        let corners_1D_indexes = CornerEncoder::compute_corners_1D_indexes();
        CornerEncoder { corner_and_orientation_of_color_triplet_table, corners_1D_indexes }
    }

    /// size of the array in which to put the indexes
    fn nb_indexes() -> usize
    {
        nb_permutations(Self::NB_CORNERS) * Self::NB_ORIENTATIONS.pow(Self::NB_CORNERS as u32)
    }

    /// takes a cube
    /// gets all of its corners
    /// turn them into pairs (corner index, orientation index)
    /// converts the orientations in a single values
    /// and the coner index in a permutation in a single value
    /// combines both into a single number
    fn encode(&self, cube: &Cube) -> usize
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
    fn compute_table_corner_of_triplet() -> [(u8, usize); Self::NB_COLOR_TRIPLETS]
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
        for (corner_index, (c1, c2, c3)) in corner_triplets.into_iter().enumerate()
        {
            // all possible permutations of the tree colors
            let corner_index = corner_index as u8;
            let index = CornerEncoder::index_of_color_triplet(c1, c2, c3);
            t2co[index] = (corner_index, 0);
            let index = CornerEncoder::index_of_color_triplet(c1, c3, c2);
            t2co[index] = (corner_index, 0);
            let index = CornerEncoder::index_of_color_triplet(c2, c1, c3);
            t2co[index] = (corner_index, 1);
            let index = CornerEncoder::index_of_color_triplet(c2, c3, c1);
            t2co[index] = (corner_index, 1);
            let index = CornerEncoder::index_of_color_triplet(c3, c2, c1);
            t2co[index] = (corner_index, 2);
            let index = CornerEncoder::index_of_color_triplet(c3, c1, c2);
            t2co[index] = (corner_index, 2);
        }

        t2co
    }

    /// list the indexes for all the corners
    /// the faces are given in order left_right, down_up, front_back
    fn compute_corners_1D_indexes() -> [(usize, usize, usize); Self::NB_CORNERS]
    {
        // all the corner coordinates
        let coordinates =
            [(0, 0, 0), (2, 0, 0), (0, 2, 0), (0, 0, 2), (2, 2, 0), (2, 0, 2), (0, 2, 2), (2, 2, 2)];

        // turns 3D corner coordinates into 1D faces coordinates
        let mut corners_coordinates = [(0, 0, 0); Self::NB_CORNERS];
        for ((lr, du, fb), corner_result) in coordinates.into_iter().zip(corners_coordinates.iter_mut())
        {
            let c_lr = Coordinate3D::new(lr, du, fb, RotationAxis::LeftRight).to_1D().x;
            let c_du = Coordinate3D::new(lr, du, fb, RotationAxis::DownUp).to_1D().x;
            let c_fb = Coordinate3D::new(lr, du, fb, RotationAxis::FrontBack).to_1D().x;
            *corner_result = (c_lr, c_du, c_fb);
        }

        corners_coordinates
    }
}
