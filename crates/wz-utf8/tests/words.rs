use rstest::*;
use speculoos::assert_that;
use wz_core::Counter;
use wz_utf8::Words;

include!("__resources__.rs");

#[fixture]
fn words() -> Words {
    Default::default()
}

#[rstest]
#[case(b"", 0)]
#[case(b"Hello", 1)]
#[case(b"Hello ", 1)]
#[case(b" Hello", 1)]
#[case(b" Hello ", 1)]
#[case(b"Hello world ", 2)]
#[case(b"Hello\nworld ", 2)]
#[case(b"Hello\rworld ", 2)]
#[case(empty, 0)]
#[case(french, 10)]
#[case(spanish, 3)]
#[case(small, 3)]
#[case(Lorem_big, 111618)]
#[trace]
fn utf8_contains_the_expected_amount_of_words(
    mut words: impl Counter<usize>,
    #[case] utf8_encoded: &[u8],
    #[case] expected: usize,
) {
    let mut obtained = 0;
    words.count(utf8_encoded);
    words.output(&mut obtained);

    assert_that!(obtained).is_equal_to(expected)
}
