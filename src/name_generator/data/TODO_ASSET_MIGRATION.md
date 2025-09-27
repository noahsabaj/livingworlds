# Name Generator Data Asset Migration

## Current State
The name generator currently has ~100KB of hardcoded string data across 9 culture files:
- ancient.rs (13KB)
- desert.rs (11KB)
- eastern.rs (13KB)
- island.rs (14KB)
- mystical.rs (14KB)
- northern.rs (11KB)
- southern.rs (11KB)
- western.rs (12KB)

## Proposed Migration

### 1. Create Asset Files
Move all name data to `assets/names/` as JSON or TOML files:
```
assets/names/
├── cultures/
│   ├── western.json
│   ├── eastern.json
│   ├── northern.json
│   └── ...
├── geographical.json
└── world.json
```

### 2. Asset Structure Example
```json
{
  "culture": "western",
  "male_names": ["Alexander", "Marcus", ...],
  "female_names": ["Helena", "Victoria", ...],
  "neutral_names": ["Morgan", "Alex", ...],
  "surnames": ["Blackwood", "Whitehall", ...],
  "titles": ["Lord", "Duke", ...],
  "city_prefixes": ["New", "Old", ...],
  "city_suffixes": ["burg", "shire", ...]
}
```

### 3. Benefits
- Easier modification without recompilation
- Modding support - users can add custom name lists
- Reduced binary size
- Better separation of data and code
- Localization support

### 4. Implementation Requirements
- Add serde dependencies for JSON/TOML parsing
- Create NameDataAsset type with Bevy's asset system
- Convert NameGenerator to use loaded assets instead of constants
- Add asset loading state management
- Handle missing/corrupted asset files gracefully

### 5. Migration Steps
1. Create asset file structure
2. Export current constants to JSON/TOML
3. Implement asset loading system
4. Update NameGenerator to use assets
5. Add fallback for missing assets
6. Test all culture variations
7. Document modding format for custom names

## Estimated Effort
- 4-6 hours for full implementation
- Requires careful testing of all name generation paths
- Should be done as separate PR for clean review