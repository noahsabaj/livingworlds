//! Island culture names (Polynesian/Caribbean-inspired)
//!
//! Names inspired by Pacific Islander, Caribbean, and other island cultures.
//! Features seafaring peoples, tropical paradises, and archipelago nations.

/// Male first names
pub const MALE_NAMES: &[&str] = &[
    "Kai",
    "Koa",
    "Makoa",
    "Keanu",
    "Lono",
    "Maui",
    "Nalu",
    "Pono",
    "Akamu",
    "Hoku",
    "Ikaika",
    "Kalani",
    "Kekoa",
    "Leilani",
    "Malo",
    "Diego",
    "Carlos",
    "Miguel",
    "Rafael",
    "Santiago",
    "Fernando",
    "Javier",
    "Roberto",
    "Emilio",
    "Alejandro",
    "Luis",
    "Pedro",
    "Tane",
    "Rangi",
    "Aroha",
    "Hemi",
    "Wiremu",
    "Nikau",
    "Tama",
    "Mateo",
    "Enzo",
    "Luca",
    "Marco",
    "Paolo",
    "Sergio",
    "Vito",
    "Jean",
    "Pierre",
    "Claude",
    "Henri",
    "Jacques",
    "Michel",
    "Andre",
    "Tobias",
    "Willem",
    "Dirk",
    "Pieter",
    "Johannes",
    "Hendrik",
    "Adriaan",
];

/// Female first names
pub const FEMALE_NAMES: &[&str] = &[
    "Moana",
    "Leilani",
    "Kailani",
    "Nalani",
    "Mahina",
    "Hina",
    "Kai",
    "Aloha",
    "Kala",
    "Lilo",
    "Malia",
    "Nani",
    "Pua",
    "Tiare",
    "Ula",
    "Isabella",
    "Sofia",
    "Carmen",
    "Elena",
    "Lucia",
    "Maria",
    "Nina",
    "Rosa",
    "Teresa",
    "Valentina",
    "Esperanza",
    "Paloma",
    "Marisol",
    "Aroha",
    "Kiri",
    "Maia",
    "Hine",
    "Tui",
    "Wai",
    "Anahera",
    "Bella",
    "Chiara",
    "Giulia",
    "Luna",
    "Stella",
    "Viola",
    "Zara",
    "Amelie",
    "Celeste",
    "Delphine",
    "Fleur",
    "Genevieve",
    "Helene",
    "Annika",
    "Elsa",
    "Ilse",
    "Katja",
    "Lotte",
    "Marta",
    "Saskia",
];

/// Gender-neutral names
pub const NEUTRAL_NAMES: &[&str] = &[
    "Kai", "Lei", "Noa", "Koa", "Ari", "Rio", "Sol", "Mar", "Coral", "Ocean", "Wave", "Storm",
    "Rain",
];

/// Family/clan names
pub const CLAN_NAMES: &[&str] = &[
    "Kahana",
    "Kamaka",
    "Kauai",
    "Kealoha",
    "Mahalo",
    "Ohana",
    "Delgado",
    "Morales",
    "Rivera",
    "Santos",
    "Torres",
    "Vega",
    "Baptiste",
    "Beaumont",
    "Dubois",
    "Fontaine",
    "Laurent",
    "Moreau",
    "De Jong",
    "Van Der Berg",
    "Van Dijk",
    "Visser",
    "De Vries",
    "Bakker",
    "Ngata",
    "Taimana",
    "Wharepapa",
    "Parata",
    "Hohepa",
    "Tipene",
    "Coral",
    "Pearl",
    "Shell",
    "Reef",
    "Tide",
    "Wave",
    "Sunset",
    "Sunrise",
    "Moonlight",
    "Starfish",
    "Dolphin",
    "Turtle",
    "Palm",
    "Coconut",
    "Mango",
    "Papaya",
    "Hibiscus",
    "Plumeria",
];

/// Noble house/dynasty names (Polynesian chiefs, Caribbean dynasties, island kingdoms)
pub const HOUSE_NAMES: &[&str] = &[
    // Polynesian and Hawaiian houses
    "Kahana", "Kamaka", "Kauai", "Kealoha", "Mahalo", "Ohana",
    "Kamehameha", "Kalaniopuu", "Keoua", "Liholiho", "Kaahumanu",
    "Liliuokalani", "Kalakaua", "Lunalilo", "Kapiolani", "Kauikeaouli",

    // Caribbean and Spanish colonial houses
    "Delgado", "Morales", "Rivera", "Santos", "Torres", "Vega",
    "Conquistador", "Hidalgo", "Grandee", "Cacique", "Encomienda",
    "Hacienda", "Plantation", "Corsair", "Buccaneer", "Admiral",

    // French Antillean houses
    "Baptiste", "Beaumont", "Dubois", "Fontaine", "Laurent", "Moreau",
    "Martinique", "Guadeloupe", "Antilles", "Creole", "Maroon",

    // Dutch colonial houses
    "De Jong", "Van Der Berg", "Van Dijk", "Visser", "De Vries", "Bakker",
    "Nassau", "Orange", "Windward", "Leeward", "Curaçao",

    // Maori and Pacific houses
    "Ngata", "Taimana", "Wharepapa", "Parata", "Hohepa", "Tipene",
    "Ngati", "Iwi", "Hapu", "Tangata", "Whakapapa", "Mana",

    // Ocean and maritime themed
    "Coral", "Pearl", "Shell", "Reef", "Tide", "Wave",
    "Sunset", "Sunrise", "Moonlight", "Starfish", "Dolphin", "Turtle",
    "Seafarer", "Navigator", "Voyager", "Explorer", "Mariner", "Captain",

    // Tropical flora houses
    "Palm", "Coconut", "Mango", "Papaya", "Hibiscus", "Plumeria",
    "Banana", "Pineapple", "Sugar", "Rum", "Spice", "Coffee",

    // Island geographical features
    "Atoll", "Archipelago", "Cay", "Islet", "Lagoon", "Strait",
    "Channel", "Harbor", "Port", "Bay", "Cove", "Beach",

    // Weather and natural phenomena
    "Trade Winds", "Hurricane", "Typhoon", "Monsoon", "Tsunami",
    "Volcano", "Earthquake", "Geyser", "Hot Springs", "Tidal",

    // Pacific Island kingdoms
    "Tonga", "Samoa", "Fiji", "Tahiti", "Marquesas", "Cook",
    "Solomon", "Vanuatu", "New Caledonia", "Gilbert", "Marshall",

    // Mystical and spiritual houses
    "Sacred Grove", "Spirit", "Ancestor", "Totem", "Tabu",
    "Kapu", "Mana", "Aloha", "Pono", "Hale", "Makai",
];

/// Nation roots
pub const NATION_ROOTS: &[&str] = &[
    "Moana",
    "Tiki",
    "Samoa",
    "Tahiti",
    "Fiji",
    "Maui",
    "Trinidad",
    "Jamaica",
    "Bermuda",
    "Nassau",
    "Havana",
    "Barbados",
    "Antigua",
    "Grenada",
    "Martinique",
    "Dominica",
    "Bali",
    "Java",
    "Sumatra",
    "Borneo",
    "Sulawesi",
    "Mindanao",
    "Luzon",
    "Visayas",
    "Palawan",
    "Cebu",
    "Leyte",
    "Samar",
    "Madagascar",
    "Mauritius",
    "Seychelles",
    "Comoros",
    "Reunion",
    "Malta",
    "Cyprus",
    "Crete",
    "Rhodes",
    "Sicily",
    "Sardinia",
    "Corsica",
    "Balearic",
    "Canary",
    "Azores",
    "Madeira",
    "Cape Verde",
];

/// Place names
pub const PLACE_NAMES: &[&str] = &[
    "Coral Bay",
    "Pearl Harbor",
    "Turtle Beach",
    "Paradise Cove",
    "Sunset Shores",
    "Palm Grove",
    "Coconut Island",
    "Mermaid Lagoon",
    "Crystal Waters",
    "Blue Lagoon",
    "Emerald Cove",
    "Sapphire Bay",
    "Rainbow Beach",
    "Moonlight Bay",
    "Starfish Point",
    "Dolphin Cove",
    "Volcano Peak",
    "Lava Fields",
    "Hot Springs",
    "Geyser Valley",
    "Mangrove Swamp",
    "Tidal Pools",
    "Reef Gardens",
    "Kelp Forest",
    "Trade Winds Port",
    "Hurricane Harbor",
    "Typhoon Bay",
    "Monsoon Market",
    "Sugar Plantation",
    "Rum Distillery",
    "Spice Gardens",
    "Coffee Hills",
    "Pineapple Fields",
    "Banana Grove",
    "Mango Orchard",
    "Papaya Plantation",
    "Fishing Village",
    "Whaling Station",
    "Pearl Divers",
    "Salt Flats",
    "Sacred Grove",
    "Temple Island",
    "Ancestor Bay",
    "Spirit Beach",
];

// ========================================================================
// COMPOUND PATTERN SYSTEM DATA - For multi-variable name generation
// ========================================================================

/// Adjectives for compound patterns (Island theme: Maritime, Tropical, Volcanic, Seafaring)
pub const ISLAND_ADJECTIVES: &[&str] = &[
    // Maritime/ocean adjectives
    "Maritime", "Ocean", "Sea", "Tidal", "Coral", "Pearl", "Wave", "Deep",
    "Blue", "Turquoise", "Azure", "Sapphire", "Emerald", "Crystal", "Clear", "Sacred",

    // Tropical/paradise adjectives
    "Tropical", "Paradise", "Golden", "Sun", "Sunset", "Sunrise", "Palm", "Coconut",
    "Hibiscus", "Plumeria", "Mango", "Papaya", "Banana", "Pineapple", "Sugar", "Rum",

    // Seafaring/navigation adjectives
    "Seafaring", "Navigator", "Voyager", "Explorer", "Mariner", "Captain", "Admiral", "Trade",
    "Wind", "Storm", "Hurricane", "Typhoon", "Monsoon", "Sailing", "Anchored", "Drifting",

    // Volcanic/geological adjectives
    "Volcanic", "Lava", "Fire", "Magma", "Ash", "Crater", "Hot", "Steam",
    "Geyser", "Thermal", "Mineral", "Rock", "Stone", "Cliff", "Peak", "Ridge",

    // Island geography adjectives
    "Archipelago", "Atoll", "Lagoon", "Reef", "Cove", "Bay", "Harbor", "Port",
    "Channel", "Strait", "Passage", "Inlet", "Beach", "Shore", "Coast", "Tidal",

    // Cultural/spiritual adjectives
    "Sacred", "Ancient", "Noble", "Royal", "Chief", "Warrior", "Peaceful", "Blessed",
    "Ancestral", "Spirit", "Mana", "Tabu", "Kapu", "Aloha", "Pono", "Ohana",
];

/// Political structures for compound patterns
pub const POLITICAL_STRUCTURES: &[&str] = &[
    // Island political structures
    "Islands", "Archipelago", "Kingdom", "Confederation", "Federation", "Union", "Alliance", "League",
    "Empire", "Republic", "Chiefdom", "Principality", "Duchy", "Realm", "Domain", "Territory",

    // Maritime structures
    "Maritime Republic", "Ocean Empire", "Sea Kingdom", "Island Federation", "Coastal Alliance",
    "Naval Confederation", "Harbor League", "Port Union", "Fleet Empire", "Admiralty",

    // Polynesian/tribal structures
    "Clan", "Tribe", "Ohana", "Family", "House", "Dynasty", "Bloodline", "Lineage",
    "Chiefs", "Warriors", "Navigators", "Voyagers", "Council", "Assembly", "Circle",

    // Trade/economic structures
    "Trading Company", "Merchant Republic", "Commercial Federation", "Spice Alliance", "Sugar Empire",
    "Rum Confederation", "Pearl League", "Coral Union", "Plantation States", "Trade Empire",
];

/// Geographic modifiers for compound patterns
pub const GEOGRAPHIC_MODIFIERS: &[&str] = &[
    // Directional/positional
    "Northern", "Southern", "Eastern", "Western", "Central", "Upper", "Lower", "Outer", "Inner",
    "Far", "Near", "Deep", "High", "Great", "Lesser", "Hidden", "Lost", "Sacred", "Ancient",

    // Marine/aquatic terrain
    "Ocean", "Sea", "Reef", "Coral", "Lagoon", "Atoll", "Bay", "Cove", "Harbor", "Port",
    "Channel", "Strait", "Passage", "Inlet", "Beach", "Shore", "Coast", "Tidal", "Deep", "Shallow",

    // Tropical/climate modifiers
    "Tropical", "Paradise", "Golden", "Sunset", "Sunrise", "Trade Wind", "Storm", "Hurricane",
    "Typhoon", "Monsoon", "Rain", "Sun", "Hot", "Warm", "Cool", "Breezy", "Calm", "Peaceful",

    // Volcanic/geological
    "Volcanic", "Fire", "Lava", "Ash", "Steam", "Hot", "Thermal", "Mineral", "Rock", "Stone",
    "Cliff", "Peak", "Ridge", "Valley", "Crater", "Geyser", "Spring", "Cave", "Grotto", "Mesa",
];

/// Simple patterns for basic generation (expanded from original 8)
pub const SIMPLE_NATION_PATTERNS: &[&str] = &[
    // Classical patterns (original)
    "{} Islands", "Kingdom of {}", "{} Confederation", "United Isles of {}",
    "{} Archipelago", "{} Island Federation", "Maritime Republic of {}", "{} Ocean Empire",

    // NEW: Maritime/ocean variants
    "Maritime {} Islands", "Ocean Kingdom of {}", "Sea {} Confederation", "Naval {} Federation",
    "Deep {} Empire", "Coral {} Republic", "Pearl {} Alliance", "Tidal {} Union",

    // NEW: Tropical/paradise variants
    "Tropical {} Islands", "Paradise Kingdom of {}", "Golden {} Archipelago", "Sun {} Federation",
    "Palm {} Republic", "Coconut {} Empire", "Sunset {} Alliance", "Paradise {} Union",

    // NEW: Seafaring/navigation variants
    "Seafaring {} Islands", "Navigator Kingdom of {}", "Voyager {} Confederation", "Explorer {} Federation",
    "Mariner {} Republic", "Captain {} Empire", "Admiral {} Alliance", "Trade {} Union",

    // NEW: Volcanic/geological variants
    "Volcanic {} Islands", "Fire Kingdom of {}", "Lava {} Archipelago", "Crater {} Federation",
    "Ash {} Republic", "Steam {} Empire", "Hot Springs {} Alliance", "Geyser {} Union",
];

/// Weighted patterns for realistic distribution (pattern, weight)
pub const WEIGHTED_NATION_PATTERNS: &[(&str, u32)] = &[
    // Common patterns (weight 3)
    ("{} Islands", 3), ("Kingdom of {}", 3), ("{} Archipelago", 3), ("{} Confederation", 3),

    // Uncommon patterns (weight 2)
    ("{} Island Federation", 2), ("Maritime Republic of {}", 2), ("Tropical {} Islands", 2), ("Ocean Kingdom of {}", 2),

    // Rare patterns (weight 1)
    ("{} Ocean Empire", 1), ("United Isles of {}", 1), ("Paradise Kingdom of {}", 1), ("Volcanic {} Islands", 1),
];

/// Simple house patterns (expanded)
pub const SIMPLE_HOUSE_PATTERNS: &[&str] = &[
    // Classical patterns (original)
    "{} Clan", "House {}", "{} Family", "The {} Dynasty",
    "{} Bloodline", "Ohana {}", "{} Chiefs", "House of {}",

    // NEW: Polynesian/tribal variants
    "Noble {} Clan", "Chief {} Family", "Warrior {} House", "Ancient {} Dynasty",
    "Sacred {} Bloodline", "Royal {} Ohana", "Mighty {} Chiefs", "Great {} Tribe",

    // NEW: Maritime/seafaring variants
    "Seafaring {} Clan", "Navigator {} Family", "Voyager {} House", "Mariner {} Dynasty",
    "Captain {} Bloodline", "Admiral {} Family", "Explorer {} Clan", "Sailor {} House",

    // NEW: Island/tropical variants
    "Island {} Clan", "Tropical {} Family", "Palm {} House", "Coral {} Dynasty",
    "Pearl {} Bloodline", "Wave {} Family", "Tide {} Clan", "Ocean {} House",
];

/// Weighted house patterns
pub const WEIGHTED_HOUSE_PATTERNS: &[(&str, u32)] = &[
    // Common patterns (weight 3)
    ("{} Clan", 3), ("House {}", 3), ("{} Family", 3), ("{} Dynasty", 3),

    // Uncommon patterns (weight 2)
    ("Ohana {}", 2), ("{} Chiefs", 2), ("Noble {} Clan", 2), ("Chief {} Family", 2),

    // Rare patterns (weight 1)
    ("{} Bloodline", 1), ("Sacred {} Bloodline", 1), ("Seafaring {} Clan", 1), ("Royal {} Ohana", 1),
];
