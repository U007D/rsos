#[cfg(test)]
mod unit_tests;
use core::convert::TryFrom;
use super::Error;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Color  {
    Black = 0x0,
    Blue,
    Green,
    Cyan,
    Red,
    Magenta,
    Brown,
    LightGray,
    DarkGray,
    LightBlue,
    LightGreen,
    LightCyan,
    LightRed,
    Pink,
    Yellow,
    White,
}

impl From<Color> for u8 {
    fn from(color: Color) -> Self {
        color as u8
    }
}

impl TryFrom<u8> for Color {
    type Error = Error;

    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0x0 => Ok(Color::Black),
            0x1 => Ok(Color::Blue),
            0x2 => Ok(Color::Green),
            0x3 => Ok(Color::Cyan),
            0x4 => Ok(Color::Red),
            0x5 => Ok(Color::Magenta),
            0x6 => Ok(Color::Brown),
            0x7 => Ok(Color::LightGray),
            0x8 => Ok(Color::DarkGray),
            0x9 => Ok(Color::LightBlue),
            0xa => Ok(Color::LightGreen),
            0xb => Ok(Color::LightCyan),
            0xc => Ok(Color::LightRed),
            0xd => Ok(Color::Pink),
            0xe => Ok(Color::Yellow),
            0xf => Ok(Color::White),
            v => Err(Error::InvalidVgaColor(v)),
        }
    }
}
