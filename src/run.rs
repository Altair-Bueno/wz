use std::{
    io::{stdin, Read},
    iter::{once, FromIterator},
    sync::Arc,
};

use bumpalo::Bump;
use rayon::{
    iter::ParallelIterator,
    prelude::{FromParallelIterator, IntoParallelIterator},
};
use wz_conf::{Config, Encoding};
use wz_core::Counter;
use wz_fmt::{Message, Stats};

use crate::builder::{Builder, BuilderUtf8, Filter};

/// Bumpalo arena byte size. 10KB
const BUMP_BUFFER_SIZE: usize = 1_024 * 10;
/// Minimum files required to use the Rayon runtime
const MIN_FILES_RAYON: usize = 1;

pub fn run<T>(
    Config {
        from_stdin,
        lines,
        characters,
        words,
        bytes,
        newline,
        files,
        encoding,
        max_line_length,
        ..
    }: Config,
) -> T
where
    T: FromParallelIterator<Message> + FromIterator<Message>,
{
    // Create a Sheath builder filter
    let options = Filter {
        lines,
        characters,
        words,
        bytes,
        newline: newline.into(),
        max_line_length,
    };
    // Sheath builder
    let builder: Arc<dyn Builder + Send + Sync> = match encoding {
        Encoding::UTF8 => Arc::new(BuilderUtf8::from(options)),
    };

    // Runtime to use
    if files.len() > MIN_FILES_RAYON {
        // Rayon runtime
        files
            .into_par_iter()
            .map_with(builder, |builder, y| process_path(builder, y))
            .collect()
    } else if !files.is_empty() {
        // Rust's iterator runtime
        files
            .into_iter()
            .map(move |y| process_path(builder.clone(), y))
            .collect()
    } else if from_stdin {
        // STDIN file list
        stdin()
            .lines()
            .flatten()
            .map(move |path| process_path(builder.clone(), path))
            .collect()
    } else {
        // STDIN
        once(stdin())
            .map(move |file| {
                let bump = Bump::with_capacity(BUMP_BUFFER_SIZE);
                let counter = builder.build(&bump);
                let buffer = bump.alloc_slice_fill_default(bump.chunk_capacity());

                (
                    "<STDIN>".into(),
                    process_reader(file, counter, buffer).map_err(|x| x.to_string()),
                )
            })
            .collect()
    }
}

fn process_path(builder: impl AsRef<dyn Builder + Send + Sync>, path: String) -> Message {
    fn inner(builder: impl AsRef<dyn Builder + Send + Sync>, path: &str) -> std::io::Result<Stats> {
        let file = std::fs::File::open(path)?;
        let bump = Bump::with_capacity(BUMP_BUFFER_SIZE);
        let counter = builder.as_ref().build(&bump);
        let buffer = bump.alloc_slice_fill_default(bump.chunk_capacity());

        process_reader(file, counter, buffer)
    }

    let result = inner(builder, &path);

    (path, result.map_err(|x| x.to_string()))
}

fn process_reader(
    mut reader: impl Read,
    counter: &mut dyn Counter<Stats>,
    buffer: &mut [u8],
) -> std::io::Result<Stats> {
    loop {
        let ammount = reader.read(buffer)?;
        if ammount == 0 {
            let mut stats = Stats::default();
            counter.output(&mut stats);
            return Ok(stats);
        } else {
            counter.count(&buffer[..ammount])
        }
    }
}
