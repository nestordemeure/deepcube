use std::collections::BTreeSet;
use crate::cube::{Cube, color::NB_COLORS, NB_SQUARES_FACE, NB_FACES};

//-----------------------------------------------------------------------------
// CUBE

/// type used as a unique identifier for cubes
type CubeIdentifier = u128;

impl Cube
{
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

            // computes the multiplication factor uniquely identifying the center color
            let face_shift = nb_colors_u128.pow(nb_squares_per_face * rotated_index_face);

            // assemble
            result += (result_face as u128) * face_shift;
        }
        result
    }
}

//-----------------------------------------------------------------------------
// CUBESET

/// datastructure used to check whether a cube has already been encountered
pub struct CubeSet
{
    set: BTreeSet<CubeIdentifier>
}

impl CubeSet
{
    /// creates a new CubeSet
    pub fn new() -> CubeSet
    {
        let set = BTreeSet::new();
        CubeSet { set }
    }

    /// tries to add a cube to the set
    /// returns false if the cube or an isomorphic cube was already in the set
    /// NOTE: this function computes a cube identifier, it might be faster to compute it separetely if it will be recycled
    pub fn insert(&mut self, cube: &Cube) -> bool
    {
        let id = cube.to_identifier();
        self.insert_identifier(id)
    }

    /// tries to add a cube identifier to the set
    /// returns false if the cube or an isomorphic cube was already in the set
    pub fn insert_identifier(&mut self, cube_identifier: CubeIdentifier) -> bool
    {
        self.set.insert(cube_identifier)
    }
}
