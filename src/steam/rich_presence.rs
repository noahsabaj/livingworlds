//! Steam Rich Presence integration
//!
//! This module handles updating Steam Rich Presence to show what the player
//! is currently doing in Living Worlds. This information is visible to
//! Steam friends and enhances the social experience.

use bevy::prelude::*;

use super::types::SteamClient;

/// Update Steam Rich Presence to show what the player is doing
pub fn update_rich_presence(
    steam: Res<SteamClient>,
    game_state: Res<State<crate::states::GameState>>,
    game_time: Option<Res<crate::resources::GameTime>>,
    world_size: Option<Res<crate::resources::WorldSize>>,
) {
    let client = &steam.0;
    let friends = client.friends();

    let status = match **game_state {
        crate::states::GameState::MainMenu => "In Main Menu",
        crate::states::GameState::WorldConfiguration => "Configuring New World",
        crate::states::GameState::LoadingWorld => "Loading World",
        crate::states::GameState::InGame => {
            if let Some(time) = game_time {
                let year = time.current_year() as i32;
                &format!("Observing Year {}", year)
            } else {
                "Observing History"
            }
        }
        crate::states::GameState::Paused => "Paused",
        _ => "Playing",
    };

    friends.set_rich_presence("status", status);

    if let Some(size) = world_size {
        let size_str = format!("{:?}", size);
        friends.set_rich_presence("world_size", &size_str);
    }

    // Steam will display: "Playing Living Worlds - Observing Year 1453"
}
