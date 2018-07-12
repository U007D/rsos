use super::*;
use std::vec::Vec;

#[test]
fn is_printable_ascii_rejects_bytes_below_0x20_except_newline() {
    // given a range of test values
    let test_values = 0x0_u8..0x20;

    // when values are tested for ascii validity
    let result = test_values.filter(|v| v.is_printable_ascii())
                            .collect::<Vec<_>>();

    // then all values except newline should be excluded
    assert!(&result == &[10]);
}

#[test]
fn is_printable_ascii_rejects_bytes_above_0x7e() {
    // given a range of test values
    let test_values = 0x7f_u8..=0xff;

    // when values are tested for ascii validity
    let result = test_values.filter(|v| v.is_printable_ascii())
                            .collect::<Vec<_>>();

    // then all values should be excluded
    assert!(&result == &[]);
}

#[test]
fn is_printable_ascii_accepts_bytes_from_0x20_0x7e() {
    // given a range of test values
    let test_values = 0x20_u8..=0x7e;

    // when values are tested for ascii invalidity
    let result = test_values.filter(|v| !v.is_printable_ascii())
                            .collect::<Vec<_>>();

    // then all values should be excluded
    assert!(&result == &[]);
}
