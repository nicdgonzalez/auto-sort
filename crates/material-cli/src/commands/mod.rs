mod check;
mod display;
mod generate_code;

pub trait Run {
    /// Executes the subcommand.
    fn run(self) -> anyhow::Result<()>;
}

#[derive(clap::Parser)]
pub struct Cli {
    #[clap(subcommand)]
    pub subcommand: Subcommand,
}

#[derive(clap::Subcommand)]
pub enum Subcommand {
    Check(check::Check),
    Display(display::Display),
    GenerateCode(generate_code::GenerateCode),
}

/// Executes the user-selected subcommand.
pub fn run(args: Cli) -> anyhow::Result<()> {
    match args.subcommand {
        Subcommand::Check(cmd) => cmd.run(),
        Subcommand::Display(cmd) => cmd.run(),
        Subcommand::GenerateCode(cmd) => cmd.run(),
    }
}
