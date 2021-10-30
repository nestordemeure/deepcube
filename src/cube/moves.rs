//! Represents ways to twist a cube
//!
//! We convert classical notations in permutation tables
//!
//! See this website for the classical notations:
//! http://www.rubiksplace.com/move-notations/
use enum_iterator::IntoEnumIterator;
use super::color::Color;
use crate::cube::{SIZE_SIDE, SIZE_CUBE, Cube};

/// all the slice of the cube that could move
#[derive(IntoEnumIterator, Copy, Clone)]
enum MoveKind
{
    /// the face facing the solver
    Front,
    /// the back face
    Back,
    /// the right face
    Right,
    /// the left face
    Left,
    /// the upper face
    Up,
    /// the face opposite to the upper face
    Down,
    /// the middle layer parallel to the Right and Left faces
    Middle,
    /// the middle layer parallel to the Up and Down faces
    Equator,
    /// the middle layer parallel to the Front and Back faces
    Side
}

/// all possible amplitudes for a move
#[derive(IntoEnumIterator, Copy, Clone)]
enum Amplitude
{
    /// an absence of turn
    Noturn,
    /// 90° turn clockwise
    Clockwise,
    /// 90° turn counter-clockwise
    Counterclockwise,
    /// 180° turn
    Fullturn
}

/// describes all possible moves
#[derive(Copy, Clone)]
struct Move
{
    kind: MoveKind,
    amplitude: Amplitude
}

impl Move
{
    /// returns a vector containing all possible moves
    fn all_moves() -> Vec<Move>
    {
        MoveKind::into_enum_iter().flat_map(|kind| {
                                      Amplitude::into_enum_iter().map(move |amplitude| Move { kind,
                                                                                              amplitude })
                                  })
                                  .collect()
    }

    /// takes a move and compiles it
    /// if `preserve_orientation` is set to `true`, move to middle layers will instead be counter moves to lateral layers
    fn compile(self, preserve_orientation: bool) -> CompiledMove
    {
        CompiledMove::compile(self, preserve_orientation)
    }
}

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
        Move::all_moves().iter().map(|m| m.compile(preserve_orientation)).collect()
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
        match kind
        {
            // the middle layer parallel to the Right and Left faces
            MoveKind::Middle =>
            {
                // one move counterclockwise for each of the parallel layer instead of a a move clockwise
                let move_right =
                    Move { kind: MoveKind::Right, amplitude: Amplitude::Counterclockwise }.compile(false);
                let move_left =
                    Move { kind: MoveKind::Left, amplitude: Amplitude::Counterclockwise }.compile(false);
                let mut result = move_right.compose(&move_left);
                // associate the correct description with the move
                result.description = Move { kind, amplitude: Amplitude::Clockwise };
                result
            }
            // the middle layer parallel to the Up and Down faces
            MoveKind::Equator =>
            {
                // one move counterclockwise for each of the parallel layer instead of a a move clockwise
                let move_up =
                    Move { kind: MoveKind::Up, amplitude: Amplitude::Counterclockwise }.compile(false);
                let move_down =
                    Move { kind: MoveKind::Down, amplitude: Amplitude::Counterclockwise }.compile(false);
                let mut result = move_up.compose(&move_down);
                // associate the correct description with the move
                result.description = Move { kind, amplitude: Amplitude::Clockwise };
                result
            }
            // the middle layer parallel to the Front and Back faces
            MoveKind::Side =>
            {
                // one move counterclockwise for each of the parallel layer instead of a a move clockwise
                let move_front =
                    Move { kind: MoveKind::Front, amplitude: Amplitude::Counterclockwise }.compile(false);
                let move_back =
                    Move { kind: MoveKind::Back, amplitude: Amplitude::Counterclockwise }.compile(false);
                let mut result = move_front.compose(&move_back);
                // associate the correct description with the move
                result.description = Move { kind, amplitude: Amplitude::Clockwise };
                result
            }
            // any non middle layer
            _ => CompiledMove::compile_clockwise(kind)
        }
    }

    /// takes a move and compiles it down to a permutation table (a CompiledMove)
    /// if `preserve_orientation` is set to `true`, move to middle layers will instead be counter moves to lateral layers
    fn compile(m: Move, preserve_orientation: bool) -> CompiledMove
    {
        // generates a 90° clockwise move
        let mut compiled_move = if preserve_orientation
        {
            CompiledMove::compile_clockwise_orientation_preserving(m.kind)
        }
        else
        {
            CompiledMove::compile_clockwise(m.kind)
        };
        // applies it several time if needed to obtain the correct amplitude
        compiled_move.description.amplitude = m.amplitude;
        match m.amplitude
        {
            Amplitude::Noturn => CompiledMove::identity(),
            Amplitude::Clockwise => compiled_move,
            Amplitude::Fullturn => compiled_move.compose(&compiled_move), // two 90° turns
            Amplitude::Counterclockwise => compiled_move.compose(&compiled_move).compose(&compiled_move) // three 90° turn
        }
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
