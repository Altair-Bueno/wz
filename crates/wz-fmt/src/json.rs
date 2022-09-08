use rayon::{
    iter::FromParallelIterator,
    prelude::{IntoParallelRefIterator, ParallelIterator},
};
use serde::Serialize;
use std::{collections::HashMap, fmt::Display, iter::FromIterator};

use super::{Message, Stats};

#[derive(Serialize, Debug)]
pub struct Json {
    total: Stats,
    summary: HashMap<String, Result<Stats, String>>,
}

impl FromIterator<Message> for Json {
    fn from_iter<T: IntoIterator<Item = Message>>(iter: T) -> Self {
        let summary: HashMap<_, _> = iter.into_iter().collect();
        let total = summary
            .iter()
            .flat_map(|(_, y)| y)
            .cloned()
            .fold(Stats::new(), |mut x, y| {
                x.combine(&y);
                x
            });
        Json { total, summary }
    }
}

impl FromParallelIterator<Message> for Json {
    fn from_par_iter<I>(par_iter: I) -> Self
    where
        I: rayon::prelude::IntoParallelIterator<Item = Message>,
    {
        let summary: HashMap<_, _> = par_iter.into_par_iter().collect();
        let total =
            summary
                .par_iter()
                .flat_map(|(_, y)| y)
                .cloned()
                .reduce(Stats::new, |mut x, y| {
                    x.combine(&y);
                    x
                });
        Json { total, summary }
    }
}

impl Display for Json {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
