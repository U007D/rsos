use std::vec::Vec;
use super::*;
use vga::Color;

#[test]
fn ascii_character_rejects_bytes_below_0x20_except_newline() {
    // given a range of test values
    let test_values = 0x0_u8..0x20;

    // when values are tested for ascii validity
    let result = test_values.map(|v| ScreenChar::new(v, ColorCode::default()))
                            .map(|sc| sc.ascii_character())
                            .filter(|c| c.is_some())
                            .collect::<Vec<_>>();

    // then all values except newline should be excluded
    assert!(&result == &[Some(10)]);
}

#[test]
fn ascii_character_rejects_bytes_above_0x7e() {
    // given a range of test values
    let test_values = 0x7f_u8..=0xff;

    // when values are tested for ascii validity
    let result = test_values.map(|v| ScreenChar::new(v, ColorCode::default()))
                            .map(|sc| sc.ascii_character())
                            .filter(|c| c.is_some())
                            .collect::<Vec<_>>();

    // then all values should be excluded
    assert!(&result == &[]);
}

#[test]
fn ascii_character_accepts_bytes_from_0x20_0x7e() {
    // given a range of test values
    let test_values = 0x20_u8..=0x7e;

    // when values are tested for ascii invalidity
    let result = test_values.map(|v| ScreenChar::new(v, ColorCode::default()))
                            .map(|sc| sc.ascii_character())
                            .filter(|c| c.is_none())
                            .collect::<Vec<_>>();

    // then all values should be excluded
    assert!(&result == &[]);
}

#[test]
fn color_code_yields_correct_value() {
    // given a ScreenChar instance
    let fg = Color::Blue;
    let bg = Color::LightGreen;
    let screen_char = ScreenChar::new(b'A', ColorCode::from_colors(fg, bg));

    // when the ScreenChar's ColorCode is queried
    let result = screen_char.color_code();

    // then the foreground should be blue
    assert!(result.fg_color() == fg);

    // and the background should be light green
    assert!(result.bg_color() == bg)
}
