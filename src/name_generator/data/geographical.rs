//! Geographic feature name components
//!
//! Names for natural features like rivers, mountains, oceans, deserts, and forests.
//! These create names like "Silver River" or "Thunder Peak"

/// River name roots
pub const RIVER_ROOTS: &[&str] = &[
    "Silver", "Golden", "Crystal", "Serpent", "Dragon", "Swift",
    "Lazy", "Mighty", "Ancient", "Young", "Wild", "Calm",
    "Rushing", "Whispering", "Singing", "Dancing", "Weeping", "Laughing",
    "Red", "Blue", "Green", "Black", "White", "Grey",
    "Twin", "Triple", "Forked", "Winding", "Straight", "Crooked",
    "Deep", "Shallow", "Wide", "Narrow", "Long", "Short",
    "Sacred", "Cursed", "Blessed", "Holy", "Dark", "Light",
];

/// Mountain name roots
pub const MOUNTAIN_ROOTS: &[&str] = &[
    "Thunder", "Storm", "Cloud", "Sky", "Eagle", "Dragon",
    "Giant", "Titan", "Gods", "Eternal", "Lonely", "Twin",
    "Iron", "Gold", "Silver", "Crystal", "Diamond", "Stone",
    "Fire", "Ice", "Snow", "Frost", "Wind", "Sun",
    "Shadow", "Light", "Dark", "Grey", "White", "Black",
    "Ancient", "Young", "Broken", "Whole", "Sharp", "Blunt",
    "Sacred", "Cursed", "Blessed", "Forbidden", "Hidden", "Lost",
    "King's", "Queen's", "Emperor's", "God's", "Devil's", "Demon's",
];

/// Ocean name roots
pub const OCEAN_ROOTS: &[&str] = &[
    "Endless", "Sapphire", "Emerald", "Crimson", "Azure", "Dark",
    "Peaceful", "Raging", "Silent", "Singing", "Frozen", "Boiling",
    "Northern", "Southern", "Eastern", "Western", "Central", "Outer",
    "Inner", "Great", "Lesser", "Vast", "Narrow", "Wide",
    "Deep", "Shallow", "Warm", "Cold", "Tempest", "Calm",
    "Mist", "Fog", "Clear", "Murky", "Crystal", "Jade",
    "Forgotten", "Lost", "Found", "Hidden", "Revealed", "Secret",
    "Ancient", "Young", "Eternal", "Dying", "Living", "Dead",
];

/// Desert name roots
pub const DESERT_ROOTS: &[&str] = &[
    "Scorching", "Endless", "Golden", "Crimson", "Shifting", "Silent",
    "Whispering", "Forgotten", "Cursed", "Blessed", "Ancient", "Young",
    "Great", "Lesser", "Vast", "Small", "Northern", "Southern",
    "Eastern", "Western", "Central", "Outer", "Inner", "High",
    "Low", "Burning", "Blazing", "Scorched", "Baked", "Dried",
    "Dead", "Living", "Moving", "Still", "Singing", "Screaming",
    "Red", "Yellow", "White", "Black", "Grey", "Brown",
    "Crystal", "Glass", "Salt", "Bone", "Ash", "Dust",
];

/// Forest name roots
pub const FOREST_ROOTS: &[&str] = &[
    "Dark", "Light", "Ancient", "Young", "Sacred", "Cursed",
    "Whispering", "Silent", "Singing", "Dancing", "Sleeping", "Waking",
    "Green", "Golden", "Silver", "Black", "White", "Grey",
    "Deep", "Shallow", "Thick", "Thin", "Dense", "Sparse",
    "Wild", "Tame", "Mad", "Sane", "Living", "Dead",
    "Enchanted", "Haunted", "Blessed", "Forbidden", "Hidden", "Lost",
    "Elder", "Young", "Eternal", "Dying", "Growing", "Shrinking",
    "Northern", "Southern", "Eastern", "Western", "Central", "Outer",
    "Misty", "Clear", "Foggy", "Bright", "Shadow", "Twilight",
];

/// Suffixes for geographic features
pub const GEOGRAPHIC_SUFFIXES: &[&str] = &[
    // River suffixes
    " River", " Stream", " Creek", " Rapids", " Falls", " Fork",
    " Tributary", " Delta", " Confluence", " Bend", " Crossing",

    // Mountain suffixes
    " Peak", " Mountain", " Ridge", " Summit", " Spire", " Crag",
    " Cliff", " Bluff", " Heights", " Range", " Massif", " Plateau",

    // Ocean suffixes
    " Ocean", " Sea", " Waters", " Depths", " Abyss", " Gulf",
    " Bay", " Strait", " Channel", " Passage", " Current",

    // Desert suffixes
    " Desert", " Wastes", " Sands", " Barrens", " Badlands", " Flats",
    " Dunes", " Expanse", " Desolation", " Emptiness",

    // Forest suffixes
    " Forest", " Woods", " Grove", " Wildwood", " Timber", " Thicket",
    " Jungle", " Rainforest", " Taiga", " Copse", " Glade",
];