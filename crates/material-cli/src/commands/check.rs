use std::fs;
use std::path::PathBuf;

use anyhow::{Context as _, bail};
use material_core::MATERIALS;

use crate::commands::Run;

#[derive(clap::Args)]
pub struct Check {
    /// Path to the newline-separated materials list.
    #[clap(long, short)]
    pub input: PathBuf,
}

impl Run for Check {
    fn run(self) -> anyhow::Result<()> {
        let content = fs::read_to_string(&self.input).context("failed to read material list")?;
        let total_input = i64::try_from(content.lines().count()).expect("usize overflowed u64");
        let total_output = i64::try_from(MATERIALS.len()).expect("usize overflowed u64");

        println!("Length of input: {total_input}");
        println!("Total implemented: {total_output}");
        println!("Remaining: {}", total_input - total_output);

        if let Some((count, material_id)) = content
            .lines()
            .enumerate()
            .find(|(_, material_id)| !MATERIALS.contains_key(*material_id))
        {
            println!("Passed: {count} (in a row, alphabetically)");
            bail!("{material_id:?} not implemented");
        }

        println!("All checks passed!");

        Ok(())
    }
}
