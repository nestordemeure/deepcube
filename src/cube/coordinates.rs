//! various coordinates representations
//! each with their pros and cons
use enum_iterator::IntoEnumIterator;
use super::sizes::{NB_SQUARES_FACE, NB_SQUARES_SIDE};
use super::moves::{MoveKind, MoveDescription};

//-----------------------------------------------------------------------------
// 1D

/// coordinates into a 1D array
/// this format leads to very efficient implementations
/// but is impractical to manipulata as the mapping to the cube is non-trivial
#[derive(Clone, Copy)]
pub struct Coordinate1D
{
    /// 0 to NB_SQUARES_CUBE-1
    pub x: usize
}

impl Coordinate1D
{
    /// creates new coordinates
    pub fn new(x: usize) -> Coordinate1D
    {
        Coordinate1D { x }
    }

    /// converts to 2D coordinates
    fn to_2D(self) -> Coordinate2D
    {
        let face = Face::from_usize(self.x / NB_SQUARES_FACE);
        let x = (self.x % NB_SQUARES_FACE) / NB_SQUARES_SIDE;
        let y = self.x % NB_SQUARES_SIDE;
        Coordinate2D { face, x, y }
    }

    /// converts to 3D coordinates
    fn to_3D(self) -> Coordinate3D
    {
        self.to_2D().to_3D()
    }

    /// rotates along the given axis
    pub fn rotate(&self, axis: RotationAxis) -> Coordinate1D
    {
        self.to_3D().rotate(axis).to_1D()
    }

    /// takes a move and produces new, rotated, coordinates by applying the move
    pub fn apply_move(&self, m: &MoveDescription) -> Coordinate1D
    {
        self.to_3D().apply_move(m).to_1D()
    }
}

//-----------------------------------------------------------------------------
// 2D + face

/// all faces of a cube
#[derive(Clone, Copy, PartialEq, IntoEnumIterator)]
#[repr(usize)]
pub enum Face
{
    Left = 0,
    Front = 1,
    Right = 2,
    Back = 3,
    Up = 4,
    Down = 5
}

impl Face
{
    /// turns an usize into a Face
    fn from_usize(n: usize) -> Face
    {
        debug_assert!(n < 6);
        unsafe { ::std::mem::transmute(n) }
    }
}

/// coordinates into a 2D array with additional face coordinate
/// it is easy to identify classical position (center and corners) in this format
/// it is also easy to convert it to 3D and 1D
pub struct Coordinate2D
{
    /// in order [Left, Front, Right, Back, Up, Down]
    pub face: Face,
    /// 0 to NB_SQUARES_SIDE-1
    pub x: usize,
    /// 0 to NB_SQUARES_SIDE-1
    pub y: usize
}

impl Coordinate2D
{
    /// converts into 1D coordinates
    pub fn to_1D(&self) -> Coordinate1D
    {
        let x = (self.face as usize) * NB_SQUARES_FACE + self.x * NB_SQUARES_SIDE + self.y;
        Coordinate1D { x }
    }

    /// converts into 3D coordinates
    fn to_3D(&self) -> Coordinate3D
    {
        match self.face
        {
            Face::Left =>
            {
                let axis = RotationAxis::LeftRight;
                let left_right = 0;
                let front_back = (NB_SQUARES_SIDE - 1) - self.x;
                let down_up = self.y;
                Coordinate3D { left_right, down_up, front_back, axis }
            }
            Face::Right =>
            {
                let axis = RotationAxis::LeftRight;
                let left_right = 2;
                let front_back = self.x;
                let down_up = self.y;
                Coordinate3D { left_right, down_up, front_back, axis }
            }
            Face::Front =>
            {
                let axis = RotationAxis::FrontBack;
                let front_back = 0;
                let left_right = self.x;
                let down_up = self.y;
                Coordinate3D { left_right, down_up, front_back, axis }
            }
            Face::Back =>
            {
                let axis = RotationAxis::FrontBack;
                let front_back = 2;
                let left_right = (NB_SQUARES_SIDE - 1) - self.x;
                let down_up = self.y;
                Coordinate3D { left_right, down_up, front_back, axis }
            }
            Face::Down =>
            {
                let axis = RotationAxis::DownUp;
                let down_up = 0;
                let left_right = self.x;
                let front_back = (NB_SQUARES_SIDE - 1) - self.y;
                Coordinate3D { left_right, down_up, front_back, axis }
            }
            Face::Up =>
            {
                let axis = RotationAxis::DownUp;
                let down_up = 2;
                let left_right = self.x;
                let front_back = self.y;
                Coordinate3D { left_right, down_up, front_back, axis }
            }
        }
    }
}

//-----------------------------------------------------------------------------
// 3D + axis

/// axis along which a rotation can be done
#[derive(Clone, Copy, IntoEnumIterator, Debug)]
pub enum RotationAxis
{
    LeftRight,
    DownUp,
    FrontBack
}

/// coordinates into a 3D cube with additional facing axis information
/// this format is very easy to rotate correctly
/// one downside of this format is that some position do not correspond to a square on the cube
#[derive(Clone)]
struct Coordinate3D
{
    /// right to left
    /// 0 to NB_SQUARES_SIDE-1
    left_right: usize,
    /// bottom to top
    /// 0 to NB_SQUARES_SIDE-1
    down_up: usize,
    /// front to back
    /// 0 to NB_SQUARES_SIDE-1
    front_back: usize,
    /// along which axis is the face facing
    /// RighLeft => Right or Left face
    axis: RotationAxis
}

impl Coordinate3D
{
    /// converts into 1D coordinates
    fn to_1D(&self) -> Coordinate1D
    {
        self.to_2D().to_1D()
    }

    /// converts into 2D coordinates
    fn to_2D(&self) -> Coordinate2D
    {
        match self.axis
        {
            RotationAxis::LeftRight if self.left_right == 0 =>
            {
                let face = Face::Left;
                let x = (NB_SQUARES_SIDE - 1) - self.front_back;
                let y = self.down_up;
                Coordinate2D { face, x, y }
            }
            RotationAxis::LeftRight if self.left_right == 2 =>
            {
                let face = Face::Right;
                let x = self.front_back;
                let y = self.down_up;
                Coordinate2D { face, x, y }
            }
            RotationAxis::FrontBack if self.front_back == 0 =>
            {
                let face = Face::Front;
                let x = self.left_right;
                let y = self.down_up;
                Coordinate2D { face, x, y }
            }
            RotationAxis::FrontBack if self.front_back == 2 =>
            {
                let face = Face::Back;
                let x = (NB_SQUARES_SIDE - 1) - self.left_right;
                let y = self.down_up;
                Coordinate2D { face, x, y }
            }
            RotationAxis::DownUp if self.down_up == 0 =>
            {
                let face = Face::Down;
                let x = self.left_right;
                let y = (NB_SQUARES_SIDE - 1) - self.front_back;
                Coordinate2D { face, x, y }
            }
            RotationAxis::DownUp if self.down_up == 2 =>
            {
                let face = Face::Up;
                let x = self.left_right;
                let y = self.front_back;
                Coordinate2D { face, x, y }
            }
            _ =>
            {
                panic!("The given 3D coordinates do not map to an actual face of the cube.")
            }
        }
    }

    /// does a 90° clockwise rotation along the LeftRight axis
    fn rotate_left_right(&self) -> Coordinate3D
    {
        // change the axis
        let axis = match self.axis
        {
            RotationAxis::LeftRight => RotationAxis::LeftRight,
            RotationAxis::DownUp => RotationAxis::FrontBack,
            RotationAxis::FrontBack => RotationAxis::DownUp
        };
        // rotates the coordinates
        let left_right = self.left_right;
        let down_up = (NB_SQUARES_SIDE - 1) - self.front_back;
        let front_back = self.down_up;
        Coordinate3D { left_right, down_up, front_back, axis }
    }

    /// does a 90° clockwise rotation along the DownUp axis
    fn rotate_down_up(&self) -> Coordinate3D
    {
        // change the axis
        let axis = match self.axis
        {
            RotationAxis::LeftRight => RotationAxis::FrontBack,
            RotationAxis::DownUp => RotationAxis::DownUp,
            RotationAxis::FrontBack => RotationAxis::LeftRight
        };
        // rotates the coordinates
        let left_right = (NB_SQUARES_SIDE - 1) - self.front_back;
        let down_up = self.down_up;
        let front_back = self.left_right;
        Coordinate3D { left_right, down_up, front_back, axis }
    }

    /// does a 90° clockwise rotation along the FrontBack axis
    fn rotate_front_back(&self) -> Coordinate3D
    {
        // change the axis
        let axis = match self.axis
        {
            RotationAxis::LeftRight => RotationAxis::DownUp,
            RotationAxis::DownUp => RotationAxis::LeftRight,
            RotationAxis::FrontBack => RotationAxis::FrontBack
        };
        // rotates the coordinates
        let left_right = (NB_SQUARES_SIDE - 1) - self.down_up;
        let down_up = self.left_right;
        let front_back = self.front_back;
        Coordinate3D { left_right, down_up, front_back, axis }
    }

    /// does a 90° clockwise rotation along the given axis
    fn rotate(&self, axis: RotationAxis) -> Coordinate3D
    {
        match axis
        {
            RotationAxis::LeftRight => self.rotate_left_right(),
            RotationAxis::DownUp => self.rotate_down_up(),
            RotationAxis::FrontBack => self.rotate_front_back()
        }
    }

    /// returns true if the coordinates should be impacted by the given move
    /// as a function of the slice of the cube that is rotated by the move
    fn should_move(&self, kind: MoveKind) -> bool
    {
        match kind
        {
            MoveKind::Front => self.front_back == 0,
            MoveKind::Side => self.front_back == 1,
            MoveKind::Back => self.front_back == 2,
            MoveKind::Left => self.left_right == 0,
            MoveKind::Middle => self.left_right == 1,
            MoveKind::Right => self.left_right == 2,
            MoveKind::Down => self.down_up == 0,
            MoveKind::Equator => self.down_up == 1,
            MoveKind::Up => self.down_up == 2
        }
    }

    /// takes a move and produces new, rotated, coordinates by applying the move
    fn apply_move(&self, m: &MoveDescription) -> Coordinate3D
    {
        let mut coordinates = self.clone();
        if coordinates.should_move(m.kind)
        {
            // axis along which the rotation will be done
            let axis = match m.kind
            {
                MoveKind::Front | MoveKind::Side | MoveKind::Back => RotationAxis::FrontBack,
                MoveKind::Right | MoveKind::Middle | MoveKind::Left => RotationAxis::LeftRight,
                MoveKind::Down | MoveKind::Equator | MoveKind::Up => RotationAxis::DownUp
            };
            // does 90° clockwise rotations until the desired amplitude is reached
            for _rotation in 0..m.amplitude.nb_rotations()
            {
                coordinates = coordinates.rotate(axis);
            }
        }
        coordinates
    }
}
