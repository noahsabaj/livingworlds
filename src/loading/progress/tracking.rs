//! Progress bar tracking and update system

use crate::loading::state::LoadingState;
use crate::loading::ui::LoadingProgressBar;
use crate::ui::ProgressBar;
use bevy::prelude::*;

/// Update the progress bar value
///
/// This system monitors changes to the LoadingState resource and
/// synchronizes the progress bar UI element to reflect the current
/// loading progress (0.0 to 1.0).
pub fn update_loading_progress(
    loading_state: Res<LoadingState>,
    mut query: Query<&mut ProgressBar, With<LoadingProgressBar>>,
) {
    if !loading_state.is_changed() {
        return;
    }

    for mut progress_bar in &mut query {
        progress_bar.value = loading_state.progress.clamp(0.0, 1.0);
    }
}
