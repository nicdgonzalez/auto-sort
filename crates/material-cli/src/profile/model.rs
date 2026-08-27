use std::collections::HashMap;
use std::str::FromStr;

use material_core::{Category, Family, Shape};

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Precedence {
    Category,
    Family,
    Shape,
    Modifier,
}

/// Represents the document.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TomlProfile {
    pub order: Vec<Category>,
    /// Represents each `[category.*]` table (e.g., `[category.BUILDING_BLOCKS]`).
    pub category: HashMap<Category, TomlCategoryConfig>,
    /// Represents each `[family.*]` table (e.g., `[family.OAK]`).
    pub family: HashMap<Family, TomlFamilyConfig>,
}

impl FromStr for TomlProfile {
    type Err = toml::de::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        toml::from_str(s)
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TomlCategoryConfig {
    pub precedence: Vec<Precedence>,
    pub order: Vec<Family>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TomlFamilyConfig {
    pub order: Vec<Shape>,
    /// Represents each `[family.*.modifier.*]` table (e.g., `[family.OAK.modifier.LOG]`).
    #[serde(default)]
    pub shape: HashMap<Shape, TomlModifierConfig>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TomlModifierConfig {
    pub order: Vec<String>,
}

impl Default for TomlModifierConfig {
    fn default() -> Self {
        Self {
            order: vec!["BASE".to_owned()],
        }
    }
}
