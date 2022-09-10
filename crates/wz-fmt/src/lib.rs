//! Types for generating wz's output
//!
//! [wz]: https://crates.io/crates/wz

pub mod json;
pub mod table;

pub type Message = (String, Result<Stats, String>);

pub trait Output {
    type Options;
    type Error;
    fn to_writer<W: Write>(self, options: Self::Options, writter: W) -> Result<(), Self::Error>;
}

use std::{
    fmt::Display,
    io::Write,
    ops::{Add, AddAssign},
};

use serde::Serialize;
use tabled::Tabled;
use wz_core::*;

/// Collector for [wz-utf8] counters
///
/// [wz-utf8]: https://crates.io/crates/wz-utf8
#[derive(Debug, Eq, PartialEq, Clone, Copy, Serialize, Default, Tabled)]
pub struct Stats {
    #[tabled(display_with = "display_option")]
    #[serde(skip_serializing_if = "Option::is_none")]
    lines: Option<usize>,
    #[tabled(display_with = "display_option")]
    #[serde(skip_serializing_if = "Option::is_none")]
    words: Option<usize>,
    #[tabled(display_with = "display_option")]
    #[serde(skip_serializing_if = "Option::is_none")]
    characters: Option<usize>,
    #[tabled(display_with = "display_option")]
    #[serde(skip_serializing_if = "Option::is_none")]
    bytes: Option<usize>,
    #[tabled(display_with = "display_option")]
    #[serde(skip_serializing_if = "Option::is_none")]
    max: Option<usize>,
}

impl Stats {
    // Creates a new identity stats
    pub fn identity() -> Self {
        Self {
            lines: Some(0),
            words: Some(0),
            characters: Some(0),
            bytes: Some(0),
            max: Some(0),
        }
    }
}
/// Defines display representation on [`Table`]
///
/// [`Table`]: crate::table::Table
fn display_option<T: Display>(opt: &Option<T>) -> String {
    match opt {
        Some(x) => x.to_string(),
        None => Default::default(),
    }
}

/// Creates a variable that combines two values with the same name on two
/// structs
macro_rules! add_name {
    ( $($x1:ident $x2:ident $name:tt ), * ) => {
        $(let $name = $x1.$name.zip($x2.$name).map(|(x,y)|x+y) ;)*
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
        let max_line_length = self.max.zip(rhs.max).map(|(x, y)| std::cmp::max(x, y));
        Self {
            lines,
            words,
            characters,
            bytes,
            max: max_line_length,
        }
    }
}

impl AddAssign for Stats {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

/// Implements a collector-like trait on stats
macro_rules! impl_collector_stats {
    ( $($name:ty=>$field:tt), *) => {
        $(
            impl $name for Stats {
                fn collect(&mut self, count: usize) {
                    self.$field = Some(count);
                }
            }
        )*
    };
}

impl_collector_stats!(
    LinesCollector=>lines,
    WordsCollector=>words,
    CharsCollector=>characters,
    BytesCollector=>bytes,
    MaxLineLengthCollector=>max
);
