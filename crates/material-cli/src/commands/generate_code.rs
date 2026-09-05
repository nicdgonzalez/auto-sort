use std::path::PathBuf;
use std::{env, fs};

use material_core::{Category, MATERIALS};

use crate::commands::Run;
use crate::profile::{Precedence, Profile};
use crate::sort_key::get_sort_key;

#[derive(clap::Args)]
pub struct GenerateCode {
    /// Path to the newline-separated materials list.
    #[clap(long, short)]
    pub input: PathBuf,

    /// Path to the material ordering file.
    #[clap(long, short)]
    pub profile: PathBuf,
}

impl Run for GenerateCode {
    fn run(self) -> anyhow::Result<()> {
        let material_list = fs::read_to_string(&self.input).expect("failed to read material list");

        let profile_path = env::current_dir()
            .unwrap_or_else(|_err| PathBuf::from("/"))
            .join(&self.profile);

        let profile = Profile::open(&profile_path).expect("failed to load default profile");

        let mut keys = material_list
            .lines()
            .map(|entry| {
                let material = MATERIALS
                    .get(entry)
                    .unwrap_or_else(|| panic!("material not found: {entry}"));

                (entry, get_sort_key(material, &profile))
            })
            .collect::<Vec<_>>();

        keys.sort_by_key(|&(_, k)| k);

        println!(
            "\
package io.github.nicdgonzalez.autosort;

import java.util.HashMap;
import java.util.Map;
import java.util.Optional;

public record SortKey(
        int category,
        int family,
        int shape,
        int modifier) {{
    private static Map<String, SortKey> BY_NAME = new HashMap<>();

    static {{"
        );

        for (entry, sort_key) in keys {
            let category = Category::from_repr(usize::from(sort_key.category)).unwrap();
            let precedence = profile.precedence(category).unwrap_or(&[
                Precedence::Family,
                Precedence::Shape,
                Precedence::Modifier,
            ]);

            let first = sort_key.category;

            let second = match precedence.first().unwrap() {
                Precedence::Family => sort_key.family,
                Precedence::Shape => sort_key.shape,
                Precedence::Modifier => sort_key.modifier,
            };

            let third = match precedence.get(1).unwrap() {
                Precedence::Family => sort_key.family,
                Precedence::Shape => sort_key.shape,
                Precedence::Modifier => sort_key.modifier,
            };

            let fourth = match precedence.get(2).unwrap() {
                Precedence::Family => sort_key.family,
                Precedence::Shape => sort_key.shape,
                Precedence::Modifier => sort_key.modifier,
            };

            println!(
                "        BY_NAME.put(\"{entry}\", new SortKey({first}, {second}, {third}, {fourth}));"
            );
        }

        println!(
            "        BY_NAME.put(\"AIR\", new SortKey({}, 0, 0, 0));",
            u8::MAX
        );

        println!(
            "    }}

    public static Optional<SortKey> byName(String name) {{
        return Optional.ofNullable(BY_NAME.get(name));
    }}
}}"
        );

        Ok(())
    }
}
