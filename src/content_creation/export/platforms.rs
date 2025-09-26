//! Platform-specific export optimizations

use crate::content_creation::types::{OutputFormat, SocialPlatform};
use crate::nations::{DramaEvent, DramaEventType};

/// Recommend platforms based on event type and viral score
pub fn recommend_platforms(event: &DramaEvent, viral_score: f32) -> Vec<SocialPlatform> {
    let mut platforms = Vec::new();

    // TikTok loves absurd short moments
    if viral_score > 0.8 || matches!(event.event_type,
        DramaEventType::BabyRuler { .. } |
        DramaEventType::AbsurdEvent { .. } |
        DramaEventType::AnimalIncident { .. }
    ) {
        platforms.push(SocialPlatform::TikTok);
    }

    // Reddit loves detailed drama and complex stories
    if matches!(event.event_type,
        DramaEventType::Betrayal { .. } |
        DramaEventType::SuccessionCrisis { .. } |
        DramaEventType::SecretExposed { .. } |
        DramaEventType::FamilyFeud { .. }
    ) {
        platforms.push(SocialPlatform::Reddit);
    }

    // Twitter for quick viral moments
    if viral_score > 0.7 || matches!(event.event_type,
        DramaEventType::DrunkenIncident { .. } |
        DramaEventType::QuirkIncident { .. }
    ) {
        platforms.push(SocialPlatform::Twitter);
    }

    // YouTube for complex stories with multiple events
    if event.consequences.len() > 3 {
        platforms.push(SocialPlatform::YouTube);
    }

    // Discord for community sharing
    if viral_score > 0.6 {
        platforms.push(SocialPlatform::Discord);
    }

    // Steam for achievements and milestones
    platforms.push(SocialPlatform::Steam);

    platforms
}

/// Optimize format for a specific platform
pub fn optimize_for_platform(platform: SocialPlatform, default: OutputFormat) -> OutputFormat {
    match platform {
        SocialPlatform::TikTok => OutputFormat::MP4,   // TikTok prefers MP4
        SocialPlatform::Twitter => OutputFormat::MP4,  // Twitter handles MP4 well
        SocialPlatform::Reddit => OutputFormat::GIF,   // GIFs autoplay on Reddit
        SocialPlatform::YouTube => OutputFormat::MP4,  // YouTube standard
        SocialPlatform::Instagram => OutputFormat::MP4, // Instagram reels
        SocialPlatform::Discord => OutputFormat::WebM, // WebM embeds well
        SocialPlatform::Twitch => OutputFormat::MP4,  // Twitch clips
        SocialPlatform::Steam => default,              // Steam is flexible
    }
}

/// Get platform-specific constraints
pub fn platform_constraints(platform: SocialPlatform) -> PlatformConstraints {
    match platform {
        SocialPlatform::TikTok => PlatformConstraints {
            max_duration_seconds: 60.0,
            max_file_size_mb: 287.0,
            aspect_ratio: Some((9.0, 16.0)), // Vertical
            character_limit: Some(2200),
        },
        SocialPlatform::Twitter => PlatformConstraints {
            max_duration_seconds: 140.0,
            max_file_size_mb: 512.0,
            aspect_ratio: None,
            character_limit: Some(280),
        },
        SocialPlatform::YouTube => PlatformConstraints {
            max_duration_seconds: 43200.0, // 12 hours
            max_file_size_mb: 128000.0,    // 128 GB
            aspect_ratio: Some((16.0, 9.0)), // Horizontal
            character_limit: Some(5000),
        },
        SocialPlatform::Reddit => PlatformConstraints {
            max_duration_seconds: 60.0,
            max_file_size_mb: 1000.0,
            aspect_ratio: None,
            character_limit: Some(40000),
        },
        _ => PlatformConstraints::default(),
    }
}

/// Platform-specific constraints for content
#[derive(Debug, Clone)]
pub struct PlatformConstraints {
    pub max_duration_seconds: f32,
    pub max_file_size_mb: f32,
    pub aspect_ratio: Option<(f32, f32)>,
    pub character_limit: Option<usize>,
}

impl Default for PlatformConstraints {
    fn default() -> Self {
        Self {
            max_duration_seconds: 300.0,
            max_file_size_mb: 100.0,
            aspect_ratio: None,
            character_limit: None,
        }
    }
}