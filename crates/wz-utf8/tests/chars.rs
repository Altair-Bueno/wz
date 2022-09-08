use rstest::*;
use speculoos::*;

use wz_utf8::Chars;

include!("__resources__.rs");

#[fixture]
fn chars() -> Chars {
    Default::default()
}

#[rstest]
#[case(b"", 0)]
#[case(b" ", 1)]
#[case(b"\xF1", 1)]
#[case(b"/", 1)]
#[case(b"hello", 5)]
#[case(b"\xF1\xF3", 2)]
#[case(empty, 0)]
#[case(french, 58)]
#[case(spanish, 19)]
#[case(small, 18)]
#[case(Lorem_big, 751539)]
#[trace]
fn utf8_contains_the_expected_amount_of_characters(
    mut chars: impl wz_core::Counter<usize>,
    #[case] utf8_encoded: &[u8],
    #[case] expected: usize,
) {
    let mut obtained = 0;
    chars.count(utf8_encoded);
    chars.output(&mut obtained);

    assert_that!(obtained).is_equal_to(expected)
}
