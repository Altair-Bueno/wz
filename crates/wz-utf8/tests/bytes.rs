use rstest::*;
use speculoos::assert_that;
use wz_core::Counter;
use wz_utf8::Bytes;

include!("__resources__.rs");

#[fixture]
pub fn bytes() -> Bytes {
    Default::default()
}

#[rstest]
#[case(b"", 0)]
#[case(b"Hello world", 11)]
//#[case(b"\xF1", 2)] // TODO ñ fails
#[case(b"\r", 1)]
#[case(b"\xE2\x9D\xA4", 3)]
#[case(empty, 0)]
#[case(french, 61)]
#[case(spanish, 22)]
#[case(small, 18)]
#[case(Lorem_big, 751539)]
#[trace]
fn utf8_has_the_expected_bytesize(
    mut bytes: impl Counter<usize>,
    #[case] utf8_encoded: &[u8],
    #[case] expected: usize,
) {
    let mut obtained = 0;
    bytes.count(utf8_encoded);
    bytes.output(&mut obtained);

    assert_that!(obtained).is_equal_to(expected)
}
