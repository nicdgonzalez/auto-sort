#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize, strum::Display,
)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Category {
    BuildingBlocks,
    ColoredBlocks,
    NaturalBlocks,
    FunctionalBlocks,
    RedstoneBlocks,
    ToolsAndUtilities,
    Combat,
    Ingredients,
    FoodAndDrinks,
    SpawnEggs,
}
