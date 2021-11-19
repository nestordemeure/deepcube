mod sizes;
mod color;
mod moves;
mod flat;
use color::Color;
use sizes::{NB_SQUARES_SIDE, NB_SQUARES_CUBE};
use moves::{MoveKind, Amplitude, Move};
use flat::cube::FlatCube;

//---------------------------------------------------------------------------------------
// Blocks

/// coordinates along the 3 3D axis
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Coordinate3D<T: Copy>
{
    pub right_left: T,
    pub top_down: T,
    pub front_back: T
}

/// one of the tiny colored 3D blocks that make a cube
#[derive(Clone)]
pub struct Block
{
    pub position: Coordinate3D<usize>,
    pub color: Coordinate3D<Color>
}

impl Block
{
    /// takes a block
    /// returns true if the block should be rotated by the given move
    fn should_move(&self, m: Move) -> bool
    {
        let position = self.position;
        match m.kind
        {
            MoveKind::Front => position.front_back == 0,
            MoveKind::Side => position.front_back == 1,
            MoveKind::Back => position.front_back == 2,
            MoveKind::Right => position.right_left == 0,
            MoveKind::Middle => position.right_left == 1,
            MoveKind::Left => position.right_left == 2,
            MoveKind::Up => position.top_down == 0,
            MoveKind::Equator => position.top_down == 1,
            MoveKind::Down => position.top_down == 2
        }
    }

    /// does a 90° clockwise rotation along the right-left axis
    pub fn rotate_right_left(&self) -> Block
    {
        // permutes the colors
        let right_left = self.color.right_left;
        let top_down = self.color.front_back;
        let front_back = self.color.top_down;
        let color = Coordinate3D { right_left, top_down, front_back };
        // rotates the coordinates
        let right_left = self.position.right_left;
        let top_down = NB_SQUARES_SIDE - self.position.front_back;
        let front_back = self.position.top_down;
        let position = Coordinate3D { right_left, top_down, front_back };
        Block { position, color }
    }

    /// does a 90° clockwise rotation along the top_down axis
    pub fn rotate_top_down(&self) -> Block
    {
        // permutes the colors
        let right_left = self.color.front_back;
        let top_down = self.color.top_down;
        let front_back = self.color.right_left;
        let color = Coordinate3D { right_left, top_down, front_back };
        // rotates the coordinates
        let right_left = self.position.front_back;
        let top_down = self.position.top_down;
        let front_back = NB_SQUARES_SIDE - self.position.right_left;
        let position = Coordinate3D { right_left, top_down, front_back };
        Block { position, color }
    }

    /// does a 90° clockwise rotation along the front-back axis
    pub fn rotate_front_back(&self) -> Block
    {
        // permutes the colors
        let right_left = self.color.top_down;
        let top_down = self.color.right_left;
        let front_back = self.color.front_back;
        let color = Coordinate3D { right_left, top_down, front_back };
        // rotates the coordinates
        let right_left = NB_SQUARES_SIDE - self.position.top_down;
        let top_down = self.position.right_left;
        let front_back = self.position.front_back;
        let position = Coordinate3D { right_left, top_down, front_back };
        Block { position, color }
    }

    /// applies a single 90° clockwise rotation to a block
    /// along the appropriate axis, given the MoveKind
    fn rotate(&self, kind: MoveKind) -> Block
    {
        match kind
        {
            MoveKind::Front | MoveKind::Side | MoveKind::Back => self.rotate_front_back(),
            MoveKind::Right | MoveKind::Middle | MoveKind::Left => self.rotate_right_left(),
            MoveKind::Up | MoveKind::Equator | MoveKind::Down => self.rotate_top_down()
        }
    }

    /// takes a move and produces a new, rotated, block by applying the move
    fn apply_move(&self, m: Move) -> Block
    {
        let mut block = self.clone();
        if block.should_move(m)
        {
            for _rotation in 0..m.amplitude.nb_rotations()
            {
                block = block.rotate(m.kind);
            }
        }
        block
    }
}

//---------------------------------------------------------------------------------------
// Cube

/// a Rubik's cube is defined as 3D blocks that can be accessed and rotated
/// this representation makes it particularly easy to implement moves and understand the 3D structure of the cube
/// however, it is ineficient when one want to apply moves quickly and it waste memory

pub struct Cube
{
    pub blocks: Vec<Block>
}

impl Cube
{
    /// produces a new, solved, Rubik's cube
    /// we use the western color scheme as a reference for the colors
    /// https://www.speedsolving.com/wiki/index.php/Western_Color_Scheme
    pub fn solved() -> Cube
    {
        let mut blocks = Vec::with_capacity(NB_SQUARES_SIDE * NB_SQUARES_SIDE * NB_SQUARES_SIDE);
        for rl in 0..NB_SQUARES_SIDE
        {
            let rl_color = [Color::Red, Color::Invalid, Color::Orange][rl];
            for td in 0..NB_SQUARES_SIDE
            {
                let td_color = [Color::White, Color::Invalid, Color::Blue][td];
                for fb in 0..NB_SQUARES_SIDE
                {
                    let fb_color = [Color::Green, Color::Invalid, Color::Yellow][fb];
                    // builds the block with appropriate color and position
                    let color =
                        Coordinate3D { top_down: td_color, right_left: rl_color, front_back: fb_color };
                    let position = Coordinate3D { right_left: rl, top_down: td, front_back: fb };
                    let block = Block { color, position };
                    blocks.push(block)
                }
            }
        }
        Cube { blocks }
    }

    /// gets the colors of the square at the given 3D coordinates
    /// NOTE: this operation is linear in the number of Blocks and can thus be relatively expensive
    pub fn get(&self, right_left: usize, top_down: usize, front_back: usize) -> Coordinate3D<Color>
    {
        let is_target_position = |position: &Coordinate3D<usize>| -> bool {
            (position.right_left == right_left)
            && (position.top_down == top_down)
            && (position.front_back == front_back)
        };
        self.blocks
            .iter()
            .find(|block| is_target_position(&block.position))
            .map(|block| block.color)
            .expect("you tried to get a block that does not exist")
    }

    /// takes a move and produces a new, twisted, cube by applying the move
    /// does NOT insures that the blocks of the output are sorted
    pub fn apply_move_unsorted(&self, m: Move) -> Cube
    {
        // applies the move to all blocks
        let blocks = self.blocks.iter().map(|block| block.apply_move(m)).collect();
        Cube { blocks }
    }

    /// takes a move and produces a new, twisted, cube by applying the move
    /// however, preserves orientation when applying a middle layer rotation
    /// does NOT insures that the blocks of the output are sorted
    pub fn apply_move_orientation_preserving_unsorted(&self, m: Move) -> Cube
    {
        // rotates the parallel layers instead of the middle layer
        let (kind1, kind2) = match m.kind
        {
            // the middle layer parallel to the Right and Left faces
            MoveKind::Middle => (MoveKind::Right, MoveKind::Left),
            // the middle layer parallel to the Up and Down faces
            MoveKind::Equator => (MoveKind::Up, MoveKind::Down),
            // the middle layer parallel to the Front and Back faces
            MoveKind::Side => (MoveKind::Front, MoveKind::Back),
            // any non middle layer, we can just apply as usual
            _ => return self.apply_move_unsorted(m)
        };
        // converts clockwise motion into counterclockwise motion
        let amplitude = match m.amplitude
        {
            Amplitude::Clockwise => Amplitude::Counterclockwise,
            Amplitude::Fullturn => Amplitude::Fullturn,
            Amplitude::Counterclockwise => Amplitude::Clockwise
        };
        // applies the two moves
        let move1 = Move { kind: kind1, amplitude };
        let move2 = Move { kind: kind2, amplitude };
        self.apply_move_unsorted(move1).apply_move_unsorted(move2)
    }

    /// takes a move and produces a new, twisted, cube by applying the move
    pub fn apply_move(&self, m: Move) -> Cube
    {
        let mut cube = self.apply_move_unsorted(m);
        // sorts the blocks by position to insure reproducibility
        // this is especially important to make sure flatten always behave identically
        // (one could move the sort into flatten but it would be inelegant and this function is not efficient anyway)
        cube.blocks.sort_unstable_by_key(|block| block.position);
        cube
    }

    /*/// takes a cube and produces a new cube with color switched such that the center of the first face is of the first color, etc
    /// this let us ignore orientation further in the code
    /// NOTE: it is not equivalent to rotating the cube into a standard orientation
    fn normalize_orientation(&self) -> Cube
    {
        // builds a mapping to turn colors into expected colors
        // uses the fact that we know that the center of each face will be of a different color
        let mut color_mapping = [Color::Invalid; NB_FACES];
        for face in 0..NB_FACES
        {
            let color_center_face = self.get(face, SIZE_SIDE / 2, SIZE_SIDE / 2);
            let expected_color = Color::ALL[face];
            color_mapping[color_center_face as usize] = expected_color;
        }
        // maps the squares
        let mut squares: [Color; SIZE_CUBE] = [Color::Invalid; SIZE_CUBE];
        for i in 0..SIZE_CUBE
        {
            let color = self.squares[i];
            squares[i] = color_mapping[color as usize]
        }
        // returns the new square
        Cube { squares }
    }*/

    /// takes a cube and outputs a FlatCube suited to fast application of moves
    pub fn flatten(&self) -> FlatCube
    {
        // insures that the vector starts sorted
        debug_assert!(self.blocks.is_sorted_by_key(|block| block.position));
        // puts the valid colors in the array one after the other
        let mut squares: [Color; NB_SQUARES_CUBE] = [Color::Invalid; NB_SQUARES_CUBE];
        let mut index_square = 0;
        for colors in self.blocks.iter().map(|block| block.color)
        {
            let rl = colors.right_left;
            if rl != Color::Invalid
            {
                squares[index_square] = rl;
                index_square += 1;
            }
            let td = colors.top_down;
            if td != Color::Invalid
            {
                squares[index_square] = td;
                index_square += 1;
            }
            let fb = colors.front_back;
            if fb != Color::Invalid
            {
                squares[index_square] = fb;
                index_square += 1;
            }
        }
        // insures that we used all the squares
        debug_assert!(index_square == NB_SQUARES_CUBE);
        FlatCube { squares }
    }
}
