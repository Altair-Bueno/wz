//! # Wortzahl! Count words **fast**
//!
//! `wz` is a faster alternative to GNU wc with UTF8 support and human readable
//! output, written in Rust
//!
//! ![wz](resources/wz.png)
//!
//! # Installation
//!
//! ## Cargo
//!
//! ```sh
//! # Latest release
//! cargo install wz
//! # Latest commit
//! cargo install --git https://github.com/Altair-Bueno/wz.git
//! ```
//!
//! # Performance
//!
//! See [BENCH.md](BENCH.md)
//!
//! # Usage
//!
//! `wz` is not backwards compatible with `wc`, but most command line options are
//! left intact
//!
//! Run `wz --help` to see the full list of options

use std::{io::stdout, error::Error};

use wz_fmt::{
    json::Json,
    table::{Table, TableStyle},
    Output,
};
mod builder;
mod run;
mod sheath;

fn main() -> Result<(), Box<dyn Error>> {
    let config = wz_conf::load();
    match config.output {
        wz_conf::Format::Json => {
            let json: Json = crate::run::run(config);
            json.to_writer((), stdout())?;
        }
        style => {
            let style = match style {
                wz_conf::Format::Ascii => TableStyle::Ascii,
                wz_conf::Format::Psql => TableStyle::Psql,
                wz_conf::Format::Markdown => TableStyle::Markdown,
                wz_conf::Format::Rounded => TableStyle::Rounded,
                wz_conf::Format::Extended => TableStyle::Extended,
                wz_conf::Format::Json => unreachable!(),
            };
            let options = wz_fmt::table::TableOptions { style };
            let table: Table = crate::run::run(config);
            table.to_writer(options, stdout())?;
        }
    };
    Ok(())
}
