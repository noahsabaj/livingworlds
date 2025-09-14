//! World name components for epic planet/realm name generation
//!
//! These components can be combined in various patterns to create
//! names like "Crystal Gaia Prime" or "The Endless Realm"

/// Prefixes that can modify world names
pub const PREFIXES: &[&str] = &[
    "New", "Ancient", "Lost", "Eternal", "Prime", "Nova", "Neo", "Crystal",
    "Golden", "Silver", "Mystic", "Shadow", "Dawn", "Twilight", "Astral",
    "Forgotten", "Hidden", "Sacred", "Divine", "Cosmic", "Infinite", "Pristine",
    "Shattered", "Blessed", "Cursed", "Awakened", "Sleeping", "Burning", "Frozen",
    "Living", "Dying", "Rising", "Fallen", "First", "Last", "True", "False",
];

/// Core root words for world names
pub const ROOTS: &[&str] = &[
    "Terra", "Gaia", "Eden", "Avalon", "Elysium", "Pangaea", "Atlantis",
    "Aetheria", "Celestia", "Arcadia", "Zephyr", "Olympus", "Valhalla",
    "Midgard", "Asgard", "Nibiru", "Xanadu", "Shangri", "Lemuria",
    "Hyperborea", "Thule", "Mu", "Eldoria", "Mythos", "Cosmos",
    "Solaris", "Lunaris", "Stellaris", "Nebula", "Aurora", "Phoenix",
    "Utopia", "Dystopia", "Paradiso", "Inferno", "Purgatorio", "Limbo",
    "Nirvana", "Samsara", "Karma", "Dharma", "Zenith", "Nadir",
];

/// Suffixes that can be appended to world names
pub const SUFFIXES: &[&str] = &[
    "", " Prime", " Nova", " Alpha", " Beta", " Omega", " Major", " Minor",
    " Reborn", " Ascendant", " Eternal", " Infinite", " Pristine",
    " Secundus", " Tertius", " Maximus", " Magnus", " Ultima",
    " Rising", " Falling", " Lost", " Found", " Blessed", " Cursed",
];

/// Complete epic names that stand alone
pub const EPIC_NAMES: &[&str] = &[
    "Genesis", "Revelation", "Paradox", "Eternity", "Infinity",
    "Chronos", "Nexus", "Apex", "Zenith", "Odyssey", "Legacy",
    "Equilibrium", "Singularity", "Horizon", "Meridian", "Equinox",
    "Solstice", "Eclipse", "Aurora", "Twilight", "Eventide",
    "Providence", "Destiny", "Fortune", "Serenity", "Tranquility",
    "Maelstrom", "Tempest", "Crucible", "Catalyst", "Synthesis",
    "Renaissance", "Apocalypse", "Ragnarok", "Armageddon", "Nirvana",
];

/// Adjectives for "The [Adjective] [Noun]" pattern
pub const ADJECTIVES: &[&str] = &[
    "Endless", "Verdant", "Crimson", "Azure", "Golden", "Shattered",
    "Frozen", "Burning", "Living", "Dying", "Awakening", "Sleeping",
    "Eternal", "Mortal", "Divine", "Profane", "Sacred", "Cursed",
    "Blessed", "Forgotten", "Remembered", "Lost", "Found", "Hidden",
    "Revealed", "Ancient", "Young", "Wise", "Foolish", "Noble",
    "Savage", "Civilized", "Wild", "Tamed", "Free", "Bound",
    "Infinite", "Finite", "Perfect", "Flawed", "Whole", "Broken",
];

/// Nouns for "The [Adjective] [Noun]" pattern
pub const NOUNS: &[&str] = &[
    "Realm", "Sphere", "Domain", "Expanse", "Frontier", "Horizon",
    "Sanctuary", "Crucible", "Tapestry", "Symphony", "Echo",
    "Garden", "Wasteland", "Paradise", "Purgatory", "Labyrinth",
    "Citadel", "Throne", "Crown", "Jewel", "Pearl", "Diamond",
    "Mirror", "Veil", "Gate", "Bridge", "Path", "Journey",
    "Dream", "Nightmare", "Vision", "Memory", "Promise", "Prophecy",
    "Song", "Dance", "Story", "Legend", "Myth", "Truth",
    "Light", "Shadow", "Dawn", "Dusk", "Star", "Void",
];