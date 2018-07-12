use super::*;
use vga::ColorCode;

#[test]
fn buffer_height() {
    // given a Buffer instance
    const HEIGHT: usize = 25;
    const WIDTH: usize = 80;
    let buf = Buffer {
        chars: [[ScreenChar::new(b'!', ColorCode::default()); WIDTH]; HEIGHT],
    };

    // when its height is queried
    let result = buf.height();

    // then the height should be 25
    assert!(result == HEIGHT);
}

#[test]
fn buffer_width() {
    // given a Buffer instance
    const HEIGHT: usize = 25;
    const WIDTH: usize = 80;
    let buf = Buffer {
        chars: [[ScreenChar::new(b'!', ColorCode::default()); WIDTH]; HEIGHT],
    };

    // when its width is queried
    let result = buf.width();

    // then the height should be 25
    assert!(result == WIDTH);
}

