use rayon::{
    iter::FromParallelIterator,
    prelude::{IntoParallelRefIterator, ParallelIterator},
};
use serde::Serialize;
use std::{collections::HashMap, io::Write, iter::FromIterator};

use crate::Output;

use super::{Message, Stats};

/// JSON representation of wz's output
#[derive(Serialize, Debug)]
pub struct Json {
    total: Stats,
    summary: HashMap<String, Result<Stats, String>>,
}

impl Output for Json {
    type Options = ();
    type Error = serde_json::Error;
    fn to_writer<W: Write>(self, _: Self::Options, writter: W) -> serde_json::Result<()> {
        serde_json::to_writer(writter, &self)
    }
}

impl FromIterator<Message> for Json {
    fn from_iter<T: IntoIterator<Item = Message>>(iter: T) -> Self {
        let summary: HashMap<_, _> = iter.into_iter().collect();
        let total = summary
            .iter()
            .flat_map(|(_, y)| y)
            .cloned()
            .fold(Stats::identity(), |x, y| x + y);
        Json { total, summary }
    }
}

impl FromParallelIterator<Message> for Json {
    fn from_par_iter<I>(par_iter: I) -> Self
    where
        I: rayon::prelude::IntoParallelIterator<Item = Message>,
    {
        let summary: HashMap<_, _> = par_iter.into_par_iter().collect();
        let total = summary
            .par_iter()
            .flat_map(|(_, y)| y)
            .copied()
            .reduce(Stats::identity, |x, y| x + y);
        Json { total, summary }
    }
}
