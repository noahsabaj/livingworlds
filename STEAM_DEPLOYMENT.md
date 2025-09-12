# Steam Deployment Guide for Living Worlds

## Overview

Living Worlds is distributed exclusively on Steam, leveraging Steam's features for achievements, cloud saves, workshop support, and rich presence. This guide covers the complete process from development to release.

## 📋 Prerequisites

1. **Steam Partner Account** - Register at [partner.steamgames.com](https://partner.steamgames.com)
2. **App ID** - Obtain from Valve after approval (currently using 480 for testing)
3. **Steamworks SDK** - Automatically bundled with bevy_steamworks
4. **Steam Client** - Must be running for development/testing

## 🏗️ Build Configuration

### Development Build (Without Steam)
```bash
cargo run --release
```

### Steam Build
```bash
cargo run --release --features steam
```

### Production Build
```bash
cargo build --release --features steam
```

## 🎮 Steam Features Implementation

### 1. Achievements (16 Total)

#### Observer Achievements
- **First World** - Generate your first world
- **Observer Novice** - Watch for 1 hour
- **Observer Veteran** - Watch for 10 hours  
- **Observer Master** - Watch for 100 hours

#### Gameplay Achievements
- **Witness War** - See your first war
- **Witness Peace** - See 100 years of continuous peace
- **World Explorer** - Generate 10 different worlds
- **Large World** - Generate a large world (900k provinces)
- **Millennium** - Simulate 1000 years in a single world
- **Population Boom** - Reach 1 billion world population
- **Rise and Fall** - Witness a nation's complete lifecycle
- **Golden Age** - Maintain low tension for 500 years
- **Apocalypse** - Reach maximum world tension
- **Speed Demon** - Use fastest simulation speed
- **Photographer** - Take 100 screenshots
- **Modder** - Subscribe to a workshop item

### 2. Cloud Saves

Cloud saves are automatic with Steam Cloud. Configure in `app_build_config.vdf`:

```vdf
"DepotBuildConfig"
{
    "DepotID" "YOUR_DEPOT_ID"
    "ContentRoot" "."
    "FileMapping"
    {
        "LocalPath" "saves/*.lws"
        "DepotPath" "saves/"
        "recursive" "1"
    }
    "FileExclusion" "*.tmp"
}
```

Steam Cloud quotas (configure in Steamworks):
- **Quota**: 100MB per user
- **File Pattern**: `*.lws` (compressed save files ~10MB each)
- **Max Files**: 20 saves per user

### 3. Rich Presence

Shows what players are doing in their friends list:
- "In Main Menu"
- "Configuring New World"
- "Observing Year 1453 - Large World"
- "Paused - Medium World"

### 4. Workshop Support

Planned mod types:
- **World Presets** - Custom generation parameters
- **Color Schemes** - Custom terrain/nation colors
- **Music Packs** - Additional procedural parameters
- **Balance Mods** - Modified simulation values

### 5. Statistics Tracking

Tracked stats (for achievements and leaderboards):
- Total playtime (minutes)
- Worlds generated
- Provinces explored
- Years simulated
- Nations witnessed
- Wars observed
- Peak world population

## 📦 Deployment Process

### Step 1: Update App ID

Edit `src/steam.rs`:
```rust
const STEAM_APP_ID: u32 = YOUR_ACTUAL_APP_ID; // Replace 480
```

### Step 2: Build Release Version

```bash
# Clean build
cargo clean

# Build with Steam features
cargo build --release --features steam

# Output location: target/release/living-worlds
```

### Step 3: Prepare Steam Depot

Create directory structure:
```
steam_build/
├── living-worlds           # Main executable
├── steam_appid.txt         # Contains your App ID
└── saves/                  # Empty directory for cloud saves
```

`steam_appid.txt` content:
```
YOUR_APP_ID
```

### Step 4: Configure Steamworks

In Steamworks Partner site:

1. **Application Settings**
   - Name: Living Worlds
   - Type: Game
   - Supported OS: Windows, Linux
   
2. **Cloud Settings**
   - Enable Steam Cloud
   - Quota: 100MB
   - File sync on exit: Yes
   
3. **Achievements**
   - Add all 16 achievements with icons
   - Set progress tracking for cumulative achievements
   
4. **Store Page**
   - Price: $9.99
   - Release date: December 25, 2025
   - Tags: Simulation, Strategy, Sandbox, Moddable

### Step 5: Upload to Steam

Using SteamPipe (Valve's content delivery):

1. Create `app_build.vdf`:
```vdf
"AppBuild"
{
    "AppID" "YOUR_APP_ID"
    "Desc" "Living Worlds v0.1.0"
    "BuildOutput" "./output/"
    "ContentRoot" "./steam_build/"
    "SetLive" "default"
    "Depots"
    {
        "YOUR_DEPOT_ID"
        {
            "FileMapping"
            {
                "LocalPath" "*"
                "DepotPath" "."
                "recursive" "1"
            }
        }
    }
}
```

2. Upload using SteamCMD:
```bash
steamcmd +login YOUR_USERNAME +run_app_build ../app_build.vdf +quit
```

## 🧪 Testing

### Local Testing

1. Create `steam_appid.txt` in game directory with `480` (Spacewar test ID)
2. Run Steam client
3. Build with `--features steam`
4. Test achievements, stats, rich presence

### Beta Testing

1. Create beta branch in Steamworks
2. Provide beta keys to testers
3. Monitor crash reports via Steam
4. Gather feedback through Steam forums

### Pre-Release Checklist

- [ ] All achievements unlock correctly
- [ ] Cloud saves sync properly
- [ ] Rich presence displays accurately
- [ ] Statistics track correctly
- [ ] Game launches from Steam library
- [ ] Steam overlay works (Shift+Tab)
- [ ] Screenshots work (F12)
- [ ] Trading cards configured (optional)
- [ ] Workshop uploads work
- [ ] Leaderboards display

## 🚀 Launch Day

1. **Set Release Time**: 10 AM PST (peak Steam traffic)
2. **Update Store Page**: Add launch trailer
3. **Announce**: Steam announcements, social media
4. **Monitor**: Watch for day-1 issues
5. **Respond**: Quick patches for critical bugs

## 📊 Post-Launch

### Analytics to Monitor
- Daily Active Users (DAU)
- Average session length
- Achievement completion rates
- Workshop subscription rates
- Refund rate (target <5%)
- User reviews

### Common Issues & Solutions

**Issue**: "Steam must be running"
**Solution**: Ensure Steam client is running before launching game

**Issue**: Achievements not unlocking
**Solution**: Check `user_stats.store_stats()` is called after setting

**Issue**: Cloud saves not syncing
**Solution**: Verify file permissions and Steam Cloud quota

**Issue**: Workshop items not downloading
**Solution**: Check subscription status and available disk space

## 🔧 Maintenance

### Regular Updates
- Bug fixes: Weekly patches as needed
- Content updates: Monthly
- Major features: Quarterly

### Version Management
```toml
[package]
version = "0.1.0"  # Increment for each Steam update
```

### Steam Build IDs
Track build IDs for rollback capability:
- v0.1.0 - Build 1234567
- v0.1.1 - Build 1234568
- etc.

## 📝 Legal Requirements

- [ ] EULA in game and on store page
- [ ] Privacy policy (if collecting any data)
- [ ] Age rating (likely E for Everyone)
- [ ] Export compliance (for encryption)

## 🎯 Marketing Integration

### Steam Features to Leverage
- **Wishlist Campaign**: Start 3 months before launch
- **Demo**: Consider free demo for Steam Next Fest
- **Trading Cards**: Optional monetization
- **Curator Connect**: Send keys to relevant curators
- **Steam Sales**: Participate in seasonal sales

### Review Strategy
- Encourage reviews at ~2 hours playtime
- Respond to negative reviews professionally
- Update store page based on feedback

## 💰 Revenue Considerations

- **Base Price**: $9.99
- **Launch Discount**: 10% first week
- **Regional Pricing**: Use Steam's recommended prices
- **DLC Strategy**: Future expansion packs possible
- **Revenue Split**: 70/30 (Valve/Developer) standard

## 🛠️ Support

### Steam Support Integration
- Monitor Steam forums daily
- Respond to support tickets within 24 hours
- Maintain FAQ on discussions page
- Post regular development updates

### Crash Reporting
Steam automatically collects crash dumps. Access via:
1. Steamworks Partner → Your Game → Support → Crash Dumps

## 📚 Additional Resources

- [Steamworks Documentation](https://partner.steamgames.com/doc)
- [SteamDB](https://steamdb.info) - Track your game's data
- [Steam Spy](https://steamspy.com) - Sales estimates
- [bevy_steamworks docs](https://docs.rs/bevy-steamworks)

## 🎮 Living Worlds Specific Notes

### Save File Compatibility
- Save version checking prevents crashes from incompatible saves
- Steam Cloud handles version conflicts automatically
- Compression reduces save sizes by ~90%

### Performance Targets
- 60 FPS on GTX 1060 or better
- 30 FPS minimum on integrated graphics
- <2 second world generation for Medium worlds
- <8 second generation for Large worlds

### System Requirements (Steam Store Page)

**Minimum:**
- OS: Windows 10 / Ubuntu 20.04
- Processor: Intel i5-4460 / AMD FX-6300
- Memory: 4 GB RAM
- Graphics: GTX 960 / R9 280
- Storage: 500 MB available space

**Recommended:**
- OS: Windows 11 / Ubuntu 22.04
- Processor: Intel i7-8700 / AMD Ryzen 5 3600
- Memory: 8 GB RAM
- Graphics: GTX 1060 / RX 580
- Storage: 1 GB available space

---

*Last Updated: December 2024*
*Game Version: 0.1.0*
*Steam SDK: v158a via bevy_steamworks 0.13*