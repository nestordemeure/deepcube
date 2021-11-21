use super::sizes::{NB_FACES, NB_SQUARES_CUBE, NB_SQUARES_FACE, NB_SQUARES_SIDE};
use super::moves::{MoveKind, Amplitude, Move};

//-----------------------------------------------------------------------------
// 1D

/// coordinates into a 1D array
/// this format leads to very efficient implementations
struct Coordinate1D
{
    /// 0 to NB_SQUARES_CUBE-1
    x: usize
}

impl Coordinate1D
{
    /// converts to 2D coordinates
    fn to_2D(&self) -> Coordinate2D
    {
        let face = Face::from_usize(self.x / NB_SQUARES_FACE);
        let x = (self.x % NB_SQUARES_FACE) / NB_SQUARES_SIDE;
        let y = self.x % NB_SQUARES_SIDE;
        Coordinate2D { face, x, y }
    }

    /// converts to 3D coordinates
    fn to_3D(&self) -> Coordinate3D
    {
        self.to_2D().to_3D()
    }
}

//-----------------------------------------------------------------------------
// 2D + face

/// all faces of a cube
#[derive(Clone, Copy, PartialEq)]
#[repr(usize)]
enum Face
{
    Left = 0,
    Front = 1,
    Right = 2,
    Back = 3,
    Top = 4,
    Bottom = 5
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
struct Coordinate2D
{
    /// in order [Left, Front, Right, Back, Top, Bottom]
    face: Face,
    /// 0 to NB_SQUARES_SIDE-1
    x: usize,
    /// 0 to NB_SQUARES_SIDE-1
    y: usize
}

impl Coordinate2D
{
    /// converts into 1D coordinates
    fn to_1D(&self) -> Coordinate1D
    {
        let x = (self.face as usize) * NB_SQUARES_FACE + self.x * NB_SQUARES_SIDE + self.y;
        Coordinate1D { x }
    }

    /// converts into 3D coordinates
    fn to_3D(&self) -> Coordinate3D
    {
        match self.face
        {
            Face::Left | Face::Right =>
            {
                let axis = RotationAxis::RightLeft;
                let right_left = if self.face == Face::Right { 0 } else { 2 };
                let front_back = self.x;
                let bottom_top = self.y;
                Coordinate3D { right_left, bottom_top, front_back, axis }
            }
            Face::Front | Face::Back =>
            {
                let axis = RotationAxis::FrontBack;
                let front_back = if self.face == Face::Front { 0 } else { 2 };
                let right_left = self.x;
                let bottom_top = self.y;
                Coordinate3D { right_left, bottom_top, front_back, axis }
            }
            Face::Top | Face::Bottom =>
            {
                let axis = RotationAxis::BottomTop;
                let bottom_top = if self.face == Face::Bottom { 0 } else { 2 };
                let right_left = self.x;
                let front_back = self.y;
                Coordinate3D { right_left, bottom_top, front_back, axis }
            }
        }
    }
}

//-----------------------------------------------------------------------------
// 3D + axis

/// axis along which a rotation can be done
enum RotationAxis
{
    RightLeft,
    BottomTop,
    FrontBack
}

/// coordinates into a 3D cube with additional facing axis information
/// this format is very easy to rotate correctly
struct Coordinate3D
{
    /// right to left
    /// 0 to NB_SQUARES_SIDE-1
    right_left: usize,
    /// bottom to top
    /// 0 to NB_SQUARES_SIDE-1
    bottom_top: usize,
    /// front to back
    /// 0 to NB_SQUARES_SIDE-1
    front_back: usize,
    /// one of three possibilities
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
            RotationAxis::RightLeft =>
            {
                debug_assert_ne!(self.right_left, 1);
                let face = if self.right_left == 0 { Face::Right } else { Face::Left };
                let x = self.front_back;
                let y = self.bottom_top;
                Coordinate2D { face, x, y }
            }
            RotationAxis::FrontBack =>
            {
                debug_assert_ne!(self.front_back, 1);
                let face = if self.front_back == 0 { Face::Front } else { Face::Back };
                let x = self.right_left;
                let y = self.bottom_top;
                Coordinate2D { face, x, y }
            }
            RotationAxis::BottomTop =>
            {
                debug_assert_ne!(self.bottom_top, 1);
                let face = if self.bottom_top == 0 { Face::Bottom } else { Face::Top };
                let x = self.right_left;
                let y = self.front_back;
                Coordinate2D { face, x, y }
            }
        }
    }
}
