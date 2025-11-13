//! Systems for updating animations each frame

use bevy::prelude::*;
use std::time::Duration;
use super::components::*;
use super::types::*;

/// Update all active animations
pub fn update_animations(
    time: Res<Time>,
    mut commands: Commands,
    mut animation_query: Query<(
        Entity,
        &mut Animation,
        &mut UIAnimationPlayer,
        Option<&mut Transform>,
        Option<&mut BackgroundColor>,
        Option<&mut TextColor>,
        Option<&mut Node>,
    )>,
) {
    for (entity, mut animation, mut player, transform, bg_color, text_color, node) in &mut animation_query {
        // Skip if not playing
        if animation.state != AnimationState::Playing && animation.state != AnimationState::Pending {
            continue;
        }

        let delta = time.delta();

        // Handle delay phase
        if animation.state == AnimationState::Pending {
            player.delay_elapsed += delta;
            if player.delay_elapsed >= animation.delay {
                animation.state = AnimationState::Playing;
                player.delay_elapsed = animation.delay; // Clamp to exact delay
            } else {
                continue; // Still in delay phase
            }
        }

        // Update elapsed time
        player.elapsed += delta;

        // Calculate raw progress (0.0 to 1.0)
        let raw_progress = if animation.duration.as_secs_f32() > 0.0 {
            (player.elapsed.as_secs_f32() / animation.duration.as_secs_f32()).min(1.0)
        } else {
            1.0 // Instant animation
        };

        // Apply direction for PingPong mode
        let directional_progress = match player.direction {
            AnimationDirection::Forward => raw_progress,
            AnimationDirection::Reverse => 1.0 - raw_progress,
        };

        // Apply easing function
        player.progress = animation.easing.ease(directional_progress);

        // Get interpolated value
        let value = animation.target.interpolate(player.progress);

        // Apply the interpolated value to the appropriate component
        apply_animation_value(value, transform, bg_color, text_color, node);

        // Check if animation is complete
        if raw_progress >= 1.0 {
            handle_animation_completion(
                &mut commands,
                entity,
                &mut animation,
                &mut player,
            );
        }
    }
}

/// Apply an animation value to the appropriate component
fn apply_animation_value(
    value: AnimationValue,
    transform: Option<Mut<Transform>>,
    bg_color: Option<Mut<BackgroundColor>>,
    text_color: Option<Mut<TextColor>>,
    node: Option<Mut<Node>>,
) {
    match value {
        AnimationValue::Position(pos) => {
            if let Some(mut transform) = transform {
                transform.translation = pos;
            }
        }
        AnimationValue::Scale(scale) => {
            if let Some(mut transform) = transform {
                transform.scale = scale;
            }
        }
        AnimationValue::Rotation(rot) => {
            if let Some(mut transform) = transform {
                transform.rotation = rot;
            }
        }
        AnimationValue::BackgroundColor(color) => {
            if let Some(mut bg) = bg_color {
                bg.0 = color;
            }
        }
        AnimationValue::TextColor(color) => {
            if let Some(mut text) = text_color {
                text.0 = color;
            }
        }
        AnimationValue::Opacity(alpha) => {
            // Apply to whichever color component exists
            if let Some(mut bg) = bg_color {
                bg.0 = bg.0.with_alpha(alpha);
            }
            if let Some(mut text) = text_color {
                text.0 = text.0.with_alpha(alpha);
            }
        }
        AnimationValue::Width(val) => {
            if let Some(mut node) = node {
                node.width = val;
            }
        }
        AnimationValue::Height(val) => {
            if let Some(mut node) = node {
                node.height = val;
            }
        }
    }
}

/// Handle animation completion based on repeat mode
fn handle_animation_completion(
    commands: &mut Commands,
    entity: Entity,
    animation: &mut Animation,
    player: &mut UIAnimationPlayer,
) {
    match animation.repeat_mode {
        AnimationRepeatMode::Once => {
            animation.state = AnimationState::Completed;

            // Add completion marker
            commands.entity(entity).insert(AnimationComplete {
                animation_id: animation.id,
            });

            // Auto-cleanup if enabled
            if animation.auto_cleanup {
                commands.entity(entity)
                    .remove::<Animation>()
                    .remove::<UIAnimationPlayer>();
            }
        }
        AnimationRepeatMode::Loop => {
            // Reset for next loop
            player.elapsed = Duration::ZERO;
            player.loops_completed += 1;
        }
        AnimationRepeatMode::PingPong => {
            // Reverse direction
            player.direction = match player.direction {
                AnimationDirection::Forward => AnimationDirection::Reverse,
                AnimationDirection::Reverse => {
                    player.loops_completed += 1;
                    AnimationDirection::Forward
                }
            };
            player.elapsed = Duration::ZERO;
        }
        AnimationRepeatMode::Count(count) => {
            player.loops_completed += 1;
            if player.loops_completed >= count {
                animation.state = AnimationState::Completed;

                commands.entity(entity).insert(AnimationComplete {
                    animation_id: animation.id,
                });

                if animation.auto_cleanup {
                    commands.entity(entity)
                        .remove::<Animation>()
                        .remove::<UIAnimationPlayer>();
                }
            } else {
                player.elapsed = Duration::ZERO;
            }
        }
    }
}

/// Process animation sequences
pub fn process_animation_sequences(
    mut commands: Commands,
    mut sequence_query: Query<(Entity, &mut AnimationSequence), Without<Animation>>,
) {
    for (entity, sequence) in &mut sequence_query {
        // Check if we need to start the first animation
        if let Some(animation) = sequence.current_animation() {
            let animation = animation.clone();
            // Add both Animation and UIAnimationPlayer
            commands.entity(entity).insert((animation, UIAnimationPlayer::default()));
        }
    }
}

/// Advance sequences when current animation completes
pub fn advance_animation_sequences(
    mut commands: Commands,
    mut sequence_query: Query<(Entity, &mut AnimationSequence, &Animation)>,
) {
    for (entity, mut sequence, animation) in &mut sequence_query {
        if animation.state == AnimationState::Completed {
            // Remove current animation and player
            commands.entity(entity)
                .remove::<Animation>()
                .remove::<UIAnimationPlayer>()
                .remove::<AnimationComplete>();

            // Advance to next animation
            if sequence.advance() {
                if let Some(next_animation) = sequence.current_animation() {
                    let next_animation = next_animation.clone();
                    // Add both Animation and UIAnimationPlayer
                    commands.entity(entity).insert((next_animation, UIAnimationPlayer::default()));
                }
            } else {
                // Sequence complete, remove it
                commands.entity(entity).remove::<AnimationSequence>();
            }
        }
    }
}

/// Clean up completed animations that have been marked
pub fn cleanup_completed_animations(
    mut commands: Commands,
    query: Query<Entity, With<AnimationComplete>>,
) {
    for entity in &query {
        commands.entity(entity).remove::<AnimationComplete>();
    }
}

/// Pause all animations
pub fn pause_all_animations(
    mut animation_query: Query<&mut Animation>,
) {
    for mut animation in &mut animation_query {
        if animation.state == AnimationState::Playing {
            animation.state = AnimationState::Paused;
        }
    }
}

/// Resume all paused animations
pub fn resume_all_animations(
    mut animation_query: Query<&mut Animation>,
) {
    for mut animation in &mut animation_query {
        if animation.state == AnimationState::Paused {
            animation.state = AnimationState::Playing;
        }
    }
}

/// Cancel all animations on specific entity
pub fn cancel_entity_animations(
    entity: Entity,
    mut commands: Commands,
    mut animation_query: Query<&mut Animation>,
) {
    if let Ok(mut animation) = animation_query.get_mut(entity) {
        animation.state = AnimationState::Cancelled;
        commands.entity(entity)
            .remove::<Animation>()
            .remove::<UIAnimationPlayer>()
            .remove::<AnimationSequence>();
    }
}

/// Debug system to log animation states
#[cfg(debug_assertions)]
pub fn debug_animations(
    animation_query: Query<(Entity, &Animation, &UIAnimationPlayer), Changed<UIAnimationPlayer>>,
) {
    for (entity, animation, player) in &animation_query {
        if player.progress > 0.0 && player.progress < 1.0 {
            debug!(
                "Animation {:?} on entity {:?}: progress {:.2}%, state {:?}",
                animation.id,
                entity,
                player.progress * 100.0,
                animation.state
            );
        }
    }
}

/// System to handle global animation configuration
pub fn apply_animation_config(
    config: Res<super::types::AnimationConfig>,
    mut animation_query: Query<&mut Animation>,
) {
    if config.paused {
        for mut animation in &mut animation_query {
            if animation.state == AnimationState::Playing {
                animation.state = AnimationState::Paused;
            }
        }
    }
}