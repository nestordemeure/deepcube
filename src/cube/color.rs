//! Colors and color related operations
use super::sizes::NB_FACES;
use ansi_term::Colour;

//-----------------------------------------------------------------------------
// Color

/// one color per face of the cube
pub const NB_COLORS: usize = NB_FACES;

/// color of the faces of the cube
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Color
{
    Orange = 0,
    Green = 1,
    Red = 2,
    Blue = 3,
    White = 4,
    Yellow = 5,
    /// if this color comes up, you know there is a bug
    Invalid
}

impl Color
{
    /// list of all colors in [Left, Front, Right, Back, Up, Down] order
    /// according to the western color scheme: https://www.speedsolving.com/wiki/index.php/Western_Color_Scheme
    pub const ALL: [Color; NB_COLORS] =
        [Color::Orange, Color::Green, Color::Red, Color::Blue, Color::White, Color::Yellow];

    /// converts the color to a shell color for display
    pub fn to_shell_color(&self) -> Colour
    {
        match self
        {
            Color::Orange => Colour::RGB(255, 127, 80),
            Color::Green => Colour::Green,
            Color::Red => Colour::Red,
            Color::Blue => Colour::Blue,
            Color::White => Colour::White,
            Color::Yellow => Colour::Yellow,
            Color::Invalid => panic!("The Invalid color cannot be converted into a shell-displayeable color.")
        }
    }
}
