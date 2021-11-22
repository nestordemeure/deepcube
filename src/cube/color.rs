//! Colors and color related operations
use super::sizes::NB_FACES;

//-----------------------------------------------------------------------------
// Color

/// one color per face of the cube
pub const NB_COLORS: usize = NB_FACES;

/// color of the faces of the cube
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Color
{
    Red = 0,
    Blue = 1,
    Green = 2,
    Yellow = 3,
    White = 4,
    Orange = 5,
    /// if this color comes up, you know there is a bug
    Invalid
}

impl Color
{
    /// list of all colors in [Left, Front, Right, Back, Up, Down] order
    /// according to the western color scheme: https://www.speedsolving.com/wiki/index.php/Western_Color_Scheme
    pub const ALL: [Color; NB_COLORS] =
        [Color::Orange, Color::Green, Color::Red, Color::Blue, Color::White, Color::Yellow];
}
