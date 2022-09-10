pub mod json;
pub mod table;

pub type Message = (String, Result<Stats, String>);

use std::{
    fmt::Display,
    ops::{Add, AddAssign},
};

use serde::Serialize;
use tabled::Tabled;
use wz_core::*;

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
    //length: usize,
}

impl Stats {
    pub fn new() -> Self {
        Self {
            lines: Some(0),
            words: Some(0),
            characters: Some(0),
            bytes: Some(0),
        }
    }
}

fn display_option<T: Display>(opt: &Option<T>) -> String {
    match opt {
        Some(x) => x.to_string(),
        None => Default::default(),
    }
}

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
    BytesCollector=>bytes
);
