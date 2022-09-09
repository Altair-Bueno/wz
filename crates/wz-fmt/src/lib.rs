pub mod json;
pub mod table;

pub type Message = (String, Result<Stats, String>);

use std::ops::{Add, AddAssign};

use serde::Serialize;
use tabled::Tabled;
use wz_core::*;

#[derive(Debug, Eq, PartialEq, Clone, Copy, Serialize, Default, Tabled)]
pub struct Stats {
    lines: usize,
    words: usize,
    characters: usize,
    bytes: usize,
    //length: usize,
}

macro_rules! add_name {
    ( $($x1:ident $x2:ident $name:tt ), * ) => {
        $(let $name = $x1.$name + $x2.$name;)*
    };
}

impl Add for Stats {
    type Output = Stats;

    fn add(self, rhs: Self) -> Self::Output {
        add_name!(
            self rhs lines,
            self rhs words,
            self rhs characters,
            self rhs bytes
        );
        Self {
            lines,
            words,
            characters,
            bytes,
        }
    }
}

impl AddAssign for Stats {
    fn add_assign(&mut self, rhs: Self) {
        *self = self.clone() + rhs
    }
}

// impl LinesCollector for Stats {
//     fn collect(&mut self, count: usize) {
//         self.lines = count;
//     }
// }
macro_rules! impl_collector_stats {
    ( $($name:ty=>$field:tt), *) => {
        $(
            impl $name for Stats {
                fn collect(&mut self, count: usize) {
                    self.$field = count;
                }
            }
        )*
    };
}

impl_collector_stats!(
    LinesCollector=>lines,
    WordsCollector=>words,
    CharsCollector=>characters,
    BytesCollector=>bytes
);

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
