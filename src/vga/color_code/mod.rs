#[cfg(test)]
mod unit_tests;
use consts::*;
use core::convert::TryFrom;
use super::Color;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ColorCode(u8);

impl ColorCode {
    pub fn from_colors(fg_color: Color, bg_color: Color) -> Self {
            ColorCode(ColorCode::color_code_value(fg_color, bg_color))
    }

    pub fn bg_color(&self) -> Color {
        self.colors().1
    }

    pub fn fg_color(&self) -> Color {
        self.colors().0
    }

    pub fn set_bg_color(&mut self, bg_color: Color) -> &mut Self {
        self.set_colors(self.colors().0, bg_color)
    }

    pub fn set_colors(&mut self, fg_color: Color, bg_color: Color) -> &mut Self {
        self.0 = ColorCode::color_code_value(fg_color, bg_color);
        self
    }

    pub fn set_fg_color(&mut self, fg_color: Color) -> &mut Self {
        self.set_colors(fg_color, self.colors().1)
    }

    #[inline]
    fn colors(&self) -> (Color, Color) {
        (Color::try_from(self.0 >> 4).expect(MSG_ERR_INTERNAL_INVALID_VGA_COLOR),
         Color::try_from(self.0 & 0x0f).expect(MSG_ERR_INTERNAL_INVALID_VGA_COLOR))
    }

    #[inline]
    fn color_code_value(fg_color: Color, bg_color: Color) -> u8 {
        u8::from(fg_color) << 4 | u8::from(bg_color)
    }
}

impl Default for ColorCode {
    fn default() -> Self {
        ColorCode::from_colors(Color::White, Color::Black)
    }
}
