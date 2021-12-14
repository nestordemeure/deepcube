//! functions used for display
use ansi_term::{Style, Colour};
use super::Cube;
use super::color::Color;
use super::coordinates::Face;
use super::sizes::NB_SQUARES_SIDE;

impl Color
{
    /// converts the color to a shell color for display
    pub fn to_shell_color(self) -> Colour
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

impl Face
{
    /// turns a face into a single letter for display purposes
    pub fn to_single_letter_string(self) -> String
    {
        let letter = match self
        {
            Face::Left => "L",
            Face::Front => "F",
            Face::Right => "R",
            Face::Back => "B",
            Face::Up => "U",
            Face::Down => "D"
        };
        letter.to_string()
    }
}

impl Cube
{
    /// displays the cube in the shell
    pub fn display(&self)
    {
        // displays the square at a given 2D coordinate
        let display_square = |face: Face, x: usize, y: usize| {
            let color = self.get(face, x, y).to_shell_color();
            let text = if (x == 1) && (y == 1)
            {
                format!("{} ", face.to_single_letter_string())
            }
            else
            {
                "  ".to_string()
            };
            let colored_text = Style::new().on(color).fg(Colour::Black).bold().paint(text);
            print!("{}", colored_text);
        };

        // displays top square
        for y in (0..NB_SQUARES_SIDE).rev()
        {
            print!("      "); // empty line
            for x in 0..NB_SQUARES_SIDE
            {
                display_square(Face::Up, x, y);
            }
            println!();
        }

        // displays middle squares
        for y in (0..NB_SQUARES_SIDE).rev()
        {
            for face in [Face::Left, Face::Front, Face::Right, Face::Back]
            {
                for x in 0..NB_SQUARES_SIDE
                {
                    display_square(face, x, y);
                }
            }
            println!();
        }

        // displays bottom square
        for y in (0..NB_SQUARES_SIDE).rev()
        {
            print!("      "); // empty line
            for x in 0..NB_SQUARES_SIDE
            {
                display_square(Face::Down, x, y);
            }
            println!();
        }
    }
}
