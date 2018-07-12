#[cfg(test)]
mod unit_tests;

use super::{AsciiExt, Buffer, Color, ColorCode, ScreenChar, NON_ASCII_CHAR};

pub struct Writer<'a> {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'a mut Buffer,
}

impl<'a> Writer<'a> {
    pub fn new(fg_color: Color, bg_color: Color, buffer: &'a mut Buffer) -> Self {
        Self {
            column_position: 0,
            color_code: ColorCode::from_colors(fg_color, bg_color),
            buffer,
        }
    }

    fn new_line(&mut self) {/* TODO */}

    pub fn write_byte(&mut self, byte: u8) -> &mut Self {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= self.buffer.width() {
                    self.new_line();
                }

                let row = self.buffer.height() - 1;
                let col = self.column_position;

                self.buffer.chars[row][col] = ScreenChar::new(byte, self.color_code);
                self.column_position += 1;
            }
        }
        self
    }

    pub fn write_str(&mut self, s: &str) -> &mut Self {
        s.bytes().for_each(|b| {
            match b.is_ascii() {
                true => self.write_byte(b),
                false => self.write_byte(NON_ASCII_CHAR),
            };
        });
        self
    }
}
