#[cfg(test)]
mod unit_tests;
use super::ScreenChar;

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

pub struct Buffer {
    pub chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

impl Buffer {
    pub fn height(&self) -> usize {
        BUFFER_HEIGHT
    }

    pub fn width(&self) -> usize {
        BUFFER_WIDTH
    }
}
