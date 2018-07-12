#[cfg(test)]
mod unit_tests;
pub const NON_ASCII_CHAR: u8 = 0xbf;

pub trait AsciiExt {
    // Code-page-437-printable ASCII characters or newline
    fn is_printable_ascii(&self) -> bool;
}

impl AsciiExt for u8 {
    fn is_printable_ascii(&self) -> bool {
        (*self >= 0x20 && *self <= 0x7e) || *self == b'\n'
    }
}
