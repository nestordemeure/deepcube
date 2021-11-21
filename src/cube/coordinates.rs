use super::sizes::{NB_SQUARES_FACE, NB_SQUARES_SIDE};
use super::moves::{MoveKind, Move};

//-----------------------------------------------------------------------------
// 1D

/// coordinates into a 1D array
/// this format leads to very efficient implementations
/// but is impractical to manipulata as the mapping to the cube is non-trivial
struct Coordinate1D
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

    /// takes a move and produces new, rotated, coordinates by applying the move
    pub fn apply_move(&self, m: Move) -> Coordinate1D
    {
        self.to_3D().apply_move(m).to_1D()
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
                let down_up = self.y;
                Coordinate3D { right_left, down_up, front_back, axis }
            }
            Face::Front | Face::Back =>
            {
                let axis = RotationAxis::FrontBack;
                let front_back = if self.face == Face::Front { 0 } else { 2 };
                let right_left = self.x;
                let down_up = self.y;
                Coordinate3D { right_left, down_up, front_back, axis }
            }
            Face::Down | Face::Up =>
            {
                let axis = RotationAxis::DownUp;
                let down_up = if self.face == Face::Down { 0 } else { 2 };
                let right_left = self.x;
                let front_back = self.y;
                Coordinate3D { right_left, down_up, front_back, axis }
            }
        }
    }
}

//-----------------------------------------------------------------------------
// 3D + axis

/// axis along which a rotation can be done
#[derive(Clone, Copy)]
enum RotationAxis
{
    RightLeft,
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
    right_left: usize,
    /// bottom to top
    /// 0 to NB_SQUARES_SIDE-1
    down_up: usize,
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
                let y = self.down_up;
                Coordinate2D { face, x, y }
            }
            RotationAxis::FrontBack =>
            {
                debug_assert_ne!(self.front_back, 1);
                let face = if self.front_back == 0 { Face::Front } else { Face::Back };
                let x = self.right_left;
                let y = self.down_up;
                Coordinate2D { face, x, y }
            }
            RotationAxis::DownUp =>
            {
                debug_assert_ne!(self.down_up, 1);
                let face = if self.down_up == 0 { Face::Down } else { Face::Up };
                let x = self.right_left;
                let y = self.front_back;
                Coordinate2D { face, x, y }
            }
        }
    }

    /// does a 90° clockwise rotation along the RightLeft axis
    fn rotate_right_left(&self) -> Coordinate3D
    {
        // change the axis
        let axis = match self.axis
        {
            RotationAxis::RightLeft => RotationAxis::RightLeft,
            RotationAxis::DownUp => RotationAxis::FrontBack,
            RotationAxis::FrontBack => RotationAxis::DownUp
        };
        // rotates the coordinates
        let right_left = self.right_left;
        let down_up = NB_SQUARES_SIDE - self.front_back;
        let front_back = self.down_up;
        Coordinate3D { right_left, down_up, front_back, axis }
    }

    /// does a 90° clockwise rotation along the DownUp axis
    fn rotate_down_up(&self) -> Coordinate3D
    {
        // change the axis
        let axis = match self.axis
        {
            RotationAxis::RightLeft => RotationAxis::FrontBack,
            RotationAxis::DownUp => RotationAxis::DownUp,
            RotationAxis::FrontBack => RotationAxis::RightLeft
        };
        // rotates the coordinates
        let right_left = self.front_back;
        let down_up = self.down_up;
        let front_back = NB_SQUARES_SIDE - self.right_left;
        Coordinate3D { right_left, down_up, front_back, axis }
    }

    /// does a 90° clockwise rotation along the FrontBack axis
    fn rotate_front_back(&self) -> Coordinate3D
    {
        // change the axis
        let axis = match self.axis
        {
            RotationAxis::RightLeft => RotationAxis::DownUp,
            RotationAxis::DownUp => RotationAxis::RightLeft,
            RotationAxis::FrontBack => RotationAxis::RightLeft
        };
        // rotates the coordinates
        let right_left = NB_SQUARES_SIDE - self.down_up;
        let down_up = self.right_left;
        let front_back = self.front_back;
        Coordinate3D { right_left, down_up, front_back, axis }
    }

    /// does a 90° clockwise rotation along the given axis
    fn rotate(&self, axis: RotationAxis) -> Coordinate3D
    {
        match axis
        {
            RotationAxis::RightLeft => self.rotate_right_left(),
            RotationAxis::DownUp => self.rotate_down_up(),
            RotationAxis::FrontBack => self.rotate_front_back()
        }
    }

    /// returns true if the coordinates should be impacted by the given move
    /// as a function of the slice of the cube that is rotated by the move
    fn should_move(&self, m: Move) -> bool
    {
        match m.kind
        {
            MoveKind::Front => self.front_back == 0,
            MoveKind::Side => self.front_back == 1,
            MoveKind::Back => self.front_back == 2,
            MoveKind::Right => self.right_left == 0,
            MoveKind::Middle => self.right_left == 1,
            MoveKind::Left => self.right_left == 2,
            MoveKind::Up => self.down_up == 2,
            MoveKind::Equator => self.down_up == 1,
            MoveKind::Down => self.down_up == 0
        }
    }

    /// takes a move and produces new, rotated, coordinates by applying the move
    fn apply_move(&self, m: Move) -> Coordinate3D
    {
        let mut coordinates = self.clone();
        if coordinates.should_move(m)
        {
            // axis along which the rotation will be done
            let axis = match m.kind
            {
                MoveKind::Front | MoveKind::Side | MoveKind::Back => RotationAxis::FrontBack,
                MoveKind::Right | MoveKind::Middle | MoveKind::Left => RotationAxis::RightLeft,
                MoveKind::Up | MoveKind::Equator | MoveKind::Down => RotationAxis::DownUp
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
