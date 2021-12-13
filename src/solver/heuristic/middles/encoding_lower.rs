use serde::{Serialize, Deserialize};
use crate::cube::{Cube, Color, NB_FACES, NB_SQUARES_CUBE};
use crate::cube::coordinates::{Coordinate3D, RotationAxis};
use super::super::permutations::{nb_partial_permutations, decimal_from_partial_permutation,
                             partial_permutation_from_decimal};

/// used to turn a cube into a single, unique and consecutiv, middles code
/// and back again
#[derive(Serialize, Deserialize)]
pub struct MiddleEncoder
{
    /// turns a pair index into a middle index and an orientation index
    #[serde(with = "serde_arrays")]
    middle_and_orientation_of_color_pair_table: [(u8, usize); Self::NB_COLOR_PAIRS],
    /// turns a middle index and an orientation index into a pair of colors
    color_pair_of_middle_and_orientation_table: [(Color, Color); Self::NB_LEGAL_COLOR_TRIPLETS],
    /// 1D coordinates of the faces making each middle
    middles_1D_indexes: [(usize, usize); Self::NB_MIDDLES]
}

impl MiddleEncoder
{
    //-------------------------------------------------------------------------
    // CONSTANTS

    /// number of middles that will be kept
    const NB_MIDDLES_KEPT: usize = 6;

    /// number of middles
    const NB_MIDDLES: usize = 12;

    /// number of possible orientations for a middle
    const NB_ORIENTATIONS: usize = 2;

    /// number of different colors
    const NB_COLORS: usize = NB_FACES;

    /// number of possible pair of colors
    const NB_COLOR_PAIRS: usize = Self::NB_COLORS * Self::NB_COLORS;

    /// number of legal, present on actual cubes, triplet of colors
    const NB_LEGAL_COLOR_TRIPLETS: usize = Self::NB_ORIENTATIONS * Self::NB_MIDDLES;

    //-------------------------------------------------------------------------
    // PRECOMPUTATION

    /// turns a triplet of colors into an index
    fn index_of_color_pair(c1: Color, c2: Color) -> usize
    {
        let i1 = c1 as usize;
        let i2 = c2 as usize;
        i1 + Self::NB_COLORS * i2
    }

    /// turns a triplet (middle_index, orientation_index) into an index
    fn index_of_middle_orientation(middle_index: u8, orientation_index: usize) -> usize
    {
        (middle_index as usize) * Self::NB_ORIENTATIONS + orientation_index
    }

    /// computes a table which associate the index of a color triplet (representing a middle) with a middle index and an orientation
    fn compute_tables(
        )
        -> ([(u8, usize); Self::NB_COLOR_PAIRS], [(Color, Color); Self::NB_LEGAL_COLOR_TRIPLETS])
    {
        // all possible pairs of colors making a middle
        let middle_pairs = vec![(Color::Orange, Color::Green),
                                (Color::Green, Color::Red),
                                (Color::Red, Color::Blue),
                                (Color::Blue, Color::Orange),
                                (Color::White, Color::Green),
                                (Color::Green, Color::Yellow),
                                (Color::Yellow, Color::Blue),
                                (Color::Blue, Color::White),
                                (Color::Orange, Color::White),
                                (Color::White, Color::Red),
                                (Color::Red, Color::Yellow),
                                (Color::Yellow, Color::Orange)];

        // builds the table
        let mut t2co = [(0, 0); Self::NB_COLOR_PAIRS];
        let mut co2t = [(Color::Invalid, Color::Invalid); Self::NB_LEGAL_COLOR_TRIPLETS];
        for (middle_index, (c1, c2)) in middle_pairs.into_iter().enumerate()
        {
            // all possible permutations of the tree colors
            let middle_index = middle_index as u8;

            let index = MiddleEncoder::index_of_color_pair(c1, c2);
            t2co[index] = (middle_index, 0);
            let index = MiddleEncoder::index_of_middle_orientation(middle_index, 0);
            co2t[index] = (c1, c2);

            let index = MiddleEncoder::index_of_color_pair(c2, c1);
            t2co[index] = (middle_index, 1);
            let index = MiddleEncoder::index_of_middle_orientation(middle_index, 1);
            co2t[index] = (c2, c1);
        }

        (t2co, co2t)
    }

    /// list the indexes for all the middles
    /// the faces are given in order left_right, down_up, front_back
    fn compute_middles_1D_indexes() -> [(usize, usize); Self::NB_MIDDLES]
    {
        // all the middle coordinates
        let coordinates = [(1, 0, 0),
                           (1, 2, 0),
                           (1, 0, 2),
                           (1, 2, 2),
                           (0, 1, 0),
                           (2, 1, 0),
                           (0, 1, 2),
                           (2, 1, 2),
                           (0, 0, 1),
                           (2, 0, 1),
                           (0, 2, 1),
                           (2, 2, 1)];

        // turns 3D middle coordinates into 1D faces coordinates
        let mut middles_coordinates = [(0, 0); Self::NB_MIDDLES];
        for ((lr, du, fb), middle_result) in coordinates.into_iter().zip(middles_coordinates.iter_mut())
        {
            // uses the correct combination of faces
            if lr == 1
            {
                let c_du = Coordinate3D::new(lr, du, fb, RotationAxis::DownUp).to_1D().x;
                let c_fb = Coordinate3D::new(lr, du, fb, RotationAxis::FrontBack).to_1D().x;
                *middle_result = (c_du, c_fb);
            }
            else if du == 1
            {
                let c_lr = Coordinate3D::new(lr, du, fb, RotationAxis::LeftRight).to_1D().x;
                let c_fb = Coordinate3D::new(lr, du, fb, RotationAxis::FrontBack).to_1D().x;
                *middle_result = (c_lr, c_fb);
            }
            else if fb == 1
            {
                let c_lr = Coordinate3D::new(lr, du, fb, RotationAxis::LeftRight).to_1D().x;
                let c_du = Coordinate3D::new(lr, du, fb, RotationAxis::DownUp).to_1D().x;
                *middle_result = (c_lr, c_du);
            }
            else
            {
                panic!("This is not a middle");
            }
        }

        middles_coordinates
    }

    //-------------------------------------------------------------------------
    // ENCODER

    /// creates a new MiddleEncoder
    pub fn new() -> MiddleEncoder
    {
        let (middle_and_orientation_of_color_pair_table, color_pair_of_middle_and_orientation_table) =
            MiddleEncoder::compute_tables();
        let middles_1D_indexes = MiddleEncoder::compute_middles_1D_indexes();
        MiddleEncoder { middle_and_orientation_of_color_pair_table,
                        color_pair_of_middle_and_orientation_table,
                        middles_1D_indexes }
    }

    /// returns the number of (consecutive) middle code that can be produced by the encoder
    pub fn nb_middles_code(&self) -> usize
    {
        nb_partial_permutations(Self::NB_MIDDLES_KEPT, Self::NB_MIDDLES)
        * Self::NB_ORIENTATIONS.pow(Self::NB_MIDDLES_KEPT as u32)
    }

    /// takes a cube
    /// gets all of its middles
    /// turn them into pairs (middle index, orientation index)
    /// converts the orientations in a single values
    /// and the coner index in a permutation in a single value
    /// combines both into a single number
    pub fn middles_code_of_cube(&self, cube: &Cube) -> usize
    {
        let mut total_orientation_index = 0;
        let mut permutation = [0; Self::NB_MIDDLES_KEPT]; // (middle_index -> position_index)
        for (i, (i1, i2)) in self.middles_1D_indexes.iter().enumerate()
        {
            let pair_index = MiddleEncoder::index_of_color_pair(cube.squares[*i1], cube.squares[*i2]);
            // we pass the pair if it is invalid (leftover from a partial decoding)
            if pair_index >= Self::NB_COLOR_PAIRS
            {
                continue;
            }
            let (middle_index, orientation_index) =
                self.middle_and_orientation_of_color_pair_table[pair_index];
            let middle_index = middle_index as usize;
            // we only register the middle if it is meant to be kept
            if middle_index < Self::NB_MIDDLES_KEPT
            {
                permutation[middle_index] = i as u8;
                total_orientation_index += orientation_index * Self::NB_ORIENTATIONS.pow(middle_index as u32);
            }
        }
        let permutation_index = decimal_from_partial_permutation(&permutation, Self::NB_MIDDLES);
        permutation_index
        + total_orientation_index * nb_partial_permutations(Self::NB_MIDDLES_KEPT, Self::NB_MIDDLES)
    }

    /// takes a middle code
    /// split it into permutation_index and orientation_index
    /// deduces the middle index and orientation index for each middles
    /// rebuilds the corresponding colors
    /// rebuilds a cube with the proper middles
    pub fn cube_of_middle_code(&self, middles_code: usize) -> Cube
    {
        // splits middles_code into both pieces of information
        let max_permutation_index = nb_partial_permutations(Self::NB_MIDDLES_KEPT, Self::NB_MIDDLES);
        let permutation_index = middles_code % max_permutation_index;
        let mut total_orientation_index = middles_code / max_permutation_index;
        // rebuilds the permutation (middle_index -> position_index)
        let permutation =
            partial_permutation_from_decimal(permutation_index, Self::NB_MIDDLES_KEPT, Self::NB_MIDDLES);
        // rebuilds the cube middle per middle
        let mut squares = [Color::Invalid; NB_SQUARES_CUBE];
        for (middle_index, i) in permutation.iter().enumerate().rev()
        {
            // gets the orientation back
            let orientation_index = total_orientation_index % Self::NB_ORIENTATIONS;
            total_orientation_index /= Self::NB_ORIENTATIONS;
            // gets the color back
            let triplet_index =
                MiddleEncoder::index_of_middle_orientation(middle_index as u8, orientation_index);
            let (c1, c2) = self.color_pair_of_middle_and_orientation_table[triplet_index];
            // gets the index back
            let (i1, i2) = self.middles_1D_indexes[*i as usize];
            // rebuilds the middle
            squares[i1] = c1;
            squares[i2] = c2;
        }

        Cube { squares }
    }
}

/*
this implementation deal with the first NB_MIDDLES_KEPT middles
we would need another one to deal with the last ones
*/
