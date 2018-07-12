use super::*;

#[test]
fn test_try_from_0x0_yields_vga_black() {
    assert!(Color::try_from(0x0) == Ok(Color::Black));
}

#[test]
fn test_try_from_0x1_yields_vga_blue() {
    assert!(Color::try_from(0x1) == Ok(Color::Blue));
}

#[test]
fn test_try_from_0x2_yields_vga_green() {
    assert!(Color::try_from(0x2) == Ok(Color::Green));
}

#[test]
fn test_try_from_0x3_yields_vga_cyan() {
    assert!(Color::try_from(0x3) == Ok(Color::Cyan));
}

#[test]
fn test_try_from_0x4_yields_vga_red() {
    assert!(Color::try_from(0x4) == Ok(Color::Red));
}

#[test]
fn test_try_from_0x5_yields_vga_magenta() {
    assert!(Color::try_from(0x5) == Ok(Color::Magenta));
}

#[test]
fn test_try_from_0x6_yields_vga_brown() {
    assert!(Color::try_from(0x6) == Ok(Color::Brown));
}

#[test]
fn test_try_from_0x7_yields_vga_light_gray() {
    assert!(Color::try_from(0x7) == Ok(Color::LightGray));
}

#[test]
fn test_try_from_0x8_yields_vga_dark_gray() {
    assert!(Color::try_from(0x8) == Ok(Color::DarkGray));
}

#[test]
fn test_try_from_0x9_yields_vga_light_blue() {
    assert!(Color::try_from(0x9) == Ok(Color::LightBlue));
}

#[test]
fn test_try_from_0xa_yields_vga_light_green() {
    assert!(u8::from(Color::LightGreen) == 0xa);
}

#[test]
fn test_try_from_0xb_yields_vga_light_cyan() {
    assert!(u8::from(Color::LightCyan) == 0xb);
}

#[test]
fn test_try_from_0xc_yields_vga_light_red() {
    assert!(u8::from(Color::LightRed) == 0xc);
}

#[test]
fn test_try_from_0xd_yields_vga_pink() {
    assert!(u8::from(Color::Pink) == 0xd);
}

#[test]
fn test_try_from_0xe_yields_vga_yellow() {
    assert!(u8::from(Color::Yellow) == 0xe);
}

#[test]
fn test_try_from_0xf_yields_vga_white() {
    assert!(u8::from(Color::White) == 0xf);
}
