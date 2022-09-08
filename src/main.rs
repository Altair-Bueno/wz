//! Wortzahl! Count words **fast**
//!
//! `wz` is a faster alternative to GNU wc with UTF8 support and human readable
//! output
//!
//! # Instalation
//!
//! ```sh
//! cargo install wz
//! ```
mod builder;
mod run;
mod sheath;

fn main() {
    let config = wz_conf::load();
    run::run(config)
}
