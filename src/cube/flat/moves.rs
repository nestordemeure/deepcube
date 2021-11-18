//! Represents ways to twist a cube
//!
//! We convert classical notations in permutation tables
//!
//! See this website for the classical notations:
//! http://www.rubiksplace.com/move-notations/
use super::color::Color;
use crate::cube::{SIZE_SIDE, SIZE_CUBE, Cube};

/// move compiled into a permutation table
/// the compilation step is expensive but needs to be run only once
struct CompiledMove
{
    description: Move,
    indexes: [usize; SIZE_CUBE]
}

impl CompiledMove
{
    /// returns a vector containing all possible compiled moves
    fn all_compiled_moves(preserve_orientation: bool) -> Vec<CompiledMove>
    {
        Move::all_moves().into_iter().map(|m| CompiledMove::compile(m, preserve_orientation)).collect()
    }

    /// the identity move, does nothing
    /// used as a basis for other moves
    fn identity() -> CompiledMove
    {
        let mut indexes = [0; SIZE_CUBE];
        for i in 0..SIZE_CUBE
        {
            indexes[i] = i;
        }
        let description = Move { kind: MoveKind::Front, amplitude: Amplitude::Noturn };
        CompiledMove { description, indexes }
    }

    /// takes a move kind and returns the corresponding 90° clockwise turn compiled move
    fn compile_clockwise(kind: MoveKind) -> CompiledMove
    {
        // generates the indexes
        let mut indexes = CompiledMove::identity().indexes;
        unimplemented!("TODO implement the clockwise turn for all moves");
        match kind
        {
            /// the face facing the solver
            MoveKind::Front =>
            {}
            /// the back face
            MoveKind::Back =>
            {}
            /// the right face
            MoveKind::Right =>
            {}
            /// the left face
            MoveKind::Left =>
            {}
            /// the upper face
            MoveKind::Up =>
            {}
            /// the face opposite to the upper face
            MoveKind::Down =>
            {}
            /// the middle layer parallel to the Right and Left faces
            MoveKind::Middle =>
            {}
            /// the middle layer parallel to the Up and Down faces
            MoveKind::Equator =>
            {}
            /// the middle layer parallel to the Front and Back faces
            MoveKind::Side =>
            {}
        }
        // generates the move description
        let description = Move { kind, amplitude: Amplitude::Clockwise };
        CompiledMove { description, indexes }
    }

    /// same as compile_clockwise but preserves orientation when applying a middle layer rotation
    fn compile_clockwise_orientation_preserving(kind: MoveKind) -> CompiledMove
    {
        // one move counterclockwise for each of the parallel layer instead of a a move clockwise
        let (move1, move2) = match kind
        {
            // the middle layer parallel to the Right and Left faces
            MoveKind::Middle => (Move { kind: MoveKind::Right, amplitude: Amplitude::Counterclockwise },
                                 Move { kind: MoveKind::Left, amplitude: Amplitude::Counterclockwise }),
            // the middle layer parallel to the Up and Down faces
            MoveKind::Equator => (Move { kind: MoveKind::Up, amplitude: Amplitude::Counterclockwise },
                                  Move { kind: MoveKind::Down, amplitude: Amplitude::Counterclockwise }),
            // the middle layer parallel to the Front and Back faces
            MoveKind::Side => (Move { kind: MoveKind::Front, amplitude: Amplitude::Counterclockwise },
                               Move { kind: MoveKind::Back, amplitude: Amplitude::Counterclockwise }),
            // any non middle layer
            _ => return CompiledMove::compile_clockwise(kind)
        };
        // compiles the moves and compose the lateral moves
        let move1 = CompiledMove::compile(move1, false);
        let move2 = CompiledMove::compile(move2, false);
        let mut result = move1.compose(&move2);
        // associate the correct description with the move
        result.description = Move { kind, amplitude: Amplitude::Clockwise };
        result
    }

    /// takes a move and compiles it down to a permutation table (a CompiledMove)
    /// if `preserve_orientation` is set to `true`, move to middle layers will instead be counter moves to lateral layers
    /// NOTE: this step is too expensive to be run whenever a move needs to be applied, instead it is meant as a preparation step
    fn compile(m: Move, preserve_orientation: bool) -> CompiledMove
    {
        // generates a 90° clockwise move
        let compiled_move = if preserve_orientation
        {
            CompiledMove::compile_clockwise_orientation_preserving(m.kind)
        }
        else
        {
            CompiledMove::compile_clockwise(m.kind)
        };
        // applies it several time if needed to obtain the correct amplitude
        let mut result = match m.amplitude
        {
            Amplitude::Noturn => CompiledMove::identity(),
            Amplitude::Clockwise => compiled_move,
            Amplitude::Fullturn => compiled_move.compose(&compiled_move), // two 90° turns
            Amplitude::Counterclockwise => compiled_move.compose(&compiled_move).compose(&compiled_move) // three 90° turn
        };
        // sets the correct amplitude for the result
        result.description.amplitude = m.amplitude;
        result
    }

    /// compose with another move to produce a new move
    /// that is the application of the first move followed by the second one
    /// the description is inherited from the first move
    fn compose(&self, second_move: &CompiledMove) -> CompiledMove
    {
        // takes the first move
        let mut indexes = self.indexes;
        // applies the second move on top
        for i in 0..SIZE_CUBE
        {
            indexes[i] = indexes[second_move.indexes[i]];
        }
        CompiledMove { indexes, description: self.description }
    }

    /// takes a cube and produces a new, twisted, cube by applying the move
    /// cube[i] is replaced by cube[index[i]]
    fn apply(&self, cube: &Cube) -> Cube
    {
        // applies the permutation
        let mut squares: [Color; SIZE_CUBE] = [Color::Invalid; SIZE_CUBE];
        for i in 0..SIZE_CUBE
        {
            squares[i] = cube.squares[self.indexes[i]];
        }
        // returns the new square
        Cube { squares }
    }
}
