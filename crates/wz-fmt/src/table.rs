use std::{fmt::Display, iter::FromIterator};

use rayon::prelude::{FromParallelIterator, ParallelIterator};
use tabled::{width::PriorityMax, Style, TableIteratorExt, Tabled, Width};

use super::{Message, Stats};

pub struct Table {
    table: tabled::Table,
}

#[derive(Tabled)]
struct Inner {
    name: String,
    #[tabled(inline)]
    result: Either,
}

#[derive(Tabled)]
enum Either {
    #[tabled(inline)]
    Stats {
        #[tabled(inline)]
        stats: Stats,
    },
    #[tabled(inline)]
    Error { error: String },
}

impl From<Result<Stats, String>> for Either {
    fn from(result: Result<Stats, String>) -> Self {
        match result {
            Ok(stats) => Self::Stats { stats },
            Err(error) => Self::Error { error },
        }
    }
}

fn configure(table: tabled::Table) -> tabled::Table {
    let width = terminal_size::terminal_size()
        .map(|(terminal_size::Width(w), _)| w)
        .unwrap_or(80) as usize;
    table
        .with(
            Width::truncate(width)
                .suffix("...")
                .priority::<PriorityMax>(),
        )
        .with(Style::psql())
}

impl FromIterator<Message> for Table {
    fn from_iter<T: IntoIterator<Item = Message>>(iter: T) -> Self {
        let table = iter
            .into_iter()
            .map(|(name, result)| Inner {
                name,
                result: result.into(),
            })
            .collect();
        Table {
            table: configure(table),
        }
    }
}

impl FromParallelIterator<Message> for Table {
    fn from_par_iter<I>(par_iter: I) -> Self
    where
        I: rayon::prelude::IntoParallelIterator<Item = Message>,
    {
        let table = par_iter
            .into_par_iter()
            .map(|(name, result)| Inner {
                name,
                result: result.into(),
            })
            .collect::<Vec<_>>()
            .table();
        Table {
            table: configure(table),
        }
    }
}

impl Display for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.table)
    }
}
