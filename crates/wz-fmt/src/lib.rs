pub mod json;
pub mod table;

pub type Message = (String, Result<Stats, String>);

use serde::Serialize;
use std::option::Option::Some;
use std::{cmp::max, fmt::Display};
use tabled::Tabled;
use wz_core::*;

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Default, Tabled)]
pub struct Stats {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[tabled(display_with = "display_option")]
    lines: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[tabled(display_with = "display_option")]
    words: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[tabled(display_with = "display_option")]
    characters: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[tabled(display_with = "display_option")]
    bytes: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[tabled(display_with = "display_option")]
    length: Option<usize>,
    //columns: Columns,
}

fn display_option<D: Display>(option: &Option<D>) -> String {
    match option {
        Some(x) => format!("{x}"),
        None => "".to_owned(),
    }
}

impl Stats {
    /// Creates a new `Stats` struct with the given stats.
    pub fn new() -> Stats {
        Stats {
            lines: Some(0),
            words: Some(0),
            characters: Some(0),
            bytes: Some(0),
            length: Some(0),
        }
    }

    pub fn combine(&mut self, s: &Stats) {
        let combine_using = |a, b, f: fn(usize, usize) -> usize| match (a, b) {
            (Some(x), Some(y)) => Some(f(x, y)),
            _ => None,
        };

        *self = Stats {
            lines: combine_using(self.lines, s.lines, std::ops::Add::add),
            words: combine_using(self.words, s.words, std::ops::Add::add),
            characters: combine_using(self.characters, s.characters, std::ops::Add::add),
            bytes: combine_using(self.bytes, s.bytes, std::ops::Add::add),
            length: combine_using(self.length, s.length, max),
        }
    }
}

impl LinesCollector for Stats {
    fn collect(&mut self, count: usize) {
        self.lines = Some(count);
    }
}
impl WordsCollector for Stats {
    fn collect(&mut self, count: usize) {
        self.words = Some(count)
    }
}

impl CharsCollector for Stats {
    fn collect(&mut self, count: usize) {
        self.characters = Some(count)
    }
}
impl BytesCollector for Stats {
    fn collect(&mut self, count: usize) {
        self.bytes = Some(count)
    }
}

#[cfg(all(test, feature = "serde"))]
mod serde_test {
    use rstest::*;
    use speculoos::assert_that;

    use crate::Stats;

    #[rstest]
    fn stats_generates_the_expected_json_output() {
        let expected = r#"{"lines":0,"words":10}"#.to_owned();
        let stats = Stats::new(Some(0), Some(10), None, None, None);

        let obtained = serde_json::to_string(&stats).unwrap();

        assert_that!(obtained).is_equal_to(expected)
    }
}
