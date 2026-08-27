#![warn(
    clippy::correctness,
    clippy::suspicious,
    clippy::complexity,
    clippy::perf,
    clippy::style,
    clippy::pedantic
)]

use clap::Parser as _;

mod commands;
mod profile;
mod sort_key;

fn main() {
    match try_main() {
        Ok(()) => {}
        Err(err) => {
            eprintln!("error: {err}");
        }
    }
}

fn try_main() -> anyhow::Result<()> {
    let args = commands::Cli::parse();
    commands::run(args)
}
