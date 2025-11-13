//! Types specific to viral moment detection


/// Configuration for viral moment detection
#[derive(Debug, Clone)]
pub struct DetectionConfig {
    /// Threshold score for considering something viral
    pub viral_threshold: f32,
    /// Maximum number of events to keep in buffer
    pub buffer_size: usize,
    /// Whether to auto-detect patterns
    pub auto_detect: bool,
    /// Minimum time between viral detections (to avoid spam)
    pub cooldown_seconds: f32,
}

impl Default for DetectionConfig {
    fn default() -> Self {
        Self {
            viral_threshold: 0.7,
            buffer_size: 100,
            auto_detect: true,
            cooldown_seconds: 5.0,
        }
    }
}