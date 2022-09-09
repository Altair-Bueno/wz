include!("src/config.rs");

use std::path::Path;

use clap::{App, CommandFactory};
use clap_complete::Shell;

fn main() -> std::io::Result<()> {
    let cmd = Config::command();
    let out_dir = std::path::PathBuf::from(
        std::env::var_os("WZ_MISC_FOLDER")
            .or_else(|| std::env::var_os("OUT_DIR"))
            .ok_or_else(|| std::io::ErrorKind::NotFound)?,
    );

    man_gen(out_dir.as_path(), cmd.clone())?;
    completions_gen(out_dir.as_path(), cmd)?;

    Ok(())
}

fn completions_gen(out_dir: &Path, mut cmd: App) -> std::io::Result<()> {
    for shell in Shell::value_variants().iter() {
        clap_complete::generate_to(shell.clone(), &mut cmd, "wz", out_dir)?;
    }

    Ok(())
}

fn man_gen(out_dir: &Path, cmd: App) -> std::io::Result<()> {
    let man = clap_mangen::Man::new(cmd);
    let mut buffer: Vec<u8> = Default::default();
    man.render(&mut buffer)?;

    std::fs::write(out_dir.join("wz.1"), buffer)?;

    Ok(())
}
