use rstest::*;
use speculoos::*;

use wz_utf8::MaxLineLength;

include!("__resources__.rs");

#[fixture]
fn counter() -> MaxLineLength {
    MaxLineLength::line_feed()
}

#[rstest]
#[case(b"", 0)]
#[case(b"Hello world", 11)]
#[case(b"H\nello", 4)]
#[case(b"Hello\n wor", 5)]
#[case(b"One\ntwo\t\nanother", 7)]
#[case(empty, 0)]
#[case(french, 58)]
#[case(spanish, 18)]
#[case(small, 17)]
#[case(Lorem_big, 1142)]
#[trace]
fn utf8_contains_the_expected_max_line_length(
    mut counter: impl wz_core::Counter<usize>,
    #[case] utf8_encoded: &[u8],
    #[case] expected: usize,
) {
    let mut obtained = 0;
    counter.count(utf8_encoded);
    counter.output(&mut obtained);

    assert_that!(obtained).is_equal_to(expected)
}
