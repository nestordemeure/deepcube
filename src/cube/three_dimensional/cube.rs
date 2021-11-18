//! a cube defined as 3D blocks that can be accessed and rotated
//! this representation makes it particularly easy to define moves

use super::super::color::Color;
use super::super::sizes::NB_SQUARES_SIDE;

//---------------------------------------------------------------------------------------
// Coordinates

/// coordinates along the 3 3D axis
#[derive(Copy, Clone)]
pub struct Coordinate3D<T: Copy>
{
    pub right_left: T,
    pub top_down: T,
    pub front_back: T
}

//---------------------------------------------------------------------------------------
// Blocks

/// one of the tiny colored blocks that make a cube
#[derive(Clone)]
pub struct Block
{
    pub position: Coordinate3D<usize>,
    pub color: Coordinate3D<Color>
}

impl Block
{
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
}

//---------------------------------------------------------------------------------------
// Cube

/// a 3D cube
pub struct Cube3D
{
    pub blocks: Vec<Block>
}

impl Cube3D
{
    /// produces a new, solved, Rubik's cube
    /// we use the western color scheme as a reference for the colors
    /// https://www.speedsolving.com/wiki/index.php/Western_Color_Scheme
    fn solved() -> Cube3D
    {
        let mut blocks = Vec::with_capacity(NB_SQUARES_SIDE * NB_SQUARES_SIDE * NB_SQUARES_SIDE);
        for td in 0..NB_SQUARES_SIDE
        {
            let td_color = [Color::White, Color::Invalid, Color::Blue][td];
            for rl in 0..NB_SQUARES_SIDE
            {
                let rl_color = [Color::Red, Color::Invalid, Color::Orange][rl];
                for fb in 0..NB_SQUARES_SIDE
                {
                    let fb_color = [Color::Green, Color::Invalid, Color::Yellow][fb];
                    // builds the block with appropriate color and position
                    let color =
                        Coordinate3D { top_down: td_color, right_left: rl_color, front_back: fb_color };
                    let position = Coordinate3D { top_down: td, right_left: rl, front_back: fb };
                    let block = Block { color, position };
                    blocks.push(block)
                }
            }
        }
        Cube3D { blocks }
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
}

// NOTES:
// flatten can just be taking the blocks in order
// unflatten would be the reverse operation
