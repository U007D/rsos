use super::*;

#[test]
fn test_vga_black_is_color_0x0() {
    assert!(u8::from(Color::Black) == 0x0);
}

#[test]
fn test_vga_blue_is_color_0x1() {
    assert!(u8::from(Color::Blue) == 0x1);
}

#[test]
fn test_vga_green_is_color_0x2() {
    assert!(u8::from(Color::Green) == 0x2);
}

#[test]
fn test_vga_cyan_is_color_0x3() {
    assert!(u8::from(Color::Cyan) == 0x3);
}

#[test]
fn test_vga_red_is_color_0x4() {
    assert!(u8::from(Color::Red) == 0x4);
}

#[test]
fn test_vga_magenta_is_color_0x5() {
    assert!(u8::from(Color::Magenta) == 0x5);
}

#[test]
fn test_vga_brown_is_color_0x6() {
    assert!(u8::from(Color::Brown) == 0x6);
}

#[test]
fn test_vga_light_gray_is_color_0x7() {
    assert!(u8::from(Color::LightGray) == 0x7);
}

#[test]
fn test_vga_dark_gray_is_color_0x8() {
    assert!(u8::from(Color::DarkGray) == 0x8);
}

#[test]
fn test_vga_light_blue_is_color_0x9() {
    assert!(u8::from(Color::LightBlue) == 0x9);
}

#[test]
fn test_vga_light_green_is_color_0xa() {
    assert!(u8::from(Color::LightGreen) == 0xa);
}

#[test]
fn test_vga_light_cyan_is_color_0xb() {
    assert!(u8::from(Color::LightCyan) == 0xb);
}

#[test]
fn test_vga_light_red_is_color_0xc() {
    assert!(u8::from(Color::LightRed) == 0xc);
}

#[test]
fn test_vga_pink_is_color_0xd() {
    assert!(u8::from(Color::Pink) == 0xd);
}

#[test]
fn test_vga_yellow_is_color_0xe() {
    assert!(u8::from(Color::Yellow) == 0xe);
}

#[test]
fn test_vga_white_is_color_0xf() {
    assert!(u8::from(Color::White) == 0xf);
}
