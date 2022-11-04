use std::{io::Write, iter::FromIterator};

use rayon::prelude::{FromParallelIterator, IntoParallelRefIterator, ParallelIterator};
use tabled::{peaker::PriorityMax, Style, TableIteratorExt, Tabled, Width};

use crate::Output;

use super::{Message, Stats};

#[derive(Debug)]
pub struct TableOptions {
    pub style: TableStyle,
}

#[derive(Debug)]
pub enum TableStyle {
    Ascii,
    Psql,
    Markdown,
    Rounded,
    Extended,
}

/// Table representation of wz's output
#[derive(Debug)]
pub struct Table {
    table: tabled::Table,
}

/// Defines a table row
#[derive(Tabled, Debug)]
struct Inner {
    name: String,
    #[tabled(inline)]
    result: Either,
}

#[derive(Tabled, Debug)]
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

impl Output for Table {
    type Options = TableOptions;
    type Error = std::io::Error;
    fn to_writer<W: Write>(
        mut self,
        options: Self::Options,
        mut writter: W,
    ) -> std::io::Result<()> {
        let width = None
            .or_else(|| Some(terminal_size::terminal_size()?.0 .0 as _))
            .unwrap_or(80);

        let table = self.table.with(
            Width::truncate(width)
                .suffix("...")
                .priority::<PriorityMax>(),
        );

        let string = match options.style {
            TableStyle::Ascii => table.with(Style::ascii()).to_string(),
            TableStyle::Psql => table.with(Style::psql()).to_string(),
            TableStyle::Markdown => table.with(Style::markdown()).to_string(),
            TableStyle::Rounded => table.with(Style::rounded()).to_string(),
            TableStyle::Extended => table.with(Style::extended()).to_string(),
        };

        writter.write_all(string.as_bytes())
    }
}

impl FromIterator<Message> for Table {
    fn from_iter<T: IntoIterator<Item = Message>>(iter: T) -> Self {
        let mut rows: Vec<_> = iter
            .into_iter()
            .map(|(name, result)| Inner {
                name,
                result: result.into(),
            })
            .collect();

        let total = rows
            .iter()
            .flat_map(|x| {
                if let Either::Stats { stats } = x.result {
                    Some(stats)
                } else {
                    None
                }
            })
            .fold(Stats::identity(), |x, y| x + y);
        rows.push(Inner {
            name: "Total".to_owned(),
            result: Either::Stats { stats: total },
        });

        Table {
            table: rows.table(),
        }
    }
}

impl FromParallelIterator<Message> for Table {
    fn from_par_iter<I>(par_iter: I) -> Self
    where
        I: rayon::prelude::IntoParallelIterator<Item = Message>,
    {
        let mut rows: Vec<_> = par_iter
            .into_par_iter()
            .map(|(name, result)| Inner {
                name,
                result: result.into(),
            })
            .collect();

        let total = rows
            .par_iter()
            .flat_map(|x| {
                if let Either::Stats { stats } = x.result {
                    Some(stats)
                } else {
                    None
                }
            })
            .reduce(Stats::identity, |x, y| x + y);
        rows.push(Inner {
            name: "Total".to_owned(),
            result: Either::Stats { stats: total },
        });

        Table {
            table: rows.table(),
        }
    }
}
