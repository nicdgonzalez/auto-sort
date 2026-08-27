use crate::category::Category as C;
use crate::modifier::Modifier as M;
use crate::shape::Shape as S;

#[derive(
    Debug,
    Clone,
    Copy,
    Hash,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    strum::Display,
    material_derive::GenerateMaterials,
)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Family {
    // => Building Blocks
    #[material(items = [
        (C::BuildingBlocks, S::Log, []),
        (C::BuildingBlocks, S::Log, [M::Stripped]),
        (C::BuildingBlocks, S::Wood, []),
        (C::BuildingBlocks, S::Wood, [M::Stripped]),
        (C::BuildingBlocks, S::Planks, []),
        (C::BuildingBlocks, S::Stairs, []),
        (C::BuildingBlocks, S::Slab, []),
        (C::BuildingBlocks, S::Fence, []),
        (C::BuildingBlocks, S::FenceGate, []),
        (C::BuildingBlocks, S::Door, []),
        (C::BuildingBlocks, S::Trapdoor, []),
        (C::BuildingBlocks, S::PressurePlate, []),
        (C::BuildingBlocks, S::Button, []),

        (C::NaturalBlocks, S::Leaves, []),
        (C::NaturalBlocks, S::Sapling, []),

        (C::FunctionalBlocks, S::Shelf, []),
        (C::FunctionalBlocks, S::Sign, []),
        (C::FunctionalBlocks, S::HangingSign, []),

        (C::ToolsAndUtilities, S::Boat, []),
        (C::ToolsAndUtilities, S::ChestBoat, []),
    ])]
    Oak,

    #[material(items = [
        (C::BuildingBlocks, S::Log, []),
        (C::BuildingBlocks, S::Log, [M::Stripped]),
        (C::BuildingBlocks, S::Wood, []),
        (C::BuildingBlocks, S::Wood, [M::Stripped]),
        (C::BuildingBlocks, S::Planks, []),
        (C::BuildingBlocks, S::Stairs, []),
        (C::BuildingBlocks, S::Slab, []),
        (C::BuildingBlocks, S::Fence, []),
        (C::BuildingBlocks, S::FenceGate, []),
        (C::BuildingBlocks, S::Door, []),
        (C::BuildingBlocks, S::Trapdoor, []),
        (C::BuildingBlocks, S::PressurePlate, []),
        (C::BuildingBlocks, S::Button, []),

        (C::NaturalBlocks, S::Leaves, []),
        (C::NaturalBlocks, S::Sapling, []),

        (C::FunctionalBlocks, S::Shelf, []),
        (C::FunctionalBlocks, S::Sign, []),
        (C::FunctionalBlocks, S::HangingSign, []),

        (C::ToolsAndUtilities, S::Boat, []),
        (C::ToolsAndUtilities, S::ChestBoat, []),
    ])]
    Spruce,

    #[material(items = [
        (C::BuildingBlocks, S::Log, []),
        (C::BuildingBlocks, S::Log, [M::Stripped]),
        (C::BuildingBlocks, S::Wood, []),
        (C::BuildingBlocks, S::Wood, [M::Stripped]),
        (C::BuildingBlocks, S::Planks, []),
        (C::BuildingBlocks, S::Stairs, []),
        (C::BuildingBlocks, S::Slab, []),
        (C::BuildingBlocks, S::Fence, []),
        (C::BuildingBlocks, S::FenceGate, []),
        (C::BuildingBlocks, S::Door, []),
        (C::BuildingBlocks, S::Trapdoor, []),
        (C::BuildingBlocks, S::PressurePlate, []),
        (C::BuildingBlocks, S::Button, []),

        (C::NaturalBlocks, S::Leaves, []),
        (C::NaturalBlocks, S::Sapling, []),

        (C::FunctionalBlocks, S::Shelf, []),
        (C::FunctionalBlocks, S::Sign, []),
        (C::FunctionalBlocks, S::HangingSign, []),

        (C::ToolsAndUtilities, S::Boat, []),
        (C::ToolsAndUtilities, S::ChestBoat, []),
    ])]
    Birch,

    #[material(items = [
        (C::BuildingBlocks, S::Log, []),
        (C::BuildingBlocks, S::Log, [M::Stripped]),
        (C::BuildingBlocks, S::Wood, []),
        (C::BuildingBlocks, S::Wood, [M::Stripped]),
        (C::BuildingBlocks, S::Planks, []),
        (C::BuildingBlocks, S::Stairs, []),
        (C::BuildingBlocks, S::Slab, []),
        (C::BuildingBlocks, S::Fence, []),
        (C::BuildingBlocks, S::FenceGate, []),
        (C::BuildingBlocks, S::Door, []),
        (C::BuildingBlocks, S::Trapdoor, []),
        (C::BuildingBlocks, S::PressurePlate, []),
        (C::BuildingBlocks, S::Button, []),

        (C::NaturalBlocks, S::Leaves, []),
        (C::NaturalBlocks, S::Sapling, []),

        (C::FunctionalBlocks, S::Shelf, []),
        (C::FunctionalBlocks, S::Sign, []),
        (C::FunctionalBlocks, S::HangingSign, []),

        (C::ToolsAndUtilities, S::Boat, []),
        (C::ToolsAndUtilities, S::ChestBoat, []),
    ])]
    Jungle,

    #[material(items = [
        (C::BuildingBlocks, S::Log, []),
        (C::BuildingBlocks, S::Log, [M::Stripped]),
        (C::BuildingBlocks, S::Wood, []),
        (C::BuildingBlocks, S::Wood, [M::Stripped]),
        (C::BuildingBlocks, S::Planks, []),
        (C::BuildingBlocks, S::Stairs, []),
        (C::BuildingBlocks, S::Slab, []),
        (C::BuildingBlocks, S::Fence, []),
        (C::BuildingBlocks, S::FenceGate, []),
        (C::BuildingBlocks, S::Door, []),
        (C::BuildingBlocks, S::Trapdoor, []),
        (C::BuildingBlocks, S::PressurePlate, []),
        (C::BuildingBlocks, S::Button, []),

        (C::NaturalBlocks, S::Leaves, []),
        (C::NaturalBlocks, S::Sapling, []),

        (C::FunctionalBlocks, S::Shelf, []),
        (C::FunctionalBlocks, S::Sign, []),
        (C::FunctionalBlocks, S::HangingSign, []),

        (C::ToolsAndUtilities, S::Boat, []),
        (C::ToolsAndUtilities, S::ChestBoat, []),
    ])]
    Acacia,

    #[material(items = [
        (C::BuildingBlocks, S::Log, []),
        (C::BuildingBlocks, S::Log, [M::Stripped]),
        (C::BuildingBlocks, S::Wood, []),
        (C::BuildingBlocks, S::Wood, [M::Stripped]),
        (C::BuildingBlocks, S::Planks, []),
        (C::BuildingBlocks, S::Stairs, []),
        (C::BuildingBlocks, S::Slab, []),
        (C::BuildingBlocks, S::Fence, []),
        (C::BuildingBlocks, S::FenceGate, []),
        (C::BuildingBlocks, S::Door, []),
        (C::BuildingBlocks, S::Trapdoor, []),
        (C::BuildingBlocks, S::PressurePlate, []),
        (C::BuildingBlocks, S::Button, []),

        (C::NaturalBlocks, S::Leaves, []),
        (C::NaturalBlocks, S::Sapling, []),

        (C::FunctionalBlocks, S::Shelf, []),
        (C::FunctionalBlocks, S::Sign, []),
        (C::FunctionalBlocks, S::HangingSign, []),

        (C::ToolsAndUtilities, S::Boat, []),
        (C::ToolsAndUtilities, S::ChestBoat, []),
    ])]
    DarkOak,

    #[material(items = [
        (C::BuildingBlocks, S::Log, []),
        (C::BuildingBlocks, S::Log, [M::Stripped]),
        (C::BuildingBlocks, S::Wood, []),
        (C::BuildingBlocks, S::Wood, [M::Stripped]),
        (C::BuildingBlocks, S::Planks, []),
        (C::BuildingBlocks, S::Stairs, []),
        (C::BuildingBlocks, S::Slab, []),
        (C::BuildingBlocks, S::Fence, []),
        (C::BuildingBlocks, S::FenceGate, []),
        (C::BuildingBlocks, S::Door, []),
        (C::BuildingBlocks, S::Trapdoor, []),
        (C::BuildingBlocks, S::PressurePlate, []),
        (C::BuildingBlocks, S::Button, []),

        (C::NaturalBlocks, S::Roots, []),
        (C::NaturalBlocks, S::Roots, [M::Muddy]),
        (C::NaturalBlocks, S::Leaves, []),
        (C::NaturalBlocks, S::Propagule, []),

        (C::FunctionalBlocks, S::Shelf, []),
        (C::FunctionalBlocks, S::Sign, []),
        (C::FunctionalBlocks, S::HangingSign, []),

        (C::ToolsAndUtilities, S::Boat, []),
        (C::ToolsAndUtilities, S::ChestBoat, []),
    ])]
    Mangrove,

    #[material(items = [
        (C::BuildingBlocks, S::Log, []),
        (C::BuildingBlocks, S::Log, [M::Stripped]),
        (C::BuildingBlocks, S::Wood, []),
        (C::BuildingBlocks, S::Wood, [M::Stripped]),
        (C::BuildingBlocks, S::Planks, []),
        (C::BuildingBlocks, S::Stairs, []),
        (C::BuildingBlocks, S::Slab, []),
        (C::BuildingBlocks, S::Fence, []),
        (C::BuildingBlocks, S::FenceGate, []),
        (C::BuildingBlocks, S::Door, []),
        (C::BuildingBlocks, S::Trapdoor, []),
        (C::BuildingBlocks, S::PressurePlate, []),
        (C::BuildingBlocks, S::Button, []),

        (C::NaturalBlocks, S::Leaves, []),
        (C::NaturalBlocks, S::Sapling, []),

        (C::FunctionalBlocks, S::Shelf, []),
        (C::FunctionalBlocks, S::Sign, []),
        (C::FunctionalBlocks, S::HangingSign, []),

        (C::ToolsAndUtilities, S::Boat, []),
        (C::ToolsAndUtilities, S::ChestBoat, []),
    ])]
    Cherry,

    #[material(items = [
        (C::BuildingBlocks, S::Log, []),
        (C::BuildingBlocks, S::Log, [M::Stripped]),
        (C::BuildingBlocks, S::Wood, []),
        (C::BuildingBlocks, S::Wood, [M::Stripped]),
        (C::BuildingBlocks, S::Planks, []),
        (C::BuildingBlocks, S::Stairs, []),
        (C::BuildingBlocks, S::Slab, []),
        (C::BuildingBlocks, S::Fence, []),
        (C::BuildingBlocks, S::FenceGate, []),
        (C::BuildingBlocks, S::Door, []),
        (C::BuildingBlocks, S::Trapdoor, []),
        (C::BuildingBlocks, S::PressurePlate, []),
        (C::BuildingBlocks, S::Button, []),

        (C::NaturalBlocks, S::Leaves, []),
        (C::NaturalBlocks, S::Sapling, []),

        (C::FunctionalBlocks, S::Shelf, []),
        (C::FunctionalBlocks, S::Sign, []),
        (C::FunctionalBlocks, S::HangingSign, []),

        (C::ToolsAndUtilities, S::Boat, []),
        (C::ToolsAndUtilities, S::ChestBoat, []),
    ])]
    PaleOak,

    #[material(items = [
        (C::NaturalBlocks, S::Base, []),
        (C::NaturalBlocks, S::Base, [M::Flowering]),
        (C::NaturalBlocks, S::Leaves, []),
        (C::NaturalBlocks, S::Leaves, [M::Flowering]),
    ])]
    Azalea,

    #[material(items = [
        (C::BuildingBlocks, S::Block, []),
        (C::BuildingBlocks, S::Block, [M::Stripped]),
        (C::BuildingBlocks, S::Planks, []),
        (C::BuildingBlocks, S::Stairs, []),
        (C::BuildingBlocks, S::Slab, []),
        (C::BuildingBlocks, S::Fence, []),
        (C::BuildingBlocks, S::FenceGate, []),
        (C::BuildingBlocks, S::Door, []),
        (C::BuildingBlocks, S::Trapdoor, []),
        (C::BuildingBlocks, S::PressurePlate, []),
        (C::BuildingBlocks, S::Button, []),

        (C::NaturalBlocks, S::Base, []),

        (C::FunctionalBlocks, S::Shelf, []),
        (C::FunctionalBlocks, S::Sign, []),
        (C::FunctionalBlocks, S::HangingSign, []),

        (C::ToolsAndUtilities, S::ChestRaft, []),
        (C::ToolsAndUtilities, S::Raft, []),
    ])]
    Bamboo,

    #[material(items = [
        (C::BuildingBlocks, S::Base, []),
        (C::BuildingBlocks, S::Stairs, []),
        (C::BuildingBlocks, S::Slab, []),
    ])]
    BambooMosaic,

    #[material(items = [
        (C::BuildingBlocks, S::Stem, []),
        (C::BuildingBlocks, S::Stem, [M::Stripped]),
        (C::BuildingBlocks, S::Hyphae, []),
        (C::BuildingBlocks, S::Hyphae, [M::Stripped]),
        (C::BuildingBlocks, S::Planks, []),
        (C::BuildingBlocks, S::Stairs, []),
        (C::BuildingBlocks, S::Slab, []),
        (C::BuildingBlocks, S::Fence, []),
        (C::BuildingBlocks, S::FenceGate, []),
        (C::BuildingBlocks, S::Door, []),
        (C::BuildingBlocks, S::Trapdoor, []),
        (C::BuildingBlocks, S::PressurePlate, []),
        (C::BuildingBlocks, S::Button, []),

        (C::NaturalBlocks, S::Nylium, []),
        (C::NaturalBlocks, S::Fungus, []),
        (C::NaturalBlocks, S::Roots, []),

        (C::FunctionalBlocks, S::Shelf, []),
        (C::FunctionalBlocks, S::Sign, []),
        (C::FunctionalBlocks, S::HangingSign, []),
    ])]
    Crimson,

    #[material(items = [
        (C::BuildingBlocks, S::Stem, []),
        (C::BuildingBlocks, S::Stem, [M::Stripped]),
        (C::BuildingBlocks, S::Hyphae, []),
        (C::BuildingBlocks, S::Hyphae, [M::Stripped]),
        (C::BuildingBlocks, S::Planks, []),
        (C::BuildingBlocks, S::Stairs, []),
        (C::BuildingBlocks, S::Slab, []),
        (C::BuildingBlocks, S::Fence, []),
        (C::BuildingBlocks, S::FenceGate, []),
        (C::BuildingBlocks, S::Door, []),
        (C::BuildingBlocks, S::Trapdoor, []),
        (C::BuildingBlocks, S::PressurePlate, []),
        (C::BuildingBlocks, S::Button, []),

        (C::NaturalBlocks, S::Nylium, []),
        (C::NaturalBlocks, S::WartBlock, []),
        (C::NaturalBlocks, S::Fungus, []),
        (C::NaturalBlocks, S::Roots, []),

        (C::FunctionalBlocks, S::Shelf, []),
        (C::FunctionalBlocks, S::Sign, []),
        (C::FunctionalBlocks, S::HangingSign, []),

    ])]
    Warped,

    #[material(items = [
        (C::BuildingBlocks, S::Base, []),
        (C::BuildingBlocks, S::Stairs, []),
        (C::BuildingBlocks, S::Slab, []),
        (C::BuildingBlocks, S::PressurePlate, []),
        (C::BuildingBlocks, S::Button, []),

        (C::BuildingBlocks, S::Base, [M::Smooth]),
        (C::BuildingBlocks, S::Slab, [M::Smooth]),

        (C::BuildingBlocks, S::Base, [M::Infested]),
    ])]
    Stone,

    #[material(items = [
        (C::BuildingBlocks, S::Base, []),
        (C::BuildingBlocks, S::Stairs, []),
        (C::BuildingBlocks, S::Slab, []),
        (C::BuildingBlocks, S::Wall, []),

        (C::BuildingBlocks, S::Base, [M::Mossy]),
        (C::BuildingBlocks, S::Stairs, [M::Mossy]),
        (C::BuildingBlocks, S::Slab, [M::Mossy]),
        (C::BuildingBlocks, S::Wall, [M::Mossy]),

        (C::BuildingBlocks, S::Base, [M::Infested]),
    ])]
    Cobblestone,

    #[material(items = [
        (C::BuildingBlocks, S::Base, []),
        (C::BuildingBlocks, S::Stairs, []),
        (C::BuildingBlocks, S::Slab, []),
        (C::BuildingBlocks, S::Wall, []),

        (C::BuildingBlocks, S::Base, [M::Cracked]),
        (C::BuildingBlocks, S::Base, [M::Chiseled]),

        (C::BuildingBlocks, S::Base, [M::Mossy]),
        (C::BuildingBlocks, S::Stairs, [M::Mossy]),
        (C::BuildingBlocks, S::Slab, [M::Mossy]),
        (C::BuildingBlocks, S::Wall, [M::Mossy]),

        (C::BuildingBlocks, S::Base, [M::Infested]),
        (C::BuildingBlocks, S::Base, [M::Infested, M::Mossy]),
        (C::BuildingBlocks, S::Base, [M::Infested, M::Cracked]),
        (C::BuildingBlocks, S::Base, [M::Infested, M::Chiseled]),
    ])]
    StoneBricks,

    #[material(items = [
        (C::BuildingBlocks, S::Base, []),
        (C::BuildingBlocks, S::Stairs, []),
        (C::BuildingBlocks, S::Slab, []),
        (C::BuildingBlocks, S::Wall, []),
        (C::BuildingBlocks, S::Base, [M::Polished]),
        (C::BuildingBlocks, S::Stairs, [M::Polished]),
        (C::BuildingBlocks, S::Slab, [M::Polished]),
    ])]
    Granite,

    #[material(items = [
        (C::BuildingBlocks, S::Base, []),
        (C::BuildingBlocks, S::Stairs, []),
        (C::BuildingBlocks, S::Slab, []),
        (C::BuildingBlocks, S::Wall, []),
        (C::BuildingBlocks, S::Base, [M::Polished]),
        (C::BuildingBlocks, S::Stairs, [M::Polished]),
        (C::BuildingBlocks, S::Slab, [M::Polished]),
    ])]
    Diorite,

    #[material(items = [
        (C::BuildingBlocks, S::Base, []),
        (C::BuildingBlocks, S::Stairs, []),
        (C::BuildingBlocks, S::Slab, []),
        (C::BuildingBlocks, S::Wall, []),
        (C::BuildingBlocks, S::Base, [M::Polished]),
        (C::BuildingBlocks, S::Stairs, [M::Polished]),
        (C::BuildingBlocks, S::Slab, [M::Polished]),
    ])]
    Andesite,

    #[material(items = [
        (C::BuildingBlocks, S::Base, []),

        (C::BuildingBlocks, S::Base, [M::Cobbled]),
        (C::BuildingBlocks, S::Stairs, [M::Cobbled]),
        (C::BuildingBlocks, S::Slab, [M::Cobbled]),
        (C::BuildingBlocks, S::Wall, [M::Cobbled]),

        (C::BuildingBlocks, S::Base, [M::Chiseled]),

        (C::BuildingBlocks, S::Base, [M::Polished]),
        (C::BuildingBlocks, S::Stairs, [M::Polished]),
        (C::BuildingBlocks, S::Slab, [M::Polished]),
        (C::BuildingBlocks, S::Wall, [M::Polished]),

        (C::BuildingBlocks, S::Base, [M::Reinforced]),

        (C::BuildingBlocks, S::Base, [M::Infested]),
    ])]
    Deepslate,

    #[material(items = [
        (C::BuildingBlocks, S::Base, []),
        (C::BuildingBlocks, S::Base, [M::Cracked]),
        (C::BuildingBlocks, S::Stairs, []),
        (C::BuildingBlocks, S::Slab, []),
        (C::BuildingBlocks, S::Wall, []),
    ])]
    DeepslateBricks,

    #[material(items = [
        (C::BuildingBlocks, S::Base, []),
        (C::BuildingBlocks, S::Base, [M::Cracked]),
        (C::BuildingBlocks, S::Stairs, []),
        (C::BuildingBlocks, S::Slab, []),
        (C::BuildingBlocks, S::Wall, []),
    ])]
    DeepslateTiles,

    #[material(items = [
        (C::BuildingBlocks, S::Base, []),
        (C::BuildingBlocks, S::Stairs, []),
        (C::BuildingBlocks, S::Slab, []),
        (C::BuildingBlocks, S::Wall, []),

        (C::BuildingBlocks, S::Base, [M::Chiseled]),

        (C::BuildingBlocks, S::Base, [M::Polished]),
        (C::BuildingBlocks, S::Stairs, [M::Polished]),
        (C::BuildingBlocks, S::Slab, [M::Polished]),
        (C::BuildingBlocks, S::Wall, [M::Polished]),
    ])]
    Tuff,

    #[material(items = [
        (C::BuildingBlocks, S::Base, []),
        (C::BuildingBlocks, S::Stairs, []),
        (C::BuildingBlocks, S::Slab, []),
        (C::BuildingBlocks, S::Wall, []),

        (C::BuildingBlocks, S::Base, [M::Chiseled]),
    ])]
    TuffBricks,

    #[material(items = [
        (C::BuildingBlocks, S::Base, []),
        (C::BuildingBlocks, S::Stairs, []),
        (C::BuildingBlocks, S::Slab, []),
        (C::BuildingBlocks, S::Wall, []),
    ])]
    Bricks,

    #[material(items = [
        (C::BuildingBlocks, S::Base, []),
        (C::BuildingBlocks, S::Base, [M::Packed]),
    ])]
    Mud,

    #[material(items = [
        (C::BuildingBlocks, S::Base, []),
        (C::BuildingBlocks, S::Stairs, []),
        (C::BuildingBlocks, S::Slab, []),
        (C::BuildingBlocks, S::Wall, []),
    ])]
    MudBricks,

    #[material(items = [
        (C::BuildingBlocks, S::Base, []),
        (C::BuildingBlocks, S::Stairs, []),
        (C::BuildingBlocks, S::Slab, []),
        (C::BuildingBlocks, S::Wall, []),

        (C::BuildingBlocks, S::Base, [M::Chiseled]),
    ])]
    ResinBricks,

    #[material(items = [
        (C::BuildingBlocks, S::Base, []),
        (C::BuildingBlocks, S::Stairs, []),
        (C::BuildingBlocks, S::Slab, []),
        (C::BuildingBlocks, S::Wall, []),

        (C::BuildingBlocks, S::Base, [M::Chiseled]),

        (C::BuildingBlocks, S::Base, [M::Smooth]),
        (C::BuildingBlocks, S::Stairs, [M::Smooth]),
        (C::BuildingBlocks, S::Slab, [M::Smooth]),

        (C::BuildingBlocks, S::Base, [M::Cut]),
        (C::BuildingBlocks, S::Slab, [M::Cut]),
    ])]
    Sandstone,

    #[material(items = [
        (C::BuildingBlocks, S::Base, []),
        (C::BuildingBlocks, S::Stairs, []),
        (C::BuildingBlocks, S::Slab, []),
        (C::BuildingBlocks, S::Wall, []),

        (C::BuildingBlocks, S::Base, [M::Chiseled]),

        (C::BuildingBlocks, S::Base, [M::Smooth]),
        (C::BuildingBlocks, S::Stairs, [M::Smooth]),
        (C::BuildingBlocks, S::Slab, [M::Smooth]),

        (C::BuildingBlocks, S::Base, [M::Cut]),
        (C::BuildingBlocks, S::Slab, [M::Cut]),
    ])]
    RedSandstone,

    #[material(items = [
        (C::BuildingBlocks, S::Base, []),
        (C::BuildingBlocks, S::Stairs, []),
        (C::BuildingBlocks, S::Slab, []),
        (C::BuildingBlocks, S::Wall, []),

        (C::BuildingBlocks, S::Base, [M::Chiseled]),

        (C::BuildingBlocks, S::Base, [M::Polished]),
        (C::BuildingBlocks, S::Stairs, [M::Polished]),
        (C::BuildingBlocks, S::Slab, [M::Polished]),
        (C::BuildingBlocks, S::Wall, [M::Polished]),
    ])]
    Cinnabar,

    #[material(items = [
        (C::BuildingBlocks, S::Base, []),
        (C::BuildingBlocks, S::Stairs, []),
        (C::BuildingBlocks, S::Slab, []),
        (C::BuildingBlocks, S::Wall, []),
    ])]
    CinnabarBricks,

    #[material(items = [
        (C::BuildingBlocks, S::Base, []),
        (C::BuildingBlocks, S::Stairs, []),
        (C::BuildingBlocks, S::Slab, []),
        (C::BuildingBlocks, S::Wall, []),

        (C::BuildingBlocks, S::Base, [M::Chiseled]),

        (C::BuildingBlocks, S::Base, [M::Polished]),
        (C::BuildingBlocks, S::Stairs, [M::Polished]),
        (C::BuildingBlocks, S::Slab, [M::Polished]),
        (C::BuildingBlocks, S::Wall, [M::Polished]),


        (C::NaturalBlocks, S::Spike, []),
    ])]
    Sulfur,

    #[material(items = [
        (C::BuildingBlocks, S::Base, []),
        (C::BuildingBlocks, S::Stairs, []),
        (C::BuildingBlocks, S::Slab, []),
        (C::BuildingBlocks, S::Wall, []),
    ])]
    SulfurBricks,

    #[material(items = [
        (C::FunctionalBlocks, S::Base, []),
    ])]
    SeaLantern,

    #[material(items = [
        (C::BuildingBlocks, S::Base, []),
        (C::BuildingBlocks, S::Stairs, []),
        (C::BuildingBlocks, S::Slab, []),
        (C::BuildingBlocks, S::Wall, []),

        (C::BuildingBlocks, S::Base, [M::Dark]),
        (C::BuildingBlocks, S::Stairs, [M::Dark]),
        (C::BuildingBlocks, S::Slab, [M::Dark]),

        (C::Ingredients, S::Crystals, []),
        (C::Ingredients, S::Shard, []),
    ])]
    Prismarine,

    #[material(items = [
        (C::BuildingBlocks, S::Base, []),
        (C::BuildingBlocks, S::Stairs, []),
        (C::BuildingBlocks, S::Slab, []),
    ])]
    PrismarineBricks,

    #[material(items = [
        (C::NaturalBlocks, S::Base, []),
    ])]
    Netherrack,

    #[material(items = [
        (C::BuildingBlocks, S::Base, []),
        (C::BuildingBlocks, S::Base, [M::Cracked]),
        (C::BuildingBlocks, S::Base, [M::Chiseled]),
        (C::BuildingBlocks, S::Stairs, []),
        (C::BuildingBlocks, S::Slab, []),
        (C::BuildingBlocks, S::Wall, []),
        (C::BuildingBlocks, S::Fence, []),
    ])]
    NetherBricks,

    #[material(items = [
        (C::BuildingBlocks, S::Base, []),
        (C::BuildingBlocks, S::Stairs, []),
        (C::BuildingBlocks, S::Slab, []),
        (C::BuildingBlocks, S::Wall, []),
    ])]
    RedNetherBricks,

    #[material(items = [
        (C::BuildingBlocks, S::Base, []),
        (C::BuildingBlocks, S::Base, [M::Smooth]),
        (C::BuildingBlocks, S::Base, [M::Polished]),
    ])]
    Basalt,

    #[material(items = [
        (C::BuildingBlocks, S::Base, []),
        (C::BuildingBlocks, S::Base, [M::Gilded]),
        (C::BuildingBlocks, S::Stairs, []),
        (C::BuildingBlocks, S::Slab, []),
        (C::BuildingBlocks, S::Wall, []),

        (C::BuildingBlocks, S::Base, [M::Chiseled, M::Polished]),
        (C::BuildingBlocks, S::Base, [M::Polished]),
        (C::BuildingBlocks, S::Stairs, [M::Polished]),
        (C::BuildingBlocks, S::Slab, [M::Polished]),
        (C::BuildingBlocks, S::Wall, [M::Polished]),
        (C::BuildingBlocks, S::PressurePlate, [M::Polished]),
        (C::BuildingBlocks, S::Button, [M::Polished]),
    ])]
    Blackstone,

    #[material(items = [
        (C::BuildingBlocks, S::Base, [M::Polished]),
        (C::BuildingBlocks, S::Base, [M::Cracked, M::Polished]),
        (C::BuildingBlocks, S::Stairs, [M::Polished]),
        (C::BuildingBlocks, S::Slab, [M::Polished]),
        (C::BuildingBlocks, S::Wall, [M::Polished]),
    ])]
    BlackstoneBricks,

    #[material(items = [
        (C::BuildingBlocks, S::Base, []),
    ])]
    EndStone,

    #[material(items = [
        (C::BuildingBlocks, S::Base, []),
        (C::BuildingBlocks, S::Stairs, []),
        (C::BuildingBlocks, S::Slab, []),
        (C::BuildingBlocks, S::Wall, []),
    ])]
    EndStoneBricks,

    #[material(items = [
        (C::BuildingBlocks, S::Block, []),
        (C::BuildingBlocks, S::Pillar, []),
        (C::BuildingBlocks, S::Stairs, []),
        (C::BuildingBlocks, S::Slab, []),
    ])]
    Purpur,

    // => Colored Blocks
    #[material(items = [
        (C::ColoredBlocks, S::Base, [M::White]),
        (C::ColoredBlocks, S::Base, [M::LightGray]),
        (C::ColoredBlocks, S::Base, [M::Gray]),
        (C::ColoredBlocks, S::Base, [M::Black]),
        (C::ColoredBlocks, S::Base, [M::Brown]),
        (C::ColoredBlocks, S::Base, [M::Red]),
        (C::ColoredBlocks, S::Base, [M::Orange]),
        (C::ColoredBlocks, S::Base, [M::Yellow]),
        (C::ColoredBlocks, S::Base, [M::Lime]),
        (C::ColoredBlocks, S::Base, [M::Green]),
        (C::ColoredBlocks, S::Base, [M::Cyan]),
        (C::ColoredBlocks, S::Base, [M::LightBlue]),
        (C::ColoredBlocks, S::Base, [M::Blue]),
        (C::ColoredBlocks, S::Base, [M::Purple]),
        (C::ColoredBlocks, S::Base, [M::Magenta]),
        (C::ColoredBlocks, S::Base, [M::Pink]),
    ])]
    Wool,

    #[material(items = [
        (C::ColoredBlocks, S::Base, [M::White]),
        (C::ColoredBlocks, S::Base, [M::LightGray]),
        (C::ColoredBlocks, S::Base, [M::Gray]),
        (C::ColoredBlocks, S::Base, [M::Black]),
        (C::ColoredBlocks, S::Base, [M::Brown]),
        (C::ColoredBlocks, S::Base, [M::Red]),
        (C::ColoredBlocks, S::Base, [M::Orange]),
        (C::ColoredBlocks, S::Base, [M::Yellow]),
        (C::ColoredBlocks, S::Base, [M::Lime]),
        (C::ColoredBlocks, S::Base, [M::Green]),
        (C::ColoredBlocks, S::Base, [M::Cyan]),
        (C::ColoredBlocks, S::Base, [M::LightBlue]),
        (C::ColoredBlocks, S::Base, [M::Blue]),
        (C::ColoredBlocks, S::Base, [M::Purple]),
        (C::ColoredBlocks, S::Base, [M::Magenta]),
        (C::ColoredBlocks, S::Base, [M::Pink]),
    ])]
    Carpet,

    #[material(items = [
        (C::ColoredBlocks, S::Base, []),
        (C::ColoredBlocks, S::Base, [M::White]),
        (C::ColoredBlocks, S::Base, [M::LightGray]),
        (C::ColoredBlocks, S::Base, [M::Gray]),
        (C::ColoredBlocks, S::Base, [M::Black]),
        (C::ColoredBlocks, S::Base, [M::Brown]),
        (C::ColoredBlocks, S::Base, [M::Red]),
        (C::ColoredBlocks, S::Base, [M::Orange]),
        (C::ColoredBlocks, S::Base, [M::Yellow]),
        (C::ColoredBlocks, S::Base, [M::Lime]),
        (C::ColoredBlocks, S::Base, [M::Green]),
        (C::ColoredBlocks, S::Base, [M::Cyan]),
        (C::ColoredBlocks, S::Base, [M::LightBlue]),
        (C::ColoredBlocks, S::Base, [M::Blue]),
        (C::ColoredBlocks, S::Base, [M::Purple]),
        (C::ColoredBlocks, S::Base, [M::Magenta]),
        (C::ColoredBlocks, S::Base, [M::Pink]),
    ])]
    Terracotta,

    #[material(items = [
        (C::ColoredBlocks, S::Base, [M::White]),
        (C::ColoredBlocks, S::Base, [M::LightGray]),
        (C::ColoredBlocks, S::Base, [M::Gray]),
        (C::ColoredBlocks, S::Base, [M::Black]),
        (C::ColoredBlocks, S::Base, [M::Brown]),
        (C::ColoredBlocks, S::Base, [M::Red]),
        (C::ColoredBlocks, S::Base, [M::Orange]),
        (C::ColoredBlocks, S::Base, [M::Yellow]),
        (C::ColoredBlocks, S::Base, [M::Lime]),
        (C::ColoredBlocks, S::Base, [M::Green]),
        (C::ColoredBlocks, S::Base, [M::Cyan]),
        (C::ColoredBlocks, S::Base, [M::LightBlue]),
        (C::ColoredBlocks, S::Base, [M::Blue]),
        (C::ColoredBlocks, S::Base, [M::Purple]),
        (C::ColoredBlocks, S::Base, [M::Magenta]),
        (C::ColoredBlocks, S::Base, [M::Pink]),
    ])]
    Concrete,

    #[material(items = [
        (C::ColoredBlocks, S::Base, [M::White]),
        (C::ColoredBlocks, S::Base, [M::LightGray]),
        (C::ColoredBlocks, S::Base, [M::Gray]),
        (C::ColoredBlocks, S::Base, [M::Black]),
        (C::ColoredBlocks, S::Base, [M::Brown]),
        (C::ColoredBlocks, S::Base, [M::Red]),
        (C::ColoredBlocks, S::Base, [M::Orange]),
        (C::ColoredBlocks, S::Base, [M::Yellow]),
        (C::ColoredBlocks, S::Base, [M::Lime]),
        (C::ColoredBlocks, S::Base, [M::Green]),
        (C::ColoredBlocks, S::Base, [M::Cyan]),
        (C::ColoredBlocks, S::Base, [M::LightBlue]),
        (C::ColoredBlocks, S::Base, [M::Blue]),
        (C::ColoredBlocks, S::Base, [M::Purple]),
        (C::ColoredBlocks, S::Base, [M::Magenta]),
        (C::ColoredBlocks, S::Base, [M::Pink]),
    ])]
    ConcretePowder,

    #[material(items = [
        (C::ColoredBlocks, S::Base, [M::White]),
        (C::ColoredBlocks, S::Base, [M::LightGray]),
        (C::ColoredBlocks, S::Base, [M::Gray]),
        (C::ColoredBlocks, S::Base, [M::Black]),
        (C::ColoredBlocks, S::Base, [M::Brown]),
        (C::ColoredBlocks, S::Base, [M::Red]),
        (C::ColoredBlocks, S::Base, [M::Orange]),
        (C::ColoredBlocks, S::Base, [M::Yellow]),
        (C::ColoredBlocks, S::Base, [M::Lime]),
        (C::ColoredBlocks, S::Base, [M::Green]),
        (C::ColoredBlocks, S::Base, [M::Cyan]),
        (C::ColoredBlocks, S::Base, [M::LightBlue]),
        (C::ColoredBlocks, S::Base, [M::Blue]),
        (C::ColoredBlocks, S::Base, [M::Purple]),
        (C::ColoredBlocks, S::Base, [M::Magenta]),
        (C::ColoredBlocks, S::Base, [M::Pink]),
    ])]
    GlazedTerracotta,

    #[material(items = [
        (C::ColoredBlocks, S::Base, []),
        (C::ColoredBlocks, S::Base, [M::Tinted]),
        (C::ColoredBlocks, S::Base, [M::White, M::Stained]),
        (C::ColoredBlocks, S::Base, [M::LightGray, M::Stained]),
        (C::ColoredBlocks, S::Base, [M::Gray, M::Stained]),
        (C::ColoredBlocks, S::Base, [M::Black, M::Stained]),
        (C::ColoredBlocks, S::Base, [M::Brown, M::Stained]),
        (C::ColoredBlocks, S::Base, [M::Red, M::Stained]),
        (C::ColoredBlocks, S::Base, [M::Orange, M::Stained]),
        (C::ColoredBlocks, S::Base, [M::Yellow, M::Stained]),
        (C::ColoredBlocks, S::Base, [M::Lime, M::Stained]),
        (C::ColoredBlocks, S::Base, [M::Green, M::Stained]),
        (C::ColoredBlocks, S::Base, [M::Cyan, M::Stained]),
        (C::ColoredBlocks, S::Base, [M::LightBlue, M::Stained]),
        (C::ColoredBlocks, S::Base, [M::Blue, M::Stained]),
        (C::ColoredBlocks, S::Base, [M::Purple, M::Stained]),
        (C::ColoredBlocks, S::Base, [M::Magenta, M::Stained]),
        (C::ColoredBlocks, S::Base, [M::Pink, M::Stained]),
    ])]
    Glass,

    #[material(items = [
        (C::ColoredBlocks, S::Base, []),
        (C::ColoredBlocks, S::Base, [M::White, M::Stained]),
        (C::ColoredBlocks, S::Base, [M::LightGray, M::Stained]),
        (C::ColoredBlocks, S::Base, [M::Gray, M::Stained]),
        (C::ColoredBlocks, S::Base, [M::Black, M::Stained]),
        (C::ColoredBlocks, S::Base, [M::Brown, M::Stained]),
        (C::ColoredBlocks, S::Base, [M::Red, M::Stained]),
        (C::ColoredBlocks, S::Base, [M::Orange, M::Stained]),
        (C::ColoredBlocks, S::Base, [M::Yellow, M::Stained]),
        (C::ColoredBlocks, S::Base, [M::Lime, M::Stained]),
        (C::ColoredBlocks, S::Base, [M::Green, M::Stained]),
        (C::ColoredBlocks, S::Base, [M::Cyan, M::Stained]),
        (C::ColoredBlocks, S::Base, [M::LightBlue, M::Stained]),
        (C::ColoredBlocks, S::Base, [M::Blue, M::Stained]),
        (C::ColoredBlocks, S::Base, [M::Purple, M::Stained]),
        (C::ColoredBlocks, S::Base, [M::Magenta, M::Stained]),
        (C::ColoredBlocks, S::Base, [M::Pink, M::Stained]),
    ])]
    GlassPane,

    #[material(items = [
        (C::Ingredients, S::Shell, []),
        (C::ColoredBlocks, S::Box, []),
        (C::ColoredBlocks, S::Box, [M::White]),
        (C::ColoredBlocks, S::Box, [M::LightGray]),
        (C::ColoredBlocks, S::Box, [M::Gray]),
        (C::ColoredBlocks, S::Box, [M::Black]),
        (C::ColoredBlocks, S::Box, [M::Brown]),
        (C::ColoredBlocks, S::Box, [M::Red]),
        (C::ColoredBlocks, S::Box, [M::Orange]),
        (C::ColoredBlocks, S::Box, [M::Yellow]),
        (C::ColoredBlocks, S::Box, [M::Lime]),
        (C::ColoredBlocks, S::Box, [M::Green]),
        (C::ColoredBlocks, S::Box, [M::Cyan]),
        (C::ColoredBlocks, S::Box, [M::LightBlue]),
        (C::ColoredBlocks, S::Box, [M::Blue]),
        (C::ColoredBlocks, S::Box, [M::Purple]),
        (C::ColoredBlocks, S::Box, [M::Magenta]),
        (C::ColoredBlocks, S::Box, [M::Pink]),
    ])]
    Shulker,

    #[material(items = [
        (C::ColoredBlocks, S::Base, [M::White]),
        (C::ColoredBlocks, S::Base, [M::LightGray]),
        (C::ColoredBlocks, S::Base, [M::Gray]),
        (C::ColoredBlocks, S::Base, [M::Black]),
        (C::ColoredBlocks, S::Base, [M::Brown]),
        (C::ColoredBlocks, S::Base, [M::Red]),
        (C::ColoredBlocks, S::Base, [M::Orange]),
        (C::ColoredBlocks, S::Base, [M::Yellow]),
        (C::ColoredBlocks, S::Base, [M::Lime]),
        (C::ColoredBlocks, S::Base, [M::Green]),
        (C::ColoredBlocks, S::Base, [M::Cyan]),
        (C::ColoredBlocks, S::Base, [M::LightBlue]),
        (C::ColoredBlocks, S::Base, [M::Blue]),
        (C::ColoredBlocks, S::Base, [M::Purple]),
        (C::ColoredBlocks, S::Base, [M::Magenta]),
        (C::ColoredBlocks, S::Base, [M::Pink]),
    ])]
    Bed,

    #[material(items = [
        (C::ColoredBlocks, S::Base, []),
        (C::ColoredBlocks, S::Base, [M::White]),
        (C::ColoredBlocks, S::Base, [M::LightGray]),
        (C::ColoredBlocks, S::Base, [M::Gray]),
        (C::ColoredBlocks, S::Base, [M::Black]),
        (C::ColoredBlocks, S::Base, [M::Brown]),
        (C::ColoredBlocks, S::Base, [M::Red]),
        (C::ColoredBlocks, S::Base, [M::Orange]),
        (C::ColoredBlocks, S::Base, [M::Yellow]),
        (C::ColoredBlocks, S::Base, [M::Lime]),
        (C::ColoredBlocks, S::Base, [M::Green]),
        (C::ColoredBlocks, S::Base, [M::Cyan]),
        (C::ColoredBlocks, S::Base, [M::LightBlue]),
        (C::ColoredBlocks, S::Base, [M::Blue]),
        (C::ColoredBlocks, S::Base, [M::Purple]),
        (C::ColoredBlocks, S::Base, [M::Magenta]),
        (C::ColoredBlocks, S::Base, [M::Pink]),
    ])]
    Candle,

    #[material(items = [
        (C::ColoredBlocks, S::Base, [M::White]),
        (C::ColoredBlocks, S::Base, [M::LightGray]),
        (C::ColoredBlocks, S::Base, [M::Gray]),
        (C::ColoredBlocks, S::Base, [M::Black]),
        (C::ColoredBlocks, S::Base, [M::Brown]),
        (C::ColoredBlocks, S::Base, [M::Red]),
        (C::ColoredBlocks, S::Base, [M::Orange]),
        (C::ColoredBlocks, S::Base, [M::Yellow]),
        (C::ColoredBlocks, S::Base, [M::Lime]),
        (C::ColoredBlocks, S::Base, [M::Green]),
        (C::ColoredBlocks, S::Base, [M::Cyan]),
        (C::ColoredBlocks, S::Base, [M::LightBlue]),
        (C::ColoredBlocks, S::Base, [M::Blue]),
        (C::ColoredBlocks, S::Base, [M::Purple]),
        (C::ColoredBlocks, S::Base, [M::Magenta]),
        (C::ColoredBlocks, S::Base, [M::Pink]),
    ])]
    Banner,

    // => Natural Blocks
    #[material(items = [
        (C::NaturalBlocks, S::Block, []),
        (C::NaturalBlocks, S::Base, [M::Short]),
        (C::NaturalBlocks, S::Base, [M::Short, M::Dry]),
        (C::NaturalBlocks, S::Base, [M::Tall]),
        (C::NaturalBlocks, S::Base, [M::Tall, M::Dry]),
    ])]
    Grass,

    #[material(items = [
        (C::NaturalBlocks, S::Base, []),
    ])]
    Podzol,

    #[material(items = [
        (C::NaturalBlocks, S::Base, []),
    ])]
    Mycelium,

    #[material(items = [
        (C::NaturalBlocks, S::Base, []),
        (C::NaturalBlocks, S::Path, []),
        (C::NaturalBlocks, S::Base, [M::Coarse]),
        (C::NaturalBlocks, S::Base, [M::Rooted]),
    ])]
    Dirt,

    #[material(items = [
        (C::NaturalBlocks, S::Base, []),
    ])]
    Farmland,

    #[material(items = [
        (C::NaturalBlocks, S::Base, []),
        (C::NaturalBlocks, S::Block, []),
    ])]
    Snow,

    #[material(items = [
        (C::NaturalBlocks, S::Block, []),
        (C::NaturalBlocks, S::Carpet, []),
    ])]
    Moss,

    #[material(items = [
        (C::NaturalBlocks, S::Block, []),
        (C::NaturalBlocks, S::Carpet, []),
    ])]
    PaleMoss,

    #[material(items = [
        (C::NaturalBlocks, S::Base, []),
    ])]
    PaleHangingMoss,

    #[material(items = [
        (C::NaturalBlocks, S::Base, []),
    ])]
    Calcite,

    #[material(items = [
        (C::NaturalBlocks, S::Block, []),
        (C::NaturalBlocks, S::Base, [M::Pointed]),
    ])]
    Dripstone,

    #[material(items = [
        (C::NaturalBlocks, S::Sand, []),
        (C::NaturalBlocks, S::Soil, []),
    ])]
    Soul,

    #[material(items = [
        (C::NaturalBlocks, S::Base, []),
        (C::NaturalBlocks, S::Base, [M::Packed]),
        (C::NaturalBlocks, S::Base, [M::Blue]),
    ])]
    Ice,

    #[material(items = [
        (C::NaturalBlocks, S::Base, []),
        (C::FoodAndDrinks, S::Base, [M::Golden]),
    ])]
    Dandelion,

    #[material(items = [
        (C::NaturalBlocks, S::Base, []),
    ])]
    Poppy,

    #[material(items = [
        (C::NaturalBlocks, S::Base, []),
    ])]
    BlueOrchid,

    #[material(items = [
        (C::NaturalBlocks, S::Base, []),
    ])]
    Allium,

    #[material(items = [
        (C::NaturalBlocks, S::Base, []),
    ])]
    AzureBluet,

    #[material(items = [
        (C::NaturalBlocks, S::Base, []),
    ])]
    RedTulip,

    #[material(items = [
        (C::NaturalBlocks, S::Base, []),
    ])]
    OrangeTulip,

    #[material(items = [
        (C::NaturalBlocks, S::Base, []),
    ])]
    WhiteTulip,

    #[material(items = [
        (C::NaturalBlocks, S::Base, []),
    ])]
    PinkTulip,

    #[material(items = [
        (C::NaturalBlocks, S::Base, []),
    ])]
    OxeyeDaisy,

    #[material(items = [
        (C::NaturalBlocks, S::Base, []),
    ])]
    Cornflower,

    #[material(items = [
        (C::NaturalBlocks, S::Base, []),
    ])]
    LilyOfTheValley,

    #[material(items = [
        (C::NaturalBlocks, S::Base, []),
        (C::NaturalBlocks, S::Seeds, []),
    ])]
    Torchflower,

    #[material(items = [
        (C::NaturalBlocks, S::Base, []),
        (C::NaturalBlocks, S::Flower, []),
    ])]
    Cactus,

    #[material(items = [
        (C::NaturalBlocks, S::Base, [M::Closed]),
        (C::NaturalBlocks, S::Base, [M::Open]),
    ])]
    Eyeblossom,

    #[material(items = [
        (C::NaturalBlocks, S::Base, []),
    ])]
    WitherRose,

    #[material(items = [
        (C::NaturalBlocks, S::Base, []),
    ])]
    PinkPetals,

    #[material(items = [
        (C::NaturalBlocks, S::Base, []),
    ])]
    Wildflowers,

    #[material(items = [
        (C::NaturalBlocks, S::Base, []),
    ])]
    LeafLitter,

    #[material(items = [
        (C::NaturalBlocks, S::Base, []),
    ])]
    SporeBlossom,

    #[material(items = [
        (C::NaturalBlocks, S::Base, []),
    ])]
    FireflyBush,

    #[material(items = [
        (C::NaturalBlocks, S::Base, []),
    ])]
    SugarCane,

    #[material(items = [
        (C::NaturalBlocks, S::Base, []),
    ])]
    CrimsonRoots,

    #[material(items = [
        (C::NaturalBlocks, S::Base, []),
    ])]
    WarpedRoots,

    #[material(items = [
        (C::NaturalBlocks, S::Base, []),
    ])]
    NetherSprouts,

    #[material(items = [
        (C::NaturalBlocks, S::Base, [M::Weeping]),
        (C::NaturalBlocks, S::Base, [M::Twisting]),
    ])]
    Vines,

    #[material(items = [
        (C::NaturalBlocks, S::Base, []),
    ])]
    Vine,

    #[material(items = [
        (C::NaturalBlocks, S::Base, []),
        (C::NaturalBlocks, S::Base, [M::Dead]),
    ])]
    Bush,

    #[material(items = [
        (C::NaturalBlocks, S::Base, []),
        (C::NaturalBlocks, S::Base, [M::Large]),
    ])]
    Fern,

    #[material(items = [
        (C::NaturalBlocks, S::Base, []),
    ])]
    Sunflower,

    #[material(items = [
        (C::NaturalBlocks, S::Base, []),
    ])]
    Lilac,

    #[material(items = [
        (C::NaturalBlocks, S::Base, []),
    ])]
    RoseBush,

    #[material(items = [
        (C::NaturalBlocks, S::Base, []),
    ])]
    Peony,

    #[material(items = [
        (C::NaturalBlocks, S::Plant, []),
        (C::NaturalBlocks, S::Pod, []),
    ])]
    Pitcher,

    #[material(items = [
        (C::NaturalBlocks, S::Plant, []),
        (C::NaturalBlocks, S::Flower, []),
        (C::NaturalBlocks, S::Fruit, []),
        (C::NaturalBlocks, S::Fruit, [M::Popped]),
    ])]
    Chorus,

    #[material(items = [
        (C::NaturalBlocks, S::Base, []),
    ])]
    GlowLichen,

    #[material(items = [
        (C::NaturalBlocks, S::Base, []),
    ])]
    HangingRoots,

    #[material(items = [
        (C::NaturalBlocks, S::Base, []),
    ])]
    Frogspawn,

    #[material(items = [
        (C::NaturalBlocks, S::Base, []),
    ])]
    DriedGhast,

    #[material(items = [
        (C::NaturalBlocks, S::Base, []),
    ])]
    CocoaBeans,

    #[material(items = [
        (C::NaturalBlocks, S::Base, []),
        (C::NaturalBlocks, S::Block, []),
    ])]
    NetherWart,

    #[material(items = [
        (C::NaturalBlocks, S::Base, []),
    ])]
    LilyPad,

    #[material(items = [
        (C::NaturalBlocks, S::Base, []),
    ])]
    Seagrass,

    #[material(items = [
        (C::NaturalBlocks, S::Base, []),
    ])]
    SeaPickle,

    #[material(items = [
        (C::NaturalBlocks, S::Block, [M::Brain]),
        (C::NaturalBlocks, S::Block, [M::Bubble]),
        (C::NaturalBlocks, S::Block, [M::Fire]),
        (C::NaturalBlocks, S::Block, [M::Horn]),
        (C::NaturalBlocks, S::Block, [M::Tube]),
        (C::NaturalBlocks, S::Block, [M::Dead, M::Brain]),
        (C::NaturalBlocks, S::Block, [M::Dead, M::Bubble]),
        (C::NaturalBlocks, S::Block, [M::Dead, M::Fire]),
        (C::NaturalBlocks, S::Block, [M::Dead, M::Horn]),
        (C::NaturalBlocks, S::Block, [M::Dead, M::Tube]),

        (C::NaturalBlocks, S::Base, [M::Brain]),
        (C::NaturalBlocks, S::Base, [M::Bubble]),
        (C::NaturalBlocks, S::Base, [M::Fire]),
        (C::NaturalBlocks, S::Base, [M::Horn]),
        (C::NaturalBlocks, S::Base, [M::Tube]),
        (C::NaturalBlocks, S::Base, [M::Dead, M::Brain]),
        (C::NaturalBlocks, S::Base, [M::Dead, M::Bubble]),
        (C::NaturalBlocks, S::Base, [M::Dead, M::Fire]),
        (C::NaturalBlocks, S::Base, [M::Dead, M::Horn]),
        (C::NaturalBlocks, S::Base, [M::Dead, M::Tube]),

        (C::NaturalBlocks, S::Fan, [M::Brain]),
        (C::NaturalBlocks, S::Fan, [M::Bubble]),
        (C::NaturalBlocks, S::Fan, [M::Fire]),
        (C::NaturalBlocks, S::Fan, [M::Horn]),
        (C::NaturalBlocks, S::Fan, [M::Tube]),
        (C::NaturalBlocks, S::Fan, [M::Dead, M::Brain]),
        (C::NaturalBlocks, S::Fan, [M::Dead, M::Bubble]),
        (C::NaturalBlocks, S::Fan, [M::Dead, M::Fire]),
        (C::NaturalBlocks, S::Fan, [M::Dead, M::Horn]),
        (C::NaturalBlocks, S::Fan, [M::Dead, M::Tube]),
    ])]
    Coral,

    #[material(items = [
        (C::NaturalBlocks, S::Base, [M::Small]),
        (C::NaturalBlocks, S::Base, [M::Big]),
    ])]
    Dripleaf,

    #[material(items = [
        (C::NaturalBlocks, S::Block, []),
        (C::NaturalBlocks, S::Base, [M::Budding]),
        (C::NaturalBlocks, S::Bud, [M::Small]),
        (C::NaturalBlocks, S::Bud, [M::Medium]),
        (C::NaturalBlocks, S::Bud, [M::Large]),
        (C::NaturalBlocks, S::Cluster, []),
        (C::Ingredients, S::Shard, []),
    ])]
    Amethyst,

    #[material(items = [
        (C::NaturalBlocks, S::Base, []),
        (C::NaturalBlocks, S::Base, [M::Wet]),
    ])]
    Sponge,

    #[material(items = [
        (C::NaturalBlocks, S::Base, []),
        (C::NaturalBlocks, S::Base, [M::Carved]),
        (C::FoodAndDrinks, S::Pie, []),
        (C::FoodAndDrinks, S::Seeds, []),
    ])]
    Pumpkin,

    #[material(items = [
        (C::NaturalBlocks, S::Base, []),
    ])]
    JackOLantern,

    #[material(items = [
        (C::NaturalBlocks, S::Block, []),
    ])]
    Hay,

    #[material(items = [
        (C::Ingredients, S::Base, []),  // TODO: double check Base.
        (C::Ingredients, S::Seeds, []),
    ])]
    Wheat,

    #[material(items = [
        (C::NaturalBlocks, S::Base, []),
    ])]
    BeeNest,

    #[material(items = [
        (C::FunctionalBlocks, S::Base, []),
    ])]
    Beehive,

    #[material(items = [
        (C::Ingredients, S::Base, []),
        (C::NaturalBlocks, S::Block, []),
    ])]
    Honeycomb,

    #[material(items = [
        (C::Ingredients, S::Ball, []),
        (C::NaturalBlocks, S::Block, []),
    ])]
    Slime,

    #[material(items = [
        (C::NaturalBlocks, S::Block, []),
    ])]
    Honey,

    #[material(items = [
        (C::NaturalBlocks, S::Block, []),
        (C::Ingredients, S::Clump, []),
        (C::NaturalBlocks, S::Brick, []),
    ])]
    Resin,

    #[material(items = [
        (C::Ingredients, S::Brick, []),
    ])]
    Nether,

    #[material(items = [
        (C::NaturalBlocks, S::Base, []),
        (C::NaturalBlocks, S::Vein, []),
        (C::NaturalBlocks, S::Catalyst, []),
        (C::NaturalBlocks, S::Shrieker, []),
        (C::NaturalBlocks, S::Sensor, []),
        (C::NaturalBlocks, S::Sensor, [M::Calibrated]),
    ])]
    Sculk,

    #[material(items = [
        (C::NaturalBlocks, S::Base, []),
    ])]
    Cobweb,

    #[material(items = [
        (C::NaturalBlocks, S::Base, []),
    ])]
    Bedrock,

    // => Functional Blocks
    #[material(items = [
        (C::FunctionalBlocks, S::Base, []),
        (C::FunctionalBlocks, S::Base, [M::Soul]),
        (C::FunctionalBlocks, S::Base, [M::Copper]),
        (C::FunctionalBlocks, S::Base, [M::Redstone]),
    ])]
    Torch,

    #[material(items = [
        (C::FunctionalBlocks, S::Base, []),
        (C::FunctionalBlocks, S::Base, [M::Soul]),
        (C::FunctionalBlocks, S::Base, [M::Copper]),
        (C::FunctionalBlocks, S::Base, [M::Exposed, M::Copper]),
        (C::FunctionalBlocks, S::Base, [M::Weathered, M::Copper]),
        (C::FunctionalBlocks, S::Base, [M::Oxidized, M::Copper]),
        (C::FunctionalBlocks, S::Base, [M::Waxed, M::Copper]),
        (C::FunctionalBlocks, S::Base, [M::Waxed, M::Exposed, M::Copper]),
        (C::FunctionalBlocks, S::Base, [M::Waxed, M::Weathered, M::Copper]),
        (C::FunctionalBlocks, S::Base, [M::Waxed, M::Oxidized, M::Copper]),
    ])]
    Lantern,

    #[material(items = [
        (C::FunctionalBlocks, S::Base, [M::Iron]),
        (C::FunctionalBlocks, S::Base, [M::Copper]),
        (C::FunctionalBlocks, S::Base, [M::Exposed, M::Copper]),
        (C::FunctionalBlocks, S::Base, [M::Weathered, M::Copper]),
        (C::FunctionalBlocks, S::Base, [M::Oxidized, M::Copper]),
        (C::FunctionalBlocks, S::Base, [M::Waxed, M::Copper]),
        (C::FunctionalBlocks, S::Base, [M::Waxed, M::Exposed, M::Copper]),
        (C::FunctionalBlocks, S::Base, [M::Waxed, M::Weathered, M::Copper]),
        (C::FunctionalBlocks, S::Base, [M::Waxed, M::Oxidized, M::Copper]),
    ])]
    Chain,

    #[material(items = [
        (C::FunctionalBlocks, S::Base, []),
    ])]
    EndRod,

    // *cries*
    #[material(items = [
        (C::Ingredients, S::Base, [M::Raw]),
        (C::FunctionalBlocks, S::Block, []),
        (C::FunctionalBlocks, S::Block, [M::Raw]),
        (C::FunctionalBlocks, S::Block, [M::Waxed]),

        (C::FunctionalBlocks, S::Base, [M::Exposed]),
        (C::FunctionalBlocks, S::Base, [M::Weathered]),
        (C::FunctionalBlocks, S::Base, [M::Oxidized]),
        (C::FunctionalBlocks, S::Base, [M::Waxed, M::Exposed]),
        (C::FunctionalBlocks, S::Base, [M::Waxed, M::Weathered]),
        (C::FunctionalBlocks, S::Base, [M::Waxed, M::Oxidized]),

        (C::FunctionalBlocks, S::Base, [M::Chiseled]),
        (C::FunctionalBlocks, S::Base, [M::Exposed, M::Chiseled]),
        (C::FunctionalBlocks, S::Base, [M::Weathered, M::Chiseled]),
        (C::FunctionalBlocks, S::Base, [M::Oxidized, M::Chiseled]),
        (C::FunctionalBlocks, S::Base, [M::Waxed, M::Chiseled]),
        (C::FunctionalBlocks, S::Base, [M::Waxed, M::Exposed, M::Chiseled]),
        (C::FunctionalBlocks, S::Base, [M::Waxed, M::Weathered, M::Chiseled]),
        (C::FunctionalBlocks, S::Base, [M::Waxed, M::Oxidized, M::Chiseled]),

        (C::FunctionalBlocks, S::Grate, []),
        (C::FunctionalBlocks, S::Grate, [M::Exposed]),
        (C::FunctionalBlocks, S::Grate, [M::Weathered]),
        (C::FunctionalBlocks, S::Grate, [M::Oxidized]),
        (C::FunctionalBlocks, S::Grate, [M::Waxed]),
        (C::FunctionalBlocks, S::Grate, [M::Waxed, M::Exposed]),
        (C::FunctionalBlocks, S::Grate, [M::Waxed, M::Weathered]),
        (C::FunctionalBlocks, S::Grate, [M::Waxed, M::Oxidized]),

        (C::FunctionalBlocks, S::Base, [M::Cut]),
        (C::FunctionalBlocks, S::Base, [M::Exposed, M::Cut]),
        (C::FunctionalBlocks, S::Base, [M::Weathered, M::Cut]),
        (C::FunctionalBlocks, S::Base, [M::Oxidized, M::Cut]),
        (C::FunctionalBlocks, S::Base, [M::Waxed, M::Cut]),
        (C::FunctionalBlocks, S::Base, [M::Waxed, M::Exposed, M::Cut]),
        (C::FunctionalBlocks, S::Base, [M::Waxed, M::Weathered, M::Cut]),
        (C::FunctionalBlocks, S::Base, [M::Waxed, M::Oxidized, M::Cut]),

        (C::FunctionalBlocks, S::Stairs, [M::Cut]),
        (C::FunctionalBlocks, S::Stairs, [M::Exposed, M::Cut]),
        (C::FunctionalBlocks, S::Stairs, [M::Weathered, M::Cut]),
        (C::FunctionalBlocks, S::Stairs, [M::Oxidized, M::Cut]),
        (C::FunctionalBlocks, S::Stairs, [M::Waxed, M::Cut]),
        (C::FunctionalBlocks, S::Stairs, [M::Waxed, M::Exposed, M::Cut]),
        (C::FunctionalBlocks, S::Stairs, [M::Waxed, M::Weathered, M::Cut]),
        (C::FunctionalBlocks, S::Stairs, [M::Waxed, M::Oxidized, M::Cut]),

        (C::FunctionalBlocks, S::Slab, [M::Cut]),
        (C::FunctionalBlocks, S::Slab, [M::Exposed, M::Cut]),
        (C::FunctionalBlocks, S::Slab, [M::Weathered, M::Cut]),
        (C::FunctionalBlocks, S::Slab, [M::Oxidized, M::Cut]),
        (C::FunctionalBlocks, S::Slab, [M::Waxed, M::Cut]),
        (C::FunctionalBlocks, S::Slab, [M::Waxed, M::Exposed, M::Cut]),
        (C::FunctionalBlocks, S::Slab, [M::Waxed, M::Weathered, M::Cut]),
        (C::FunctionalBlocks, S::Slab, [M::Waxed, M::Oxidized, M::Cut]),

        (C::FunctionalBlocks, S::Bars, []),
        (C::FunctionalBlocks, S::Bars, [M::Exposed]),
        (C::FunctionalBlocks, S::Bars, [M::Weathered]),
        (C::FunctionalBlocks, S::Bars, [M::Oxidized]),
        (C::FunctionalBlocks, S::Bars, [M::Waxed]),
        (C::FunctionalBlocks, S::Bars, [M::Waxed, M::Exposed]),
        (C::FunctionalBlocks, S::Bars, [M::Waxed, M::Weathered]),
        (C::FunctionalBlocks, S::Bars, [M::Waxed, M::Oxidized]),

        (C::FunctionalBlocks, S::Door, []),
        (C::FunctionalBlocks, S::Door, [M::Exposed]),
        (C::FunctionalBlocks, S::Door, [M::Weathered]),
        (C::FunctionalBlocks, S::Door, [M::Oxidized]),
        (C::FunctionalBlocks, S::Door, [M::Waxed]),
        (C::FunctionalBlocks, S::Door, [M::Waxed, M::Exposed]),
        (C::FunctionalBlocks, S::Door, [M::Waxed, M::Weathered]),
        (C::FunctionalBlocks, S::Door, [M::Waxed, M::Oxidized]),

        (C::FunctionalBlocks, S::Trapdoor, []),
        (C::FunctionalBlocks, S::Trapdoor, [M::Exposed]),
        (C::FunctionalBlocks, S::Trapdoor, [M::Weathered]),
        (C::FunctionalBlocks, S::Trapdoor, [M::Oxidized]),
        (C::FunctionalBlocks, S::Trapdoor, [M::Waxed]),
        (C::FunctionalBlocks, S::Trapdoor, [M::Waxed, M::Exposed]),
        (C::FunctionalBlocks, S::Trapdoor, [M::Waxed, M::Weathered]),
        (C::FunctionalBlocks, S::Trapdoor, [M::Waxed, M::Oxidized]),

        (C::FunctionalBlocks, S::Bulb, []),
        (C::FunctionalBlocks, S::Bulb, [M::Exposed]),
        (C::FunctionalBlocks, S::Bulb, [M::Weathered]),
        (C::FunctionalBlocks, S::Bulb, [M::Oxidized]),
        (C::FunctionalBlocks, S::Bulb, [M::Waxed]),
        (C::FunctionalBlocks, S::Bulb, [M::Waxed, M::Exposed]),
        (C::FunctionalBlocks, S::Bulb, [M::Waxed, M::Weathered]),
        (C::FunctionalBlocks, S::Bulb, [M::Waxed, M::Oxidized]),

        (C::FunctionalBlocks, S::GolemStatue, []),
        (C::FunctionalBlocks, S::GolemStatue, [M::Exposed]),
        (C::FunctionalBlocks, S::GolemStatue, [M::Weathered]),
        (C::FunctionalBlocks, S::GolemStatue, [M::Oxidized]),
        (C::FunctionalBlocks, S::GolemStatue, [M::Waxed]),
        (C::FunctionalBlocks, S::GolemStatue, [M::Waxed, M::Exposed]),
        (C::FunctionalBlocks, S::GolemStatue, [M::Waxed, M::Weathered]),
        (C::FunctionalBlocks, S::GolemStatue, [M::Waxed, M::Oxidized]),

        (C::Ingredients, S::Nugget, []),
        (C::Ingredients, S::Ingot, []),

        (C::NaturalBlocks, S::Ore, []),
        (C::NaturalBlocks, S::Ore, [M::Deepslate]),
    ])]
    Copper,

    #[material(items = [
        (C::FunctionalBlocks, S::Base, []),
        (C::FunctionalBlocks, S::Dust, []),
    ])]
    Glowstone,

    #[material(items = [
        (C::FunctionalBlocks, S::Base, []),
    ])]
    Shroomlight,

    #[material(items = [
        (C::FunctionalBlocks, S::Base, [M::Ochre]),
        (C::FunctionalBlocks, S::Base, [M::Verdant]),
        (C::FunctionalBlocks, S::Base, [M::Pearlescent]),
    ])]
    Froglight,

    // Crying + Base
    #[material(items = [
        (C::NaturalBlocks, S::Base, []),
        (C::NaturalBlocks, S::Base, [M::Crying]),
    ])]
    Obsidian,

    // Block + Cream
    #[material(items = [
        (C::NaturalBlocks, S::Block, []),
        (C::NaturalBlocks, S::Cream, []),
    ])]
    Magma,

    #[material(items = [
        (C::FunctionalBlocks, S::Base, []),
    ])]
    CraftingTable,

    #[material(items = [
        (C::FunctionalBlocks, S::Base, []),
    ])]
    Stonecutter,

    #[material(items = [
        (C::FunctionalBlocks, S::Base, []),
    ])]
    CartographyTable,

    #[material(items = [
        (C::FunctionalBlocks, S::Base, []),
    ])]
    FletchingTable,

    #[material(items = [
        (C::FunctionalBlocks, S::Base, []),
    ])]
    SmithingTable,

    #[material(items = [
        (C::FunctionalBlocks, S::Base, []),
    ])]
    Grindstone,

    #[material(items = [
        (C::FunctionalBlocks, S::Base, []),
    ])]
    Loom,

    #[material(items = [
        (C::FunctionalBlocks, S::Base, []),
    ])]
    Furnace,

    #[material(items = [
        (C::FunctionalBlocks, S::Base, []),
    ])]
    Smoker,

    #[material(items = [
        (C::FunctionalBlocks, S::Base, []),
    ])]
    BlastFurnace,

    #[material(items = [
        (C::FunctionalBlocks, S::Base, []),
        (C::FunctionalBlocks, S::Base, [M::Soul]),
    ])]
    Campfire,

    #[material(items = [
        (C::FunctionalBlocks, S::Base, []),
        (C::FunctionalBlocks, S::Base, [M::Chipped]),
        (C::FunctionalBlocks, S::Base, [M::Damaged]),
    ])]
    Anvil,

    #[material(items = [
        (C::FunctionalBlocks, S::Base, []),
    ])]
    Composter,

    #[material(items = [
        (C::FunctionalBlocks, S::Base, []),
    ])]
    NoteBlock,

    #[material(items = [
        (C::FunctionalBlocks, S::Base, []),
    ])]
    Jukebox,

    #[material(items = [
        (C::FunctionalBlocks, S::Base, []),
    ])]
    EnchantingTable,

    #[material(items = [
        (C::Combat, S::Base, []),
    ])]
    EndCrystal,

    #[material(items = [
        (C::FunctionalBlocks, S::Base, []),
    ])]
    BrewingStand,

    #[material(items = [
        (C::FunctionalBlocks, S::Base, []),
    ])]
    Cauldron,

    #[material(items = [
        (C::FunctionalBlocks, S::Base, []),
    ])]
    Bell,

    #[material(items = [
        (C::FunctionalBlocks, S::Base, []),
    ])]
    Beacon,

    #[material(items = [
        (C::FunctionalBlocks, S::Base, []),
    ])]
    Conduit,

    #[material(items = [
        (C::FunctionalBlocks, S::Base, []),
    ])]
    Lodestone,

    #[material(items = [
        (C::FunctionalBlocks, S::Base, []),
    ])]
    Ladder,

    #[material(items = [
        (C::FunctionalBlocks, S::Base, []),
    ])]
    Scaffolding,

    #[material(items = [
        (C::NaturalBlocks, S::Base, []),
        (C::NaturalBlocks, S::Base, [M::Suspicious]),
    ])]
    Sand,

    #[material(items = [
        (C::NaturalBlocks, S::Base, []),
    ])]
    RedSand,

    #[material(items = [
        (C::NaturalBlocks, S::Base, []),
        (C::NaturalBlocks, S::Base, [M::Suspicious]),
    ])]
    Gravel,

    #[material(items = [
        (C::FunctionalBlocks, S::Base, []),
        (C::FunctionalBlocks, S::Base, [M::Exposed]),
        (C::FunctionalBlocks, S::Base, [M::Weathered]),
        (C::FunctionalBlocks, S::Base, [M::Oxidized]),
        (C::FunctionalBlocks, S::Base, [M::Waxed]),
        (C::FunctionalBlocks, S::Base, [M::Waxed, M::Exposed]),
        (C::FunctionalBlocks, S::Base, [M::Waxed, M::Weathered]),
        (C::FunctionalBlocks, S::Base, [M::Waxed, M::Oxidized]),
    ])]
    LightningRod,

    #[material(items = [
        (C::FunctionalBlocks, S::Base, [M::Flower]),
        (C::FunctionalBlocks, S::Base, [M::Decorated]),
    ])]
    Pot,

    #[material(items = [
        (C::FunctionalBlocks, S::Base, []),
    ])]
    ArmorStand,

    #[material(items = [
        (C::FunctionalBlocks, S::Base, []),
        (C::FunctionalBlocks, S::Base, [M::Glow]),
    ])]
    ItemFrame,

    #[material(items = [
        (C::FunctionalBlocks, S::Base, []),
    ])]
    Painting,

    #[material(items = [
        (C::FunctionalBlocks, S::Base, []),
        (C::FunctionalBlocks, S::Base, [M::Chiseled]),
    ])]
    Bookshelf,

    #[material(items = [
        (C::FunctionalBlocks, S::Base, []),
    ])]
    Lectern,

    #[material(items = [
        (C::FunctionalBlocks, S::Base, []),
        (C::FunctionalBlocks, S::Base, [M::Trapped]),
        (C::FunctionalBlocks, S::Base, [M::Copper]),
        (C::FunctionalBlocks, S::Base, [M::Exposed, M::Copper]),
        (C::FunctionalBlocks, S::Base, [M::Weathered, M::Copper]),
        (C::FunctionalBlocks, S::Base, [M::Oxidized, M::Copper]),
        (C::FunctionalBlocks, S::Base, [M::Waxed, M::Copper]),
        (C::FunctionalBlocks, S::Base, [M::Waxed, M::Exposed, M::Copper]),
        (C::FunctionalBlocks, S::Base, [M::Waxed, M::Weathered, M::Copper]),
        (C::FunctionalBlocks, S::Base, [M::Waxed, M::Oxidized, M::Copper]),
        (C::FunctionalBlocks, S::Base, [M::Ender]),
    ])]
    Chest,

    #[material(items = [
        (C::FunctionalBlocks, S::Base, []),
    ])]
    Barrel,

    #[material(items = [
        (C::FunctionalBlocks, S::Base, []),
    ])]
    RespawnAnchor,

    #[material(items = [
        (C::FunctionalBlocks, S::Base, []),
        (C::FunctionalBlocks, S::Base, [M::Wither]),
    ])]
    SkeletonSkull,

    #[material(items = [
        (C::FunctionalBlocks, S::Base, [M::Player]),
        (C::FunctionalBlocks, S::Base, [M::Zombie]),
        (C::FunctionalBlocks, S::Base, [M::Creeper]),
        (C::FunctionalBlocks, S::Base, [M::Piglin]),
        (C::FunctionalBlocks, S::Base, [M::Dragon]),
    ])]
    Head,

    #[material(items = [
        (C::FunctionalBlocks, S::Base, []),
    ])]
    EndPortalFrame,

    #[material(items = [
        (C::FunctionalBlocks, S::Base, []),
    ])]
    Vault,

    // => Redstone Blocks
    #[material(items = [
        (C::Ingredients, S::Base, []),
        (C::RedstoneBlocks, S::Block, []),
        (C::NaturalBlocks, S::Ore, []),
        (C::NaturalBlocks, S::Ore, [M::Deepslate]),
        (C::RedstoneBlocks, S::Lamp, []),
    ])]
    Redstone,

    #[material(items = [
        (C::RedstoneBlocks, S::Base, []),
    ])]
    Repeater,

    #[material(items = [
        (C::RedstoneBlocks, S::Base, []),
    ])]
    Comparator,

    #[material(items = [
        (C::RedstoneBlocks, S::Base, []),
    ])]
    Target,

    #[material(items = [
        (C::RedstoneBlocks, S::Base, []),
    ])]
    Lever,

    #[material(items = [
        (C::RedstoneBlocks, S::Base, []),
    ])]
    TripwireHook,

    #[material(items = [
        (C::RedstoneBlocks, S::Base, []),
    ])]
    DaylightDetector,

    #[material(items = [
        (C::RedstoneBlocks, S::Base, []),
        (C::RedstoneBlocks, S::Base, [M::Sticky]),
    ])]
    Piston,

    #[material(items = [
        (C::RedstoneBlocks, S::Base, []),
    ])]
    Dispenser,

    #[material(items = [
        (C::RedstoneBlocks, S::Base, []),
    ])]
    Dropper,

    #[material(items = [
        (C::RedstoneBlocks, S::Base, []),
    ])]
    Crafter,

    #[material(items = [
        (C::RedstoneBlocks, S::Base, []),
    ])]
    Hopper,

    #[material(items = [
        (C::RedstoneBlocks, S::Base, []),
    ])]
    Observer,

    #[material(items = [
        (C::RedstoneBlocks, S::Base, []),
        (C::RedstoneBlocks, S::Base, [M::Activator]),
        (C::RedstoneBlocks, S::Base, [M::Detector]),
        (C::RedstoneBlocks, S::Base, [M::Powered]),
    ])]
    Rail,

    // => Tools and Utilities
    #[material(items = [
        (C::ToolsAndUtilities, S::Base, [M::Wooden]),
        (C::ToolsAndUtilities, S::Base, [M::Stone]),
        (C::ToolsAndUtilities, S::Base, [M::Copper]),
        (C::ToolsAndUtilities, S::Base, [M::Iron]),
        (C::ToolsAndUtilities, S::Base, [M::Golden]),
        (C::ToolsAndUtilities, S::Base, [M::Diamond]),
        (C::ToolsAndUtilities, S::Base, [M::Netherite]),
    ])]
    Shovel,

    #[material(items = [
        (C::ToolsAndUtilities, S::Base, [M::Wooden]),
        (C::ToolsAndUtilities, S::Base, [M::Stone]),
        (C::ToolsAndUtilities, S::Base, [M::Copper]),
        (C::ToolsAndUtilities, S::Base, [M::Iron]),
        (C::ToolsAndUtilities, S::Base, [M::Golden]),
        (C::ToolsAndUtilities, S::Base, [M::Diamond]),
        (C::ToolsAndUtilities, S::Base, [M::Netherite]),
    ])]
    Pickaxe,

    #[material(items = [
        (C::ToolsAndUtilities, S::Base, [M::Wooden]),
        (C::ToolsAndUtilities, S::Base, [M::Stone]),
        (C::ToolsAndUtilities, S::Base, [M::Copper]),
        (C::ToolsAndUtilities, S::Base, [M::Iron]),
        (C::ToolsAndUtilities, S::Base, [M::Golden]),
        (C::ToolsAndUtilities, S::Base, [M::Diamond]),
        (C::ToolsAndUtilities, S::Base, [M::Netherite]),
    ])]
    Hoe,

    #[material(items = [
        (C::ToolsAndUtilities, S::Base, []),
        (C::ToolsAndUtilities, S::Base, [M::Water]),
        (C::ToolsAndUtilities, S::Base, [M::Cod]),
        (C::ToolsAndUtilities, S::Base, [M::Salmon]),
        (C::ToolsAndUtilities, S::Base, [M::TropicalFish]),
        (C::ToolsAndUtilities, S::Base, [M::Pufferfish]),
        (C::ToolsAndUtilities, S::Base, [M::Axolotl]),
        (C::ToolsAndUtilities, S::Base, [M::Tadpole]),
        (C::ToolsAndUtilities, S::Base, [M::SulfurCube]),
        (C::ToolsAndUtilities, S::Base, [M::Lava]),
        (C::ToolsAndUtilities, S::Base, [M::PowderSnow]),
        (C::ToolsAndUtilities, S::Base, [M::Milk]),
    ])]
    Bucket,

    #[material(items = [
        (C::ToolsAndUtilities, S::Base, []),
    ])]
    FishingRod,

    #[material(items = [
        (C::ToolsAndUtilities, S::Base, []),
    ])]
    FlintAndSteel,

    #[material(items = [
        (C::ToolsAndUtilities, S::Base, []),
    ])]
    Shears,

    #[material(items = [
        (C::ToolsAndUtilities, S::Base, []),
    ])]
    Brush,

    #[material(items = [
        (C::ToolsAndUtilities, S::Base, []),
    ])]
    NameTag,

    #[material(items = [
        (C::ToolsAndUtilities, S::Base, []),
    ])]
    Lead,

    #[material(items = [
        (C::ToolsAndUtilities, S::Base, []),
        (C::ToolsAndUtilities, S::Base, [M::White]),
        (C::ToolsAndUtilities, S::Base, [M::LightGray]),
        (C::ToolsAndUtilities, S::Base, [M::Gray]),
        (C::ToolsAndUtilities, S::Base, [M::Black]),
        (C::ToolsAndUtilities, S::Base, [M::Brown]),
        (C::ToolsAndUtilities, S::Base, [M::Red]),
        (C::ToolsAndUtilities, S::Base, [M::Orange]),
        (C::ToolsAndUtilities, S::Base, [M::Yellow]),
        (C::ToolsAndUtilities, S::Base, [M::Lime]),
        (C::ToolsAndUtilities, S::Base, [M::Green]),
        (C::ToolsAndUtilities, S::Base, [M::Cyan]),
        (C::ToolsAndUtilities, S::Base, [M::LightBlue]),
        (C::ToolsAndUtilities, S::Base, [M::Blue]),
        (C::ToolsAndUtilities, S::Base, [M::Purple]),
        (C::ToolsAndUtilities, S::Base, [M::Magenta]),
        (C::ToolsAndUtilities, S::Base, [M::Pink]),
    ])]
    Bundle,

    #[material(items = [
        (C::ToolsAndUtilities, S::Base, []),
        (C::ToolsAndUtilities, S::Base, [M::Recovery]),
    ])]
    Compass,

    #[material(items = [
        (C::ToolsAndUtilities, S::Base, []),
    ])]
    Clock,

    #[material(items = [
        (C::ToolsAndUtilities, S::Base, []),
    ])]
    Spyglass,

    #[material(items = [
        (C::ToolsAndUtilities, S::Base, []),
    ])]
    Map,

    #[material(items = [
        (C::ToolsAndUtilities, S::Base, []),
    ])]
    WindCharge,

    #[material(items = [
        (C::ToolsAndUtilities, S::Pearl, []),
        (C::ToolsAndUtilities, S::Eye, []),
    ])]
    Ender,

    #[material(items = [
        (C::ToolsAndUtilities, S::Base, []),
    ])]
    Elytra,

    #[material(items = [
        (C::ToolsAndUtilities, S::Base, []),
    ])]
    FireworkRocket,

    #[material(items = [
        (C::ToolsAndUtilities, S::Base, []),
    ])]
    Saddle,

    #[material(items = [
        (C::ToolsAndUtilities, S::Base, [M::White]),
        (C::ToolsAndUtilities, S::Base, [M::LightGray]),
        (C::ToolsAndUtilities, S::Base, [M::Gray]),
        (C::ToolsAndUtilities, S::Base, [M::Black]),
        (C::ToolsAndUtilities, S::Base, [M::Brown]),
        (C::ToolsAndUtilities, S::Base, [M::Red]),
        (C::ToolsAndUtilities, S::Base, [M::Orange]),
        (C::ToolsAndUtilities, S::Base, [M::Yellow]),
        (C::ToolsAndUtilities, S::Base, [M::Lime]),
        (C::ToolsAndUtilities, S::Base, [M::Green]),
        (C::ToolsAndUtilities, S::Base, [M::Cyan]),
        (C::ToolsAndUtilities, S::Base, [M::LightBlue]),
        (C::ToolsAndUtilities, S::Base, [M::Blue]),
        (C::ToolsAndUtilities, S::Base, [M::Purple]),
        (C::ToolsAndUtilities, S::Base, [M::Magenta]),
        (C::ToolsAndUtilities, S::Base, [M::Pink]),
    ])]
    Harness,

    #[material(items = [
        (C::ToolsAndUtilities, S::Base, [M::Carrot]),
        (C::ToolsAndUtilities, S::Base, [M::WarpedFungus]),
    ])]
    OnAStick,

    #[material(items = [
        (C::ToolsAndUtilities, S::Base, []),
        (C::ToolsAndUtilities, S::Base, [M::Hopper]),
        (C::ToolsAndUtilities, S::Base, [M::Chest]),
        (C::ToolsAndUtilities, S::Base, [M::Furnace]),
        (C::ToolsAndUtilities, S::Base, [M::Tnt]),
    ])]
    Minecart,

    #[material(items = [
        (C::ToolsAndUtilities, S::Base, []),
    ])]
    GoatHorn,

    #[material(items = [
        (C::ToolsAndUtilities, S::Thirteen, []),
        (C::ToolsAndUtilities, S::Cat, []),
        (C::ToolsAndUtilities, S::Blocks, []),
        (C::ToolsAndUtilities, S::Chirp, []),
        (C::ToolsAndUtilities, S::Far, []),
        (C::ToolsAndUtilities, S::Mall, []),
        (C::ToolsAndUtilities, S::Mellohi, []),
        (C::ToolsAndUtilities, S::Stal, []),
        (C::ToolsAndUtilities, S::Strad, []),
        (C::ToolsAndUtilities, S::Ward, []),
        (C::ToolsAndUtilities, S::Eleven, []),
        (C::ToolsAndUtilities, S::CreatorMusicBox, []),
        (C::ToolsAndUtilities, S::Wait, []),
        (C::ToolsAndUtilities, S::Creator, []),
        (C::ToolsAndUtilities, S::Precipice, []),
        (C::ToolsAndUtilities, S::Otherside, []),
        (C::ToolsAndUtilities, S::Relic, []),
        (C::ToolsAndUtilities, S::Five, []),
        (C::ToolsAndUtilities, S::Pigstep, []),
        (C::ToolsAndUtilities, S::Tears, []),
        (C::ToolsAndUtilities, S::LavaChicken, []),
        (C::ToolsAndUtilities, S::Bounce, []),
    ])]
    MusicDisc,

    // => Combat
    #[material(items = [
        (C::Combat, S::Base, [M::Wooden]),
        (C::Combat, S::Base, [M::Stone]),
        (C::Combat, S::Base, [M::Copper]),
        (C::Combat, S::Base, [M::Iron]),
        (C::Combat, S::Base, [M::Golden]),
        (C::Combat, S::Base, [M::Diamond]),
        (C::Combat, S::Base, [M::Netherite]),
    ])]
    Sword,

    #[material(items = [
        (C::Combat, S::Base, [M::Wooden]),
        (C::Combat, S::Base, [M::Stone]),
        (C::Combat, S::Base, [M::Copper]),
        (C::Combat, S::Base, [M::Iron]),
        (C::Combat, S::Base, [M::Golden]),
        (C::Combat, S::Base, [M::Diamond]),
        (C::Combat, S::Base, [M::Netherite]),
    ])]
    Spear,

    #[material(items = [
        (C::Combat, S::Base, [M::Wooden]),
        (C::Combat, S::Base, [M::Stone]),
        (C::Combat, S::Base, [M::Copper]),
        (C::Combat, S::Base, [M::Iron]),
        (C::Combat, S::Base, [M::Golden]),
        (C::Combat, S::Base, [M::Diamond]),
        (C::Combat, S::Base, [M::Netherite]),
    ])]
    Axe,

    #[material(items = [
        (C::Combat, S::Base, []),
    ])]
    Trident,

    #[material(items = [
        (C::Combat, S::Base, []),
    ])]
    Mace,

    #[material(items = [
        (C::Combat, S::Base, []),
    ])]
    Shield,

    #[material(items = [
        (C::Combat, S::Base, [M::Leather]),
        (C::Combat, S::Base, [M::Copper]),
        (C::Combat, S::Base, [M::Chainmail]),
        (C::Combat, S::Base, [M::Iron]),
        (C::Combat, S::Base, [M::Golden]),
        (C::Combat, S::Base, [M::Diamond]),
        (C::Combat, S::Base, [M::Netherite]),
        (C::Combat, S::Base, [M::Turtle]),
    ])]
    Helmet,

    #[material(items = [
        (C::Combat, S::Base, [M::Leather]),
        (C::Combat, S::Base, [M::Copper]),
        (C::Combat, S::Base, [M::Chainmail]),
        (C::Combat, S::Base, [M::Iron]),
        (C::Combat, S::Base, [M::Golden]),
        (C::Combat, S::Base, [M::Diamond]),
        (C::Combat, S::Base, [M::Netherite]),
    ])]
    Chestplate,

    #[material(items = [
        (C::Combat, S::Base, [M::Leather]),
        (C::Combat, S::Base, [M::Copper]),
        (C::Combat, S::Base, [M::Chainmail]),
        (C::Combat, S::Base, [M::Iron]),
        (C::Combat, S::Base, [M::Golden]),
        (C::Combat, S::Base, [M::Diamond]),
        (C::Combat, S::Base, [M::Netherite]),
    ])]
    Leggings,

    #[material(items = [
        (C::Combat, S::Base, [M::Leather]),
        (C::Combat, S::Base, [M::Copper]),
        (C::Combat, S::Base, [M::Chainmail]),
        (C::Combat, S::Base, [M::Iron]),
        (C::Combat, S::Base, [M::Golden]),
        (C::Combat, S::Base, [M::Diamond]),
        (C::Combat, S::Base, [M::Netherite]),
    ])]
    Boots,

    #[material(items = [
        (C::Combat, S::Base, []),
        (C::Combat, S::Base, [M::Spectral]),
        (C::Combat, S::Base, [M::Tipped]),
    ])]
    Arrow,

    #[material(items = [
        (C::Combat, S::Base, [M::Leather]),
        (C::Combat, S::Base, [M::Copper]),
        (C::Combat, S::Base, [M::Iron]),
        (C::Combat, S::Base, [M::Golden]),
        (C::Combat, S::Base, [M::Diamond]),
        (C::Combat, S::Base, [M::Netherite]),
    ])]
    HorseArmor,

    #[material(items = [
        (C::Combat, S::Base, []),
    ])]
    WolfArmor,

    #[material(items = [
        (C::Combat, S::Base, [M::Copper]),
        (C::Combat, S::Base, [M::Iron]),
        (C::Combat, S::Base, [M::Golden]),
        (C::Combat, S::Base, [M::Diamond]),
        (C::Combat, S::Base, [M::Netherite]),
    ])]
    NautilusArmor,

    #[material(items = [
        (C::Combat, S::Base, []),
    ])]
    TotemOfUndying,

    #[material(items = [
        (C::Combat, S::Base, []),
    ])]
    Tnt,

    #[material(items = [
        (C::Combat, S::Base, []),
    ])]
    Snowball,

    #[material(items = [
        (C::Combat, S::Base, []),
        (C::Combat, S::Base, [M::Brown]),
        (C::Combat, S::Base, [M::Blue]),

        (C::NaturalBlocks, S::Base, [M::Turtle]),
        (C::NaturalBlocks, S::Base, [M::Sniffer]),
        (C::NaturalBlocks, S::Base, [M::Dragon]),
    ])]
    Egg,

    #[material(items = [
        (C::Combat, S::Base, []),
    ])]
    Bow,

    #[material(items = [
        (C::Combat, S::Base, []),
    ])]
    Crossbow,

    // => Food and Drinks
    #[material(items = [
        (C::FoodAndDrinks, S::Base, []),
        (C::FoodAndDrinks, S::Base, [M::Golden]),
        (C::FoodAndDrinks, S::Base, [M::Enchanted, M::Golden]),
    ])]
    Apple,

    #[material(items = [
        (C::FoodAndDrinks, S::Base, []),
        (C::NaturalBlocks, S::Seeds, []),
        (C::FoodAndDrinks, S::Slice, []),
        (C::FoodAndDrinks, S::Slice, [M::Glistering]),
    ])]
    Melon,

    #[material(items = [
        (C::FoodAndDrinks, S::Base, []),
    ])]
    SweetBerries,

    #[material(items = [
        (C::FoodAndDrinks, S::Base, []),
    ])]
    GlowBerries,

    #[material(items = [
        (C::FoodAndDrinks, S::Base, []),
        (C::FoodAndDrinks, S::Base, [M::Golden]),
    ])]
    Carrot,

    #[material(items = [
        (C::FoodAndDrinks, S::Base, []),
        (C::FoodAndDrinks, S::Base, [M::Baked]),
        (C::FoodAndDrinks, S::Base, [M::Poisonous]),
    ])]
    Potato,

    #[material(items = [
        (C::FoodAndDrinks, S::Base, []),
        (C::NaturalBlocks, S::Seeds, []),
    ])]
    Beetroot,

    #[material(items = [
        (C::NaturalBlocks, S::Base, []),
        (C::FoodAndDrinks, S::Base, [M::Dried]),
        (C::NaturalBlocks, S::Block, [M::Dried]),
    ])]
    Kelp,

    #[material(items = [
        (C::FoodAndDrinks, S::Base, []),
        (C::FoodAndDrinks, S::Base, [M::Cooked]),
    ])]
    Beef,

    #[material(items = [
        (C::FoodAndDrinks, S::Base, []),
        (C::FoodAndDrinks, S::Base, [M::Cooked]),
    ])]
    Porkchop,

    #[material(items = [
        (C::FoodAndDrinks, S::Base, []),
        (C::FoodAndDrinks, S::Base, [M::Cooked]),
    ])]
    Mutton,

    #[material(items = [
        (C::FoodAndDrinks, S::Base, []),
        (C::FoodAndDrinks, S::Base, [M::Cooked]),
    ])]
    Chicken,

    #[material(items = [
        (C::FoodAndDrinks, S::Base, []),
        (C::FoodAndDrinks, S::Base, [M::Cooked]),
    ])]
    Rabbit,

    #[material(items = [
        (C::FoodAndDrinks, S::Base, []),
        (C::FoodAndDrinks, S::Base, [M::Cooked]),
    ])]
    Cod,

    #[material(items = [
        (C::FoodAndDrinks, S::Base, []),
        (C::FoodAndDrinks, S::Base, [M::Cooked]),
    ])]
    Salmon,

    #[material(items = [
        (C::FoodAndDrinks, S::Base, []),
    ])]
    TropicalFish,

    #[material(items = [
        (C::FoodAndDrinks, S::Base, []),
    ])]
    Pufferfish,

    #[material(items = [
        (C::FoodAndDrinks, S::Base, []),
    ])]
    Bread,

    #[material(items = [
        (C::FoodAndDrinks, S::Base, []),
    ])]
    Cookie,

    #[material(items = [
        (C::FoodAndDrinks, S::Base, []),
    ])]
    Cake,

    #[material(items = [
        (C::FoodAndDrinks, S::Base, []),
    ])]
    RottenFlesh,

    #[material(items = [
        (C::FoodAndDrinks, S::Base, []),
        (C::Ingredients, S::Base, [M::Fermented]),
    ])]
    SpiderEye,

    #[material(items = [
        (C::FoodAndDrinks, S::Base, [M::Mushroom]),
        (C::FoodAndDrinks, S::Base, [M::Rabbit]),
        (C::FoodAndDrinks, S::Base, [M::Suspicious]),
    ])]
    Stew,

    #[material(items = [
        (C::NaturalBlocks, S::Base, [M::Red]),
        (C::NaturalBlocks, S::Block, [M::Red]),
        (C::NaturalBlocks, S::Base, [M::Brown]),
        (C::NaturalBlocks, S::Block, [M::Brown]),
        (C::NaturalBlocks, S::Stem, []),
    ])]
    Mushroom,

    #[material(items = [
        (C::FoodAndDrinks, S::Base, [M::Beetroot]),
    ])]
    Soup,

    #[material(items = [
        (C::FoodAndDrinks, S::Base, [M::Honey]),
        (C::FoodAndDrinks, S::Base, [M::Ominous]),
        (C::Ingredients, S::Base, [M::Glass]),
        (C::Ingredients, S::Base, [M::Experience]),
    ])]
    Bottle,

    #[material(items = [
        (C::FoodAndDrinks, S::Base, []),
        (C::FoodAndDrinks, S::Base, [M::Splash]),
        (C::FoodAndDrinks, S::Base, [M::Lingering]),
    ])]
    Potion,

    // => Ingredients
    #[material(items = [
        (C::Ingredients, S::Base, []),
        (C::NaturalBlocks, S::Block, []),
        (C::NaturalBlocks, S::Ore, []),
        (C::NaturalBlocks, S::Ore, [M::Deepslate]),
    ])]
    Coal,

    #[material(items = [
        (C::Ingredients, S::Base, []),
    ])]
    Charcoal,

    #[material(items = [
        (C::Ingredients, S::Base, [M::Raw]),
        (C::BuildingBlocks, S::Block, []),
        (C::NaturalBlocks, S::Block, [M::Raw]),
        (C::BuildingBlocks, S::Bars, []),
        (C::BuildingBlocks, S::Door, []),
        (C::BuildingBlocks, S::Trapdoor, []),
        // Add `Iron` to the `Chain` family instead for grouping.
        // (C::FunctionalBlocks, S::Chain, []),
        (C::Ingredients, S::Nugget, []),
        (C::Ingredients, S::Ingot, []),

        (C::NaturalBlocks, S::Ore, []),
        (C::NaturalBlocks, S::Ore, [M::Deepslate]),
    ])]
    Iron,

    #[material(items = [
        (C::RedstoneBlocks, S::Base, []),
    ])]
    HeavyWeightedPressurePlate,

    #[material(items = [
        (C::Ingredients, S::Base, [M::Raw]),
        (C::BuildingBlocks, S::Block, []),
        (C::NaturalBlocks, S::Block, [M::Raw]),
        (C::Ingredients, S::Nugget, []),
        (C::Ingredients, S::Ingot, []),

        (C::NaturalBlocks, S::Ore, []),
        (C::NaturalBlocks, S::Ore, [M::Deepslate]),
        (C::NaturalBlocks, S::Ore, [M::Nether]),
    ])]
    Gold,

    #[material(items = [
        (C::RedstoneBlocks, S::Base, []),
    ])]
    LightWeightedPressurePlate,

    #[material(items = [
        (C::Ingredients, S::Base, []),
        (C::BuildingBlocks, S::Block, []),

        (C::NaturalBlocks, S::Ore, []),
        (C::NaturalBlocks, S::Ore, [M::Deepslate]),
    ])]
    Emerald,

    #[material(items = [
        (C::Ingredients, S::Lazuli, []),
        (C::BuildingBlocks, S::Block, []),

        (C::NaturalBlocks, S::Ore, []),
        (C::NaturalBlocks, S::Ore, [M::Deepslate]),
    ])]
    Lapis,

    #[material(items = [
        (C::Ingredients, S::Base, []),
        (C::BuildingBlocks, S::Block, []),

        (C::NaturalBlocks, S::Ore, []),
        (C::NaturalBlocks, S::Ore, [M::Deepslate]),
    ])]
    Diamond,

    #[material(items = [
        (C::Ingredients, S::Base, []),
    ])]
    AncientDebris,

    #[material(items = [
        (C::Ingredients, S::Scrap, []),
        (C::Ingredients, S::Ingot, []),
        (C::BuildingBlocks, S::Block, []),
    ])]
    Netherite,

    #[material(items = [
        (C::Ingredients, S::Base, []),
    ])]
    Stick,

    #[material(items = [
        (C::Ingredients, S::Base, []),
    ])]
    Flint,

    #[material(items = [
        (C::Ingredients, S::Base, []),
        (C::Ingredients, S::Meal, []),
        (C::Ingredients, S::Block, []),
    ])]
    Bone,

    #[material(items = [
        (C::Ingredients, S::Base, []),
    ])]
    String,

    #[material(items = [
        (C::Ingredients, S::Base, []),
    ])]
    Feather,

    #[material(items = [
        (C::Ingredients, S::Base, []),
    ])]
    Leather,

    #[material(items = [
        (C::Ingredients, S::Base, []),
    ])]
    RabbitHide,

    #[material(items = [
        (C::Ingredients, S::Base, []),
        (C::Ingredients, S::Base, [M::Glow]),
    ])]
    InkSac,

    #[material(items = [
        (C::NaturalBlocks, S::Base, []),
        (C::Ingredients, S::Ball, []),
    ])]
    Clay,

    #[material(items = [
        (C::Ingredients, S::Base, []),
    ])]
    NautilusShell,

    #[material(items = [
        (C::Ingredients, S::Base, []),
    ])]
    HeartOfTheSea,

    #[material(items = [
        (C::Ingredients, S::Base, []),
    ])]
    FireCharge,

    #[material(items = [
        (C::Ingredients, S::Base, [M::Blaze]),
        (C::Ingredients, S::Base, [M::Breeze]),
    ])]
    Rod,

    #[material(items = [
        (C::Ingredients, S::Base, []),
    ])]
    HeavyCore,

    #[material(items = [
        (C::Ingredients, S::Base, []),
    ])]
    NetherStar,

    #[material(items = [
        (C::Ingredients, S::Base, []),
    ])]
    EchoShard,

    #[material(items = [
        (C::Ingredients, S::Five, []),
    ])]
    DiscFragment,

    #[material(items = [
        (C::Ingredients, S::Base, []),
    ])]
    Bowl,

    #[material(items = [
        (C::Ingredients, S::Base, []),
    ])]
    Brick,

    #[material(items = [
        (C::Ingredients, S::Base, []),
    ])]
    Paper,

    #[material(items = [
        (C::Ingredients, S::Base, []),
        (C::Ingredients, S::Base, [M::Writable]),
        (C::Ingredients, S::Base, [M::Enchanted]),
    ])]
    Book,

    #[material(items = [
        (C::Ingredients, S::Base, []),
    ])]
    FireworkStar,

    #[material(items = [
        (C::Ingredients, S::Base, []),
    ])]
    Gunpowder,

    #[material(items = [
        (C::Ingredients, S::Base, []),
    ])]
    DragonBreath,

    #[material(items = [
        (C::Ingredients, S::Base, []),
    ])]
    BlazePowder,

    #[material(items = [
        (C::Ingredients, S::Base, []),
    ])]
    Sugar,

    #[material(items = [
        (C::Ingredients, S::Base, []),
    ])]
    RabbitFoot,

    #[material(items = [
        (C::Ingredients, S::Base, []),
    ])]
    GhastTear,

    #[material(items = [
        (C::Ingredients, S::Base, []),
    ])]
    PhantomMembrane,

    #[material(items = [
        (C::Ingredients, S::Base, [M::BordureIndented]),
        (C::Ingredients, S::Base, [M::Creeper]),
        (C::Ingredients, S::Base, [M::FieldMasoned]),
        (C::Ingredients, S::Base, [M::Flow]),
        (C::Ingredients, S::Base, [M::Flower]),
        (C::Ingredients, S::Base, [M::Globe]),
        (C::Ingredients, S::Base, [M::Guster]),
        (C::Ingredients, S::Base, [M::Mojang]),
        (C::Ingredients, S::Base, [M::Piglin]),
        (C::Ingredients, S::Base, [M::Skull]),
    ])]
    BannerPattern,

    #[material(items = [
        (C::Ingredients, S::Base, []),
        (C::Ingredients, S::Base, [M::Ominous]),
    ])]
    TrialKey,

    #[material(items = [
        (C::Ingredients, S::Base, []),
        (C::BuildingBlocks, S::Block, []),
        (C::BuildingBlocks, S::Block, [M::Chiseled]),
        (C::BuildingBlocks, S::Bricks, []),
        (C::BuildingBlocks, S::Stairs, []),
        (C::BuildingBlocks, S::Slab, []),
        (C::BuildingBlocks, S::Pillar, []),
        (C::BuildingBlocks, S::Base, [M::Smooth]),
        (C::BuildingBlocks, S::Stairs, [M::Smooth]),
        (C::BuildingBlocks, S::Slab, [M::Smooth]),

        (C::NaturalBlocks, S::Ore, [M::Nether]),
    ])]
    Quartz,

    #[material(items = [
        (C::Ingredients, S::Base, [M::NetheriteUpgrade]),
        (C::Ingredients, S::Base, [M::BoltArmorTrim]),
        (C::Ingredients, S::Base, [M::CoastArmorTrim]),
        (C::Ingredients, S::Base, [M::DuneArmorTrim]),
        (C::Ingredients, S::Base, [M::EyeArmorTrim]),
        (C::Ingredients, S::Base, [M::FlowArmorTrim]),
        (C::Ingredients, S::Base, [M::HostArmorTrim]),
        (C::Ingredients, S::Base, [M::RaiserArmorTrim]),
        (C::Ingredients, S::Base, [M::RibArmorTrim]),
        (C::Ingredients, S::Base, [M::SentryArmorTrim]),
        (C::Ingredients, S::Base, [M::ShaperArmorTrim]),
        (C::Ingredients, S::Base, [M::SilenceArmorTrim]),
        (C::Ingredients, S::Base, [M::SnoutArmorTrim]),
        (C::Ingredients, S::Base, [M::SpireArmorTrim]),
        (C::Ingredients, S::Base, [M::TideArmorTrim]),
        (C::Ingredients, S::Base, [M::VexArmorTrim]),
        (C::Ingredients, S::Base, [M::WardArmorTrim]),
        (C::Ingredients, S::Base, [M::WayfinderArmorTrim]),
        (C::Ingredients, S::Base, [M::WildArmorTrim]),
    ])]
    SmithingTemplate,

    #[material(items = [
        (C::Ingredients, S::Base, [M::Angler]),
        (C::Ingredients, S::Base, [M::Archer]),
        (C::Ingredients, S::Base, [M::ArmsUp]),
        (C::Ingredients, S::Base, [M::Blade]),
        (C::Ingredients, S::Base, [M::Brewer]),
        (C::Ingredients, S::Base, [M::Burn]),
        (C::Ingredients, S::Base, [M::Danger]),
        (C::Ingredients, S::Base, [M::Explorer]),
        (C::Ingredients, S::Base, [M::Flow]),
        (C::Ingredients, S::Base, [M::Friend]),
        (C::Ingredients, S::Base, [M::Guster]),
        (C::Ingredients, S::Base, [M::Heart]),
        (C::Ingredients, S::Base, [M::Heartbreak]),
        (C::Ingredients, S::Base, [M::Howl]),
        (C::Ingredients, S::Base, [M::Miner]),
        (C::Ingredients, S::Base, [M::Mourner]),
        (C::Ingredients, S::Base, [M::Plenty]),
        (C::Ingredients, S::Base, [M::Prize]),
        (C::Ingredients, S::Base, [M::Scrape]),
        (C::Ingredients, S::Base, [M::Sheaf]),
        (C::Ingredients, S::Base, [M::Shelter]),
        (C::Ingredients, S::Base, [M::Skull]),
        (C::Ingredients, S::Base, [M::Snort]),
    ])]
    PotterySherd,

    #[material(items = [
        (C::Ingredients, S::Base, [M::Turtle]),
        (C::Ingredients, S::Base, [M::Armadillo]),
    ])]
    Scute,

    #[material(items = [
        (C::ColoredBlocks, S::Base, [M::White]),
        (C::ColoredBlocks, S::Base, [M::LightGray]),
        (C::ColoredBlocks, S::Base, [M::Gray]),
        (C::ColoredBlocks, S::Base, [M::Black]),
        (C::ColoredBlocks, S::Base, [M::Brown]),
        (C::ColoredBlocks, S::Base, [M::Red]),
        (C::ColoredBlocks, S::Base, [M::Orange]),
        (C::ColoredBlocks, S::Base, [M::Yellow]),
        (C::ColoredBlocks, S::Base, [M::Lime]),
        (C::ColoredBlocks, S::Base, [M::Green]),
        (C::ColoredBlocks, S::Base, [M::Cyan]),
        (C::ColoredBlocks, S::Base, [M::LightBlue]),
        (C::ColoredBlocks, S::Base, [M::Blue]),
        (C::ColoredBlocks, S::Base, [M::Purple]),
        (C::ColoredBlocks, S::Base, [M::Magenta]),
        (C::ColoredBlocks, S::Base, [M::Pink]),
    ])]
    Dye,

    // => Spawn Eggs
    #[material(items = [
        (C::SpawnEggs, S::Base, []),
        (C::SpawnEggs, S::Base, [M::Trial]),
    ])]
    Spawner,

    #[material(items = [
        (C::SpawnEggs, S::Base, []),
    ])]
    CreakingHeart,

    #[material(items = [
        (C::SpawnEggs, S::Base, [M::Chicken]),
        (C::SpawnEggs, S::Base, [M::Cow]),
        (C::SpawnEggs, S::Base, [M::Pig]),
        (C::SpawnEggs, S::Base, [M::Sheep]),
        (C::SpawnEggs, S::Base, [M::Camel]),
        (C::SpawnEggs, S::Base, [M::Donkey]),
        (C::SpawnEggs, S::Base, [M::Horse]),
        (C::SpawnEggs, S::Base, [M::Mule]),
        (C::SpawnEggs, S::Base, [M::Cat]),
        (C::SpawnEggs, S::Base, [M::Parrot]),
        (C::SpawnEggs, S::Base, [M::Wolf]),
        (C::SpawnEggs, S::Base, [M::Armadillo]),
        (C::SpawnEggs, S::Base, [M::Bat]),
        (C::SpawnEggs, S::Base, [M::Bee]),
        (C::SpawnEggs, S::Base, [M::Fox]),
        (C::SpawnEggs, S::Base, [M::Goat]),
        (C::SpawnEggs, S::Base, [M::Llama]),
        (C::SpawnEggs, S::Base, [M::Ocelot]),
        (C::SpawnEggs, S::Base, [M::Panda]),
        (C::SpawnEggs, S::Base, [M::PolarBear]),
        (C::SpawnEggs, S::Base, [M::Rabbit]),
        (C::SpawnEggs, S::Base, [M::Axolotl]),
        (C::SpawnEggs, S::Base, [M::Cod]),
        (C::SpawnEggs, S::Base, [M::Dolphin]),
        (C::SpawnEggs, S::Base, [M::Frog]),
        (C::SpawnEggs, S::Base, [M::GlowSquid]),
        (C::SpawnEggs, S::Base, [M::Nautilus]),
        (C::SpawnEggs, S::Base, [M::Pufferfish]),
        (C::SpawnEggs, S::Base, [M::Salmon]),
        (C::SpawnEggs, S::Base, [M::Squid]),
        (C::SpawnEggs, S::Base, [M::Tadpole]),
        (C::SpawnEggs, S::Base, [M::TropicalFish]),
        (C::SpawnEggs, S::Base, [M::Turtle]),
        (C::SpawnEggs, S::Base, [M::Allay]),
        (C::SpawnEggs, S::Base, [M::Mooshroom]),
        (C::SpawnEggs, S::Base, [M::Sniffer]),
        (C::SpawnEggs, S::Base, [M::SulfurCube]),
        (C::SpawnEggs, S::Base, [M::CopperGolem]),
        (C::SpawnEggs, S::Base, [M::IronGolem]),
        (C::SpawnEggs, S::Base, [M::SnowGolem]),
        (C::SpawnEggs, S::Base, [M::TraderLlama]),
        (C::SpawnEggs, S::Base, [M::Villager]),
        (C::SpawnEggs, S::Base, [M::WanderingTrader]),
        (C::SpawnEggs, S::Base, [M::Bogged]),
        (C::SpawnEggs, S::Base, [M::CamelHusk]),
        (C::SpawnEggs, S::Base, [M::Drowned]),
        (C::SpawnEggs, S::Base, [M::Husk]),
        (C::SpawnEggs, S::Base, [M::Parched]),
        (C::SpawnEggs, S::Base, [M::Skeleton]),
        (C::SpawnEggs, S::Base, [M::SkeletonHorse]),
        (C::SpawnEggs, S::Base, [M::Stray]),
        (C::SpawnEggs, S::Base, [M::Zombie]),
        (C::SpawnEggs, S::Base, [M::ZombieHorse]),
        (C::SpawnEggs, S::Base, [M::ZombieNautilus]),
        (C::SpawnEggs, S::Base, [M::ZombieVillager]),
        (C::SpawnEggs, S::Base, [M::CaveSpider]),
        (C::SpawnEggs, S::Base, [M::Spider]),
        (C::SpawnEggs, S::Base, [M::Breeze]),
        (C::SpawnEggs, S::Base, [M::Creaking]),
        (C::SpawnEggs, S::Base, [M::Creeper]),
        (C::SpawnEggs, S::Base, [M::ElderGuardian]),
        (C::SpawnEggs, S::Base, [M::Guardian]),
        (C::SpawnEggs, S::Base, [M::Phantom]),
        (C::SpawnEggs, S::Base, [M::Silverfish]),
        (C::SpawnEggs, S::Base, [M::Slime]),
        (C::SpawnEggs, S::Base, [M::Warden]),
        (C::SpawnEggs, S::Base, [M::Witch]),
        (C::SpawnEggs, S::Base, [M::Evoker]),
        (C::SpawnEggs, S::Base, [M::Pillager]),
        (C::SpawnEggs, S::Base, [M::Ravager]),
        (C::SpawnEggs, S::Base, [M::Vex]),
        (C::SpawnEggs, S::Base, [M::Vindicator]),
        (C::SpawnEggs, S::Base, [M::Blaze]),
        (C::SpawnEggs, S::Base, [M::Ghast]),
        (C::SpawnEggs, S::Base, [M::HappyGhast]),
        (C::SpawnEggs, S::Base, [M::Hoglin]),
        (C::SpawnEggs, S::Base, [M::MagmaCube]),
        (C::SpawnEggs, S::Base, [M::Piglin]),
        (C::SpawnEggs, S::Base, [M::PiglinBrute]),
        (C::SpawnEggs, S::Base, [M::Strider]),
        (C::SpawnEggs, S::Base, [M::WitherSkeleton]),
        (C::SpawnEggs, S::Base, [M::Zoglin]),
        (C::SpawnEggs, S::Base, [M::ZombifiedPiglin]),
        (C::SpawnEggs, S::Base, [M::Enderman]),
        (C::SpawnEggs, S::Base, [M::Endermite]),
        (C::SpawnEggs, S::Base, [M::Shulker]),
    ])]
    SpawnEgg,
}
