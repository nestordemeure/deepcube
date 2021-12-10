use crate::cube::{Cube, Color, NB_FACES, NB_SQUARES_CUBE};
use crate::cube::coordinates::{Coordinate3D, RotationAxis};
use lehmer::Lehmer;

/// used to turn a cube into a single, unique and consecutiv, middles code
/// and back again
pub struct MiddleEncoder
{
    /// turns a pair index into a middle index and an orientation index
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
        i1 + Self::NB_COLORS * (i2)
    }

    /// turns a triplet (middle_index, orientation_index) into an index
    fn index_of_middle_orientation(middle_index: u8, mut orientation_index: usize) -> usize
    {
        (middle_index as usize) + orientation_index * Self::NB_MIDDLES
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
        (Lehmer::max_value(Self::NB_MIDDLES) + 1) * Self::NB_ORIENTATIONS.pow(Self::NB_MIDDLES as u32)
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
        let mut permutation = [0; Self::NB_MIDDLES];
        for (i, (i1, i2)) in self.middles_1D_indexes.iter().enumerate()
        {
            let triplet_index = MiddleEncoder::index_of_color_pair(cube.squares[*i1], cube.squares[*i2]);
            let (middle_index, orientation_index) =
                self.middle_and_orientation_of_color_pair_table[triplet_index];
            permutation[i] = middle_index;
            total_orientation_index = total_orientation_index * Self::NB_ORIENTATIONS + orientation_index;
        }
        let permutation_index = Lehmer::from_permutation(&permutation).to_decimal();
        permutation_index + total_orientation_index * (Lehmer::max_value(Self::NB_MIDDLES) + 1)
    }

    /// takes a middle code
    /// split it into permutation_index and orientation_index
    /// deduces the middle index and orientation index for each middles
    /// rebuilds the corresponding colors
    /// rebuilds a cube with the proper middles
    pub fn cube_of_middle_code(&self, middles_code: usize) -> Cube
    {
        // splits middles_code into both pieces of information
        let max_permutation_index = Lehmer::max_value(Self::NB_MIDDLES) + 1;
        let permutation_index = middles_code % max_permutation_index;
        let mut total_orientation_index = middles_code / max_permutation_index;
        // rebuilds the permutation
        let permutation = Lehmer::from_decimal(permutation_index, Self::NB_MIDDLES).to_permutation();
        // rebuilds the cube middle per middle
        let mut squares = [Color::Invalid; NB_SQUARES_CUBE];
        for (i, (i1, i2)) in self.middles_1D_indexes.iter().enumerate().rev()
        {
            // gets middle_index and orientation_index back
            let middle_index = permutation[i];
            let orientation_index = total_orientation_index % Self::NB_ORIENTATIONS;
            total_orientation_index /= Self::NB_ORIENTATIONS;
            // gets the color back
            let triplet_index = MiddleEncoder::index_of_middle_orientation(middle_index, orientation_index);
            let (c1, c2) = self.color_pair_of_middle_and_orientation_table[triplet_index];
            // rebuilds the middle
            squares[*i1] = c1;
            squares[*i2] = c2;
        }

        Cube { squares }
    }
}

/*
Do only some middles in order to preserve memory
*/
