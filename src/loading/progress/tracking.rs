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
    // Temporarily removed change detection to debug the issue
    // if !loading_state.is_changed() {
    //     return;
    // }

    for mut progress_bar in &mut query {
        if progress_bar.value != loading_state.progress {
            bevy::log::info!(
                "UI System: Updating progress bar from {:.1}% to {:.1}%",
                progress_bar.value * 100.0,
                loading_state.progress * 100.0
            );
            progress_bar.value = loading_state.progress.clamp(0.0, 1.0);
        }
    }
}
