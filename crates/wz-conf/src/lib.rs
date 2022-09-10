//! Configuration options for [wz]
//!
//! [wz]: https://crates.io/crates/wz

mod config;
use clap::Parser;
pub use config::*;

/// Loads [wz] configuration options from command line arguments
///
/// [wz]: https://crates.io/crates/wz
pub fn load() -> Config {
    let mut config = Config::parse();
    if ![
        config.bytes,
        config.characters,
        config.words,
        config.lines,
        config.max_line_length,
    ]
    .into_iter()
    .any(|x| x)
    {
        config.lines = true;
        config.words = true;
        config.bytes = true;
    }
    config
}
