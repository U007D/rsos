#[cfg(test)]
mod unit_tests;
use super::{AsciiExt, ColorCode};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct ScreenChar {
    char: u8,
    color_code: ColorCode,
}

impl ScreenChar {
    pub fn new(char: u8, color_code: ColorCode) -> Self {
        Self {
            char,
            color_code,
        }
    }

    pub fn ascii_character(&self) -> Option<u8> {
        match self.char.is_printable_ascii() {
            true => Some(self.char),
            false => None,
        }
    }

    pub fn color_code(&self) -> ColorCode {
        self.color_code
    }
}
