#[derive(
    Debug, Clone, Copy, Hash, PartialEq, Eq, serde::Serialize, serde::Deserialize, strum::Display,
)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Modifier {
    // Wood
    Stripped,

    // Mangrove
    Muddy,

    // Azalea
    Flowering,

    // Stone-like
    Polished,

    // Stone
    Mossy,
    Cracked,
    Cobbled,
    Reinforced,
    Smooth,

    // Mud
    Packed,

    // Copper
    Exposed,
    Weathered,
    Oxidized,
    Waxed,
    Cut,
    Chiseled,

    // Blackstone
    Gilded,

    // Glass
    Tinted,
    Stained,

    // Dirt
    Coarse,
    Rooted,

    // Grass
    Short,
    Tall,
    Dry,

    // Coral
    Dead,
    Brain,
    Bubble,
    Fire,
    Horn,
    Tube,

    // Froglights
    Ochre,
    Verdant,
    Pearlescent,

    // Dripstone
    Pointed,

    // Ores
    Deepslate,
    Nether,

    // Torches
    Soul,
    Redstone,

    // Anvil
    Chipped,
    Damaged,

    // Apple / Books
    Enchanted,
    Writable,

    // Obsidian
    Crying,

    // Spider Eye
    Fermented,

    // Prismarine
    Dark,

    // Item Frames
    Glow,

    // Spawner
    Trial,

    // Potion
    Lingering,
    Splash,

    // Piston
    Sticky,

    // Chorus
    Popped,

    // Sculk
    Calibrated,

    // Pots
    Flower,
    Decorated,

    // Arrows
    Spectral,
    Tipped,

    // Eyeblossom
    Closed,
    Open,

    // Vines
    Twisting,
    Weeping,

    // Potato
    Baked,
    Poisonous,

    // Melon
    Glistering,

    // Kelp
    Dried,

    // Armor
    Leather,
    Copper,
    Iron,
    Golden,
    Diamond,
    Netherite,

    // Dripleaf
    Big,

    // Compass
    Recovery,

    // OnAStick
    Carrot,
    WarpedFungus,

    // Minecart
    Hopper,
    Chest,
    Furnace,
    Tnt,

    // Weapons
    Wooden,
    Stone,
    // Copper,
    // Iron,
    // Golden,
    // Diamond,
    // Netherite,

    // Armor
    // Leather,
    // Copper,
    Chainmail,
    // Iron,
    // Golden,
    // Diamond,
    // Netherite,

    // Eggs
    // Brown,
    // Blue,

    // Infested stones
    Infested,

    // Skeleton Skull
    Wither,

    // Chests
    Ender,
    Trapped,

    // Ores
    Raw,

    // Player Heads
    Player,
    // Zombie,
    // Creeper,
    // Piglin,
    Dragon,

    // Amethyst
    Budding,
    Small,
    Medium,
    Large,

    // Rails
    Activator,
    Detector,
    Powered,

    // Bucket
    Water,
    Lava,
    PowderSnow,
    Milk,

    // Sponge
    Wet,

    // Food
    Cooked,

    // Stew/Soup
    Mushroom,
    Beetroot,
    Suspicious,

    // Bottle
    Glass,
    Honey,
    Ominous,
    Experience,

    // Pumpkin
    Carved,

    // Colors
    White,
    LightGray,
    Gray,
    Black,
    Brown,
    Red,
    Orange,
    Yellow,
    Lime,
    Green,
    Cyan,
    LightBlue,
    Blue,
    Purple,
    Magenta,
    Pink,

    // Banner Patterns
    BordureIndented,
    // Creeper,
    FieldMasoned,
    // Flow,
    // Flower,
    Globe,
    // Guster,
    Mojang,
    // Piglin,
    // Skull,

    // Smithing Templates
    NetheriteUpgrade,
    BoltArmorTrim,
    CoastArmorTrim,
    DuneArmorTrim,
    EyeArmorTrim,
    FlowArmorTrim,
    HostArmorTrim,
    RaiserArmorTrim,
    RibArmorTrim,
    SentryArmorTrim,
    ShaperArmorTrim,
    SilenceArmorTrim,
    SnoutArmorTrim,
    SpireArmorTrim,
    TideArmorTrim,
    VexArmorTrim,
    WardArmorTrim,
    WayfinderArmorTrim,
    WildArmorTrim,

    // Pottery Sherds
    Angler,
    Archer,
    ArmsUp,
    Blade,
    Brewer,
    Burn,
    Danger,
    Explorer,
    Flow,
    Friend,
    Guster,
    Heart,
    Heartbreak,
    Howl,
    Miner,
    Mourner,
    Plenty,
    Prize,
    Scrape,
    Sheaf,
    Shelter,
    Skull,
    Snort,

    // Spawn Eggs
    Chicken,
    Cow,
    Pig,
    Sheep,
    Camel,
    Donkey,
    Horse,
    Mule,
    Cat,
    Parrot,
    Wolf,
    Armadillo,
    Bat,
    Bee,
    Fox,
    Goat,
    Llama,
    Ocelot,
    Panda,
    PolarBear,
    Rabbit,
    Axolotl,
    Cod,
    Dolphin,
    Frog,
    GlowSquid,
    Nautilus,
    Pufferfish,
    Salmon,
    Squid,
    Tadpole,
    TropicalFish,
    Turtle,
    Allay,
    Mooshroom,
    Sniffer,
    SulfurCube,
    CopperGolem,
    IronGolem,
    SnowGolem,
    TraderLlama,
    Villager,
    WanderingTrader,
    Bogged,
    CamelHusk,
    Drowned,
    Husk,
    Parched,
    Skeleton,
    SkeletonHorse,
    Stray,
    Zombie,
    ZombieHorse,
    ZombieNautilus,
    ZombieVillager,
    CaveSpider,
    Spider,
    Breeze,
    Creaking,
    Creeper,
    ElderGuardian,
    Guardian,
    Phantom,
    Silverfish,
    Slime,
    Warden,
    Witch,
    Evoker,
    Pillager,
    Ravager,
    Vex,
    Vindicator,
    Blaze,
    Ghast,
    HappyGhast,
    Hoglin,
    MagmaCube,
    Piglin,
    PiglinBrute,
    Strider,
    WitherSkeleton,
    Zoglin,
    ZombifiedPiglin,
    Enderman,
    Endermite,
    Shulker,
}
