use super::*;

#[test]
fn from_colors_yields_correct_colors() {
    // given a foreground and background color
    let fg = Color::Pink;
    let bg = Color::LightGreen;

    // when a ColorCode is constructed
    let result = ColorCode::from_colors(fg, bg);

    // then the ColorCode's fg color should be pink
    assert!(result.fg_color() == fg);

    // and the ColorCode's bg color should be light green
    assert!(result.bg_color() == bg);
}

#[test]
fn default_yields_correct_colors() {
    // given a foreground and background color
    let fg = Color::White;
    let bg = Color::Black;

    // when a ColorCode is constructed
    let result = ColorCode::default();

    // then the ColorCode's fg color should be white
    assert!(result.fg_color() == fg);

    // and the ColorCode's bg color should be black
    assert!(result.bg_color() == bg);
}

#[test]
fn set_colors_fluently_yields_correct_colors() {
    // given foreground and background colors and a ColorCode
    let fg1 = Color::Pink;
    let fg2 = Color::Yellow;
    let bg1 = Color::LightGreen;
    let bg2 = Color::DarkGray;
    let mut color_code = ColorCode::from_colors(fg1, bg1);
    let instance = &color_code as *const ColorCode;

    // when set_colors() is called to change the fg and bg colors
    let result = color_code.set_colors(fg2, bg2);

    // then the returned instance is the same instance
    assert!(result as *const ColorCode == instance);

    // and the ColorCode's fg color should be changed
    assert!(result.fg_color() == fg2);

    // and the ColorCode's bg color should be changed
    assert!(result.bg_color() == bg2);
}

#[test]
fn set_fg_color_fluently_yields_correct_colors() {
    // given foreground and background colors and a ColorCode
    let fg1 = Color::Pink;
    let fg2 = Color::Yellow;
    let bg1 = Color::LightGreen;
    let mut color_code = ColorCode::from_colors(fg1, bg1);
    let instance = &color_code as *const ColorCode;

    // when set_fg_color() is called to change only the fg color
    let result = color_code.set_fg_color(fg2);

    // then the returned instance is the same instance
    assert!(result as *const ColorCode == instance);

    // and the ColorCode's fg color should be changed
    assert!(result.fg_color() == fg2);

    // and the ColorCode's bg color should be unchanged
    assert!(result.bg_color() == bg1);
}

#[test]
fn set_bg_color_fluently_yields_correct_colors() {
    // given foreground and background colors and a ColorCode
    let fg1 = Color::Pink;
    let bg1 = Color::LightGreen;
    let bg2 = Color::DarkGray;
    let mut color_code = ColorCode::from_colors(fg1, bg1);
    let instance = &color_code as *const ColorCode;

    // when set_bg_color() is called to change only the bg color
    let result = color_code.set_bg_color(bg2);

    // then the returned instance is the same instance
    assert!(result as *const ColorCode == instance);

    // and the ColorCode's fg color should be unchanged
    assert!(result.fg_color() == fg1);

    // and the ColorCode's bg color should be changed
    assert!(result.bg_color() == bg2);
}
