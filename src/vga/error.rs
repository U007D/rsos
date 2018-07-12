use core::fmt;
use consts::*;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum Error {
    InvalidVgaColor(u8),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match *self {
            Error::InvalidVgaColor(_) => MSG_ERR_INVALID_VGA_COLOR,
        })
    }
}

