use std::collections::HashSet;

use heck::ToShoutySnakeCase as _;
use material_core::{Category, Family, MATERIALS};

use crate::commands::Run;

#[derive(clap::Args)]
pub struct Display;

impl Run for Display {
    fn run(self) -> anyhow::Result<()> {
        display_families(Category::BuildingBlocks);
        display_families(Category::ColoredBlocks);
        display_families(Category::NaturalBlocks);
        display_families(Category::FunctionalBlocks);
        display_families(Category::RedstoneBlocks);
        display_families(Category::ToolsAndUtilities);
        display_families(Category::Combat);
        display_families(Category::FoodAndDrinks);
        display_families(Category::Ingredients);
        display_families(Category::SpawnEggs);

        Ok(())
    }
}

fn display_families(target: Category) {
    println!("{}", target.to_string().to_shouty_snake_case());

    for entry in MATERIALS
        .iter()
        .filter_map(|m| (m.1.category == target).then_some(m.1.family))
        .collect::<HashSet<Family>>()
    {
        let id = entry.to_string().to_shouty_snake_case();
        println!("{id:?}");
    }

    println!();
}
