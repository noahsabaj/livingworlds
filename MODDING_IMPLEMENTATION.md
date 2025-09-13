# Living Worlds Modding System Implementation Plan

## Overview
Transform Living Worlds from a completely hardcoded game (ZERO moddability) into a highly moddable platform with externalized configuration, mod loading, hot-reload, and Steam Workshop support.

**Goal**: Make "highly moddable" claim TRUE by Christmas 2025  
**Timeline**: 3 weeks total  
**Current Status**: Phase 1 COMPLETED ✅ | Ready for Phase 2

---

## Phase 1: Foundation (Week 1) ✅ COMPLETED

### ✅ Step 1: Configuration Externalization
- [x] Create base configuration directory structure (`config/base/`)
- [x] Create `balance.ron` with all gameplay constants (200+ values)
- [x] Create `colors.ron` with all color definitions
- [x] Create `generation.ron` with world generation parameters
- [x] Create `simulation.ron` with population/tension calculations
- [x] Create mods directory structure (`mods/`, `mods/workshop/`)

### ✅ Step 2: Mod System Core
- [x] Create `src/modding/types.rs` with configuration structures
- [x] Define `ModManifest` structure for mod metadata
- [x] Define `GameConfig` resource for merged configuration
- [x] Define `ModConfigOverrides` for partial config replacement
- [x] Create `src/modding/manager.rs` with `ModManager` resource
- [x] Implement mod discovery from filesystem
- [x] Implement mod manifest loading
- [x] Implement config merging system
- [x] Implement dependency checking

### ✅ Step 3: Hot-Reload Support
- [x] Create `src/modding/loader.rs` with file watching
- [x] Add `notify` crate for filesystem events
- [x] Add `crossbeam-channel` for thread-safe communication
- [x] Implement `ConfigWatcher` resource
- [x] Implement `ConfigReloadEvent` system
- [x] Create config reload handlers

### ✅ Step 4: Integration
- [x] Create `src/modding/mod.rs` main module
- [x] Create `ModdingPlugin` for Bevy
- [x] Add modding module to `src/lib.rs`
- [x] Register `ModdingPlugin` in app setup
- [x] Fix compilation errors in modding system
- [x] Test mod discovery with example mod
- [x] Test hot-reload functionality
- [x] Verify config merging works correctly

---

## Phase 2: Data Migration (Week 1-2) ⬜ NOT STARTED

### ⬜ Step 5: Replace Constants Usage
- [ ] Update `src/constants.rs` to read from `GameConfig`
- [ ] Replace `HEX_SIZE_PIXELS` with `config.balance.world.hex_size_pixels`
- [ ] Replace `PROVINCES_PER_ROW` with config value
- [ ] Replace `PROVINCES_PER_COL` with config value
- [ ] Replace all camera constants with config values
- [ ] Replace all UI constants with config values
- [ ] Replace all simulation constants with config values
- [ ] Replace all cloud constants with config values
- [ ] Replace all generation constants with config values
- [ ] Update all 500+ constant references throughout codebase

### ⬜ Step 6: Replace Color Functions
- [ ] Update `src/colors.rs` to read from `GameConfig`
- [ ] Replace hardcoded terrain colors with config lookups
- [ ] Replace hardcoded mineral colors with config lookups
- [ ] Replace hardcoded UI colors with config lookups
- [ ] Create color interpolation for gradients
- [ ] Support named color references
- [ ] Update all color function calls

### ⬜ Step 7: Update Generation Systems
- [ ] Update `src/generation/mod.rs` to use config
- [ ] Update `src/generation/tectonics.rs` with config values
- [ ] Update `src/generation/provinces.rs` with config values
- [ ] Update `src/generation/rivers.rs` with config values
- [ ] Update `src/generation/agriculture.rs` with config values
- [ ] Update `src/generation/clouds.rs` with config values
- [ ] Make continent generation data-driven
- [ ] Make elevation thresholds configurable

### ⬜ Step 8: Update Simulation Systems
- [ ] Update `src/simulation.rs` to use config
- [ ] Make population growth configurable
- [ ] Make world tension calculation configurable
- [ ] Make time speeds configurable
- [ ] Update agriculture bonuses from config
- [ ] Update terrain multipliers from config
---

## Phase 3: Mod Types (Week 2) ⬜ NOT STARTED

### ⬜ Step 10: Example Mods
- [ ] Create `mods/faster_gameplay/` example mod
  - [ ] Faster time speeds
  - [ ] Higher population growth
  - [ ] More starting resources
- [ ] Create `mods/harsh_world/` example mod
  - [ ] Lower population growth
  - [ ] Higher war tension
  - [ ] Resource scarcity
- [ ] Create `mods/colorblind_friendly/` example mod
  - [ ] Alternative color schemes
  - [ ] Higher contrast
  - [ ] Pattern overlays
- [ ] Create `mods/peaceful_observer/` example mod
  - [ ] No wars
  - [ ] Focus on trade/culture
  - [ ] Slower pace
- [ ] Create `mods/mega_continents/` example mod
  - [ ] Larger landmasses
  - [ ] Fewer oceans
  - [ ] Different world generation

### ⬜ Step 11: Mod Validation
- [ ] Implement version compatibility checking
- [ ] Implement dependency resolution
- [ ] Implement conflict detection
- [ ] Create validation error messages
- [ ] Implement fallback to vanilla on error
- [ ] Add mod load order resolution

---

## Phase 4: User Interface (Week 2-3) 🟨 IN PROGRESS

### ✅ Step 12: Mod Browser UI
- [x] Create `src/modding/ui.rs` module
- [x] Replace "Credits" with "Mods" button in main menu
- [x] Create mod browser screen with tabs (Installed, Workshop, Active Modset)
- [x] Display available mods in 3-column grid
- [x] Show mod name, version, author
- [x] Show mod description placeholder
- [x] Show mod thumbnail placeholder
- [x] Add search bar UI (functionality pending)
- [x] Add filter sidebar with categories
- [x] Add sort options UI (Most Popular, Recent, etc.)

### 🟨 Step 13: Mod Management UI
- [x] Add enable/disable checkboxes with auto-enable option
- [x] Add load order display with drag handles (drag functionality pending)
- [x] Show mod version information
- [x] Show status indicators (✓ for compatible)
- [x] Add "CONFIRM MODSET" button with soft-reset
- [x] Add "Back to Game" button
- [x] Show active mod count
- [x] Display estimated load time

### ⬜ Step 14: Settings Integration
- [ ] Add "Mods" tab to settings menu
- [ ] Show per-mod configuration options
- [ ] Allow tweaking mod parameters
- [ ] Add mod profiles (save/load mod configurations)
- [ ] Add "Export Mod List" feature
- [ ] Add "Import Mod List" feature

### 🟨 Step 15: Notifications
- [x] Integrated with loading screen for mod application
- [x] Added ApplyingMods loading operation
- [ ] Show mod loaded notifications
- [ ] Show mod error notifications
- [ ] Show hot-reload notifications
- [ ] Show Workshop download progress
- [ ] Show dependency missing warnings

---

## Phase 5: Steam Workshop (Week 3) ⬜ NOT STARTED

### ⬜ Step 16: Workshop Upload
- [ ] Implement `src/modding/workshop.rs`
- [ ] Create mod packaging system
- [ ] Generate workshop item metadata
- [ ] Implement thumbnail generation
- [ ] Add upload progress tracking
- [ ] Handle upload errors
- [ ] Support mod updates
- [ ] Add version tagging

### ⬜ Step 17: Workshop Download
- [ ] Implement workshop browsing
- [ ] Add subscribe/unsubscribe
- [ ] Implement auto-download
- [ ] Handle download progress
- [ ] Extract to workshop directory
- [ ] Validate downloaded mods
- [ ] Auto-update subscribed mods

### ⬜ Step 18: Workshop UI
- [ ] Add "Workshop" button to mod browser
- [ ] Show popular mods
- [ ] Show recent mods
- [ ] Show subscribed mods
- [ ] Display ratings
- [ ] Display download count
- [ ] Add rating system
- [ ] Add reporting system

### ⬜ Step 19: Workshop Integration
- [ ] Link with Steam API
- [ ] Handle authentication
- [ ] Implement ISteamUGC interface
- [ ] Handle workshop queries
- [ ] Cache workshop metadata
- [ ] Sync subscriptions

---

## Phase 6: Documentation (Ongoing) ⬜ NOT STARTED

### ⬜ Step 20: Modding Guide
- [ ] Create `MODDING_GUIDE.md`
- [ ] Write "Your First Mod" tutorial
- [ ] Document RON format
- [ ] List all moddable parameters
- [ ] Provide balance guidelines
- [ ] Add troubleshooting section
- [ ] Create FAQ

### ⬜ Step 21: API Documentation
- [ ] Document `ModManifest` format
- [ ] Document configuration structures
- [ ] Document color system
- [ ] Document generation parameters
- [ ] Document simulation rules
- [ ] Create examples for each section

### ⬜ Step 22: Example Templates
- [ ] Create balance mod template
- [ ] Create visual mod template
- [ ] Create world generation template
- [ ] Create total conversion template
- [ ] Add inline comments
- [ ] Include best practices

---

## Testing Checklist ⬜ NOT STARTED

### ⬜ Basic Functionality
- [ ] Mod discovery works
- [ ] Mod loading works
- [ ] Config merging works
- [ ] Hot-reload works
- [ ] Vanilla fallback works

### ⬜ User Interface
- [ ] Mod browser displays correctly
- [ ] Enable/disable works
- [ ] Load order changes work
- [ ] Settings integration works
- [ ] Notifications appear

### ⬜ Steam Workshop
- [ ] Upload succeeds
- [ ] Download succeeds
- [ ] Subscriptions sync
- [ ] Auto-update works
- [ ] Ratings work

### ⬜ Performance
- [ ] No performance regression
- [ ] Config lookups are fast
- [ ] Hot-reload is responsive
- [ ] Memory usage acceptable
- [ ] Startup time acceptable

### ⬜ Edge Cases
- [ ] Invalid mod handling
- [ ] Missing dependencies
- [ ] Version conflicts
- [ ] Corrupt downloads
- [ ] Network failures

---

## Success Metrics

### ✅ Phase 1 Success Criteria (COMPLETED)
- [x] Configuration files created
- [x] Mod system compiles
- [x] Example mod loads
- [x] Hot-reload triggers

### ⬜ Phase 2 Success Criteria
- [ ] All constants externalized
- [ ] All colors externalized
- [ ] Generation uses config
- [ ] Simulation uses config

### ⬜ Phase 3 Success Criteria
- [ ] 5+ example mods created
- [ ] Each mod type works
- [ ] Validation prevents crashes
- [ ] Conflicts detected

### 🟨 Phase 4 Success Criteria
- [x] UI is intuitive (three-tab design implemented)
- [x] Core features accessible (mod browser opens from menu)
- [ ] Settings integrated (pending Phase 2 completion)
- [x] Loading screen integration complete

### ⬜ Phase 5 Success Criteria
- [ ] Workshop upload works
- [ ] Workshop download works
- [ ] Subscriptions sync
- [ ] Ratings functional

### ⬜ Phase 6 Success Criteria
- [ ] Guide is comprehensive
- [ ] Templates are useful
- [ ] Documentation complete
- [ ] Examples work

---

## Current Blockers

✅ **Phase 1 Complete - No Blockers**

Previous issues resolved:
- Compilation errors ✅ FIXED (added bevy_log feature)
- Thread safety ✅ FIXED (used crossbeam-channel)
- Borrowing conflicts ✅ FIXED (restructured code)
- Config loading ✅ WORKING
- Mod discovery ✅ WORKING
- Hot-reload ✅ IMPLEMENTED

---

## Next Immediate Steps (Phase 2)

1. [ ] Replace hardcoded constants in `src/constants.rs` with config values
2. [ ] Update all systems to read from `GameConfig` resource
3. [ ] Replace color functions with config lookups
4. [ ] Make generation systems data-driven
5. [ ] Update simulation to use config values
6. [ ] Create more example mods for testing

---

## Implementation Details (Added 2025-09-11)

### Mod Browser UI Architecture
The mod browser has been implemented with a comprehensive Steam Workshop-integrated design:

#### Core Components
- **`src/modding/ui.rs`**: Main UI module with full mod browser implementation
- **Three-tab interface**: Installed, Workshop, Active Modset
- **Grid layout**: 3-column mod display with cards
- **Sidebar filters**: Categories, sort options, time periods
- **Bottom action bar**: Back to Game, Refresh, CONFIRM MODSET

#### Key Features Implemented
1. **Direct Workshop Integration** (UI ready, API pending):
   - Mod cards with thumbnails, ratings, download counts
   - Auto-enable checkbox for convenience
   - Live data placeholders for ratings/comments

2. **Active Modset Management** (HOI4-style):
   - List view with toggle switches
   - Drag handles for load order (visual only)
   - Status indicators and version display
   - Conflict detection placeholders

3. **Soft-Reset System**:
   - CONFIRM MODSET triggers loading screen
   - ApplyingMods operation added to LoadingOperation enum
   - Seamless transition without full game restart
   - Loading screen shows "Applying mod configuration..."

4. **Menu Integration**:
   - Credits button replaced with Mods button
   - Opens mod browser via OpenModBrowserEvent
   - Full-screen overlay with proper z-indexing

#### Pending Steam Workshop Features
- Actual Workshop API calls (requires Steam SDK integration)
- Live mod browsing and downloading
- Image slideshow functionality
- Comments and ratings display
- Subscription management

---

## Notes

- **Steam SDK**: Located at `/home/nsabaj/Code/sdk`, properly linked via `build.rs`
- **Bevy Version**: Using 0.16.1, requires specific patterns for UI and ECS
- **Performance**: Must maintain 60+ FPS with 900k provinces
- **Backwards Compatibility**: Must support existing save files
- **Critical Path**: Phase 1 → Phase 2 → Phase 4 minimum for "moddable" claim

---

Last Updated: 2025-09-11
Status: Phase 1 COMPLETED ✅ | Phase 4 IN PROGRESS 🟨 - UI Implementation underway