//! Status text tracking and update system

use crate::loading::state::LoadingState;
use crate::loading::ui::LoadingStatusText;
use bevy::prelude::*;

/// Update the status text
///
/// This system monitors changes to the LoadingState resource and
/// updates the status text UI element to display the current
/// loading step message.
pub fn update_loading_text(
    loading_state: Res<LoadingState>,
    mut query: Query<&mut Text, With<LoadingStatusText>>,
) {
    if loading_state.is_changed() {
        for mut text in &mut query {
            text.0 = loading_state.current_step.clone();
        }
    }
}
