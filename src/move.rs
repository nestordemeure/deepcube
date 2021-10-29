//! Represents ways to twist a cube
//! faces are numbered 0,1,2,3,4,5 for front,right,back,left,top,bottom
use crate::cube::{SIZE_SIDE, SIZE_CUBE, Cube};

/// a move can be along a row, a column or a plane (flat and orhogonal to both)
enum MoveKind
{
    Row,
    Col,
    Plane
}

/// describes a move as a shift by 1|2|3 along the row|col|plane at the given index
struct MoveDescription
{
    kind: MoveKind,
    shift_magnitude: i8,
    index: usize
}

/// a move is represented as a permutation table, mapping index from one cube to a new twisted cube
/// moves are expensive to create but can then be reused everytime one wants to apply them
struct Move
{
    indexes: [usize; SIZE_CUBE],
    description: MoveDescription
}

impl Move
{
    /// creates a new move of the given kind, magnitude (+-1,2,3) and index (0,1,2)
    /// note that this function is not optimized for speed
    /// rather, it is designed to build a move that will be reused
    fn new(kind: MoveKind, shift_magnitude: i8, index: usize) -> Move
    {
        // initialize the result with the identity
        let mut result = [0, SIZE_CUBE];
        for i in 0..SIZE_CUBE
        {
            indexes[i] = i;
        }
        // number of face around one rotation axis
        for old_face in 0..4
        {
            // number of elemnts in a row/column
            for idx in 0..SIZE_SIDE
            {
                match kind
                {
                    Row =>
                    {
                        let row = index;
                        let faces = [0, 1, 2, 3];
                        let new_face = (SIZE_SIDE + old_face + shift_magnitude) % SIZE_SIDE;
                        result.indexes[Cube::flat_index(faces[old_face], row, idx)] =
                            Cube::flat_index(faces[new_face], row, idx);
                    }
                    Col =>
                    {
                        let col = index;
                        let faces = [0, 4, 2, 5];
                        let new_face = (SIZE_SIDE + old_face + shift_magnitude) % SIZE_SIDE;
                        result.indexes[Cube::flat_index(faces[old_face], idx, col)] =
                            Cube::flat_index(faces[new_face], idx, col);
                    }
                    Plane =>
                    {
                        let col = index;
                        let faces = [1, 4, 3, 5];
                        let new_face = (SIZE_SIDE + old_face + shift_magnitude) % SIZE_SIDE;
                        result.indexes[Cube::flat_index(faces[old_face], idx, col)] =
                            Cube::flat_index(faces[new_face], idx, col);
                    }
                }
            }
        }
        result
    }

    /// takes a cube and produces a new, twisted, cube by applying the move
    fn apply(&self, cube: &Cube) -> Cube
    {
        // applies the permutation
        let mut squares: [Color; SIZE_CUBE] = [Color::Invalid; SIZE_CUBE];
        for i in 0..SIZE_CUBE
        {
            squares[i] = cube.square[self.indexes[i]];
        }
        // returns the new square
        Cube { squares }
    }
}
