use super::*;

#[test]
fn write_byte_writes_correct_ascii_char_to_buffer() {
    // given a Writer
    const HEIGHT: usize = 25;
    const WIDTH: usize = 80;
    const ASCII_CHAR: u8 = b'!';
    let fg = Color::LightGreen;
    let bg = Color::Yellow;
    let mut buf = Buffer {
        chars: [[ScreenChar::new(0x7e, ColorCode::default()); WIDTH]; HEIGHT],
    };
    let mut writer = Writer::new(fg, bg, &mut buf);

    // when a byte is written
    let result = writer.write_byte(ASCII_CHAR);

    // then the buffer contains the right character in the right position
    assert!(result.buffer.chars[HEIGHT - 1][0].ascii_character() == Some(ASCII_CHAR));
}

#[test]
fn write_byte_writes_correct_colors_to_buffer() {
    // given a Writer
    const HEIGHT: usize = 25;
    const WIDTH: usize = 80;
    const ASCII_CHAR: u8 = b'!';
    let fg = Color::LightGreen;
    let bg = Color::Yellow;
    let mut buf = Buffer {
        chars: [[ScreenChar::new(0x7e, ColorCode::default()); WIDTH]; HEIGHT],
    };
    let mut writer = Writer::new(fg, bg, &mut buf);

    // when a byte is written
    let result = writer.write_byte(ASCII_CHAR);

    // then the buffer contains the colors in the right position
    assert!(result.buffer.chars[HEIGHT - 1][0].color_code() == ColorCode::from_colors(fg, bg));
}

#[test]
fn write_byte_writes_correct_non_printing_ascii_char_to_buffer() {
    // given a Writer
    const HEIGHT: usize = 25;
    const WIDTH: usize = 80;
    const NON_PRINTING_ASCII_CHAR: u8 = 0x19;
    let fg = Color::LightGreen;
    let bg = Color::Yellow;
    let mut buf = Buffer {
        chars: [[ScreenChar::new(b'!', ColorCode::default()); WIDTH]; HEIGHT],
    };
    let mut writer = Writer::new(fg, bg, &mut buf);

    // when a byte is written
    let result = writer.write_byte(NON_PRINTING_ASCII_CHAR);

    // then the buffer does not contain a printing ASCII char in the right position
    assert!(result.buffer.chars[HEIGHT - 1][0].ascii_character() == None);
}

#[test]
fn write_str_writes_correct_ascii_chars_to_buffer() {
    // given a Writer
    const HEIGHT: usize = 25;
    const WIDTH: usize = 80;
    const ASCII_STR: &str = "Hello, World!";
    let fg = Color::LightGreen;
    let bg = Color::Yellow;
    let mut buf = Buffer {
        chars: [[ScreenChar::new(0x7e, ColorCode::default()); WIDTH]; HEIGHT],
    };
    let mut writer = Writer::new(fg, bg, &mut buf);

    // when a str is written
    let result = writer.write_str(ASCII_STR);

    // then the buffer contains the right characters in the right position
    ASCII_STR.bytes()
             .enumerate()
             .for_each(|(i, b)| assert!(result.buffer.chars[HEIGHT - 1][i].ascii_character() == Some(b)));
}

#[test]
fn write_str_writes_correct_colors_to_buffer() {
    // given a Writer
    const HEIGHT: usize = 25;
    const WIDTH: usize = 80;
    const ASCII_STR: &str = "Hello, World!";
    let fg = Color::LightGreen;
    let bg = Color::Yellow;
    let mut buf = Buffer {
        chars: [[ScreenChar::new(0x7e, ColorCode::default()); WIDTH]; HEIGHT],
    };
    let mut writer = Writer::new(fg, bg, &mut buf);

    // when a str is written
    let result = writer.write_str(ASCII_STR);

    // then the buffer contains the right characters in the right position
    ASCII_STR.bytes()
             .enumerate()
             .for_each(|(i, _)| assert!(result.buffer
                                              .chars[HEIGHT - 1][i].color_code() == ColorCode::from_colors(fg, bg)));
}

#[test]
fn write_str_correctly_maps_non_ascii_chars_to_buffer() {
    // given a Writer
    const HEIGHT: usize = 25;
    const WIDTH: usize = 80;
    const ASCII_STR: &str = "Hello, Wörld!";
    let fg = Color::LightGreen;
    let bg = Color::Yellow;
    let mut buf = Buffer {
        chars: [[ScreenChar::new(0x7e, ColorCode::default()); WIDTH]; HEIGHT],
    };
    let mut writer = Writer::new(fg, bg, &mut buf);

    // when a str is written
    let result = writer.write_str(ASCII_STR);

    // then the buffer contains the right characters in the right position
    ASCII_STR.bytes()
             .enumerate()
             .for_each(|(i, b)| match i {
                 8..=9 => assert!(result.buffer.chars[HEIGHT - 1][i].ascii_character() == None),
                 _ => assert!(result.buffer.chars[HEIGHT - 1][i].ascii_character() == Some(b)),
             });
}

