use std::{io::Write, iter::FromIterator};

use rayon::prelude::{FromParallelIterator, ParallelIterator};
use tabled::{width::PriorityMax, Style, TableIteratorExt, Tabled, Width};

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

#[derive(Debug)]
pub struct Table {
    table: tabled::Table,
}

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

impl Table {
    pub fn to_writer(self, options: TableOptions, mut writter: impl Write) -> std::io::Result<()> {
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
        let table = iter
            .into_iter()
            .map(|(name, result)| Inner {
                name,
                result: result.into(),
            })
            .collect();
        Table { table }
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
        Table { table }
    }
}
