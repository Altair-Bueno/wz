use rstest::*;
use speculoos::assert_that;
use wz_core::Counter;
use wz_utf8::Lines;

include!("__resources__.rs");

#[fixture]
fn lines(#[default(b'\n')] linebreak: u8) -> Lines {
    Lines::with_linebreak(linebreak)
}

#[rstest]
#[case(b"", 0)]
#[case(b"Hello world", 0)]
#[case(b"This is some \n long text", 1)]
#[case(b"\n", 1)]
#[case(b"\n\n", 2)]
#[case(b" \n", 1)]
#[case(empty, 0)]
#[case(french, 0)]
#[case(spanish, 1)]
#[case(small, 1)]
#[case(Lorem_big, 1996)]
#[trace]
fn utf8_lf_contains_the_expected_amount_of_line_breaks(
    mut lines: impl Counter<usize>,
    #[case] utf8_encoded: &[u8],
    #[case] expected: usize,
) {
    let mut obtained = 0;
    lines.count(utf8_encoded);
    lines.output(&mut obtained);

    assert_that!(obtained).is_equal_to(expected)
}

#[rstest]
#[case(b"", 0)]
#[case(b"Hello world\r", 1)]
#[case(b"This is some \n long text", 0)]
#[case(b"\r", 1)]
#[case(b"\r\n", 1)]
#[case(b" \r", 1)]
#[trace]
fn utf8_cr_contains_the_expected_amount_of_line_breaks(
    #[with(b'\r')] mut lines: impl Counter<usize>,
    #[case] utf8_encoded: &[u8],
    #[case] expected: usize,
) {
    let mut obtained = 0;
    lines.count(utf8_encoded);
    lines.output(&mut obtained);

    assert_that!(obtained).is_equal_to(expected)
}
