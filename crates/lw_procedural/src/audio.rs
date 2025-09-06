//! Procedural audio generation using MIDI patterns and wavetable synthesis
//! No audio files - everything generated in real-time

use lw_core::DeterministicRNG;
use std::f32::consts::PI;

/// Basic waveform types for synthesis
#[derive(Debug, Clone, Copy)]
pub enum WaveformType {
    Sine,
    Square,
    Sawtooth,
    Triangle,
    Noise,
}

/// Musical note representation
#[derive(Debug, Clone)]
pub struct Note {
    pub pitch: i32,        // MIDI note number (60 = Middle C)
    pub velocity: f32,     // Volume (0-1)
    pub duration: f32,     // In beats
    pub start_time: f32,   // When to play in pattern
}

impl Note {
    /// Convert MIDI note to frequency
    pub fn to_frequency(&self) -> f32 {
        440.0 * 2.0_f32.powf((self.pitch - 69) as f32 / 12.0)
    }
}

/// MIDI pattern for melodies and rhythms
pub struct MidiPattern {
    notes: Vec<Note>,
    beats_per_minute: f32,
    pattern_length: f32,  // In beats
}

impl MidiPattern {
    pub fn new(bpm: f32) -> Self {
        Self {
            notes: Vec::new(),
            beats_per_minute: bpm,
            pattern_length: 4.0,
        }
    }
    
    /// Add note to pattern
    pub fn add_note(&mut self, pitch: i32, start: f32, duration: f32, velocity: f32) {
        self.notes.push(Note {
            pitch,
            velocity,
            duration,
            start_time: start,
        });
    }
    
    /// Generate pentatonic melody
    pub fn generate_pentatonic_melody(&mut self, rng: &mut DeterministicRNG, base_note: i32, num_notes: usize) {
        let pentatonic = [0, 2, 4, 7, 9]; // Minor pentatonic intervals
        let mut time = 0.0;
        
        for _ in 0..num_notes {
            let interval = rng.choose(&pentatonic).unwrap_or(&0);
            let octave = rng.range_i32(-1, 2);
            let pitch = base_note + interval + octave * 12;
            
            let duration = if rng.next_bool(0.7) { 0.25 } else { 0.5 };
            let velocity = 0.6 + rng.next_f32() * 0.3;
            
            self.add_note(pitch, time, duration, velocity);
            time += duration;
        }
        
        self.pattern_length = time;
    }
    
    /// Generate drum pattern
    pub fn generate_drum_pattern(&mut self, rng: &mut DeterministicRNG, complexity: i32) {
        // Kick drum on beats
        for i in 0..4 {
            self.add_note(36, i as f32, 0.25, 0.9); // C2 kick
        }
        
        // Snare on 2 and 4
        self.add_note(38, 1.0, 0.25, 0.8); // D2 snare
        self.add_note(38, 3.0, 0.25, 0.8);
        
        // Hi-hats based on complexity
        if complexity > 1 {
            for i in 0..16 {
                if rng.next_bool(0.6) {
                    let time = i as f32 * 0.25;
                    self.add_note(42, time, 0.125, 0.4 + rng.next_f32() * 0.2);
                }
            }
        }
        
        self.pattern_length = 4.0;
    }
}

/// Wavetable oscillator for sound synthesis
pub struct Oscillator {
    waveform: WaveformType,
    frequency: f32,
    amplitude: f32,
    phase: f32,
    sample_rate: f32,
}

impl Oscillator {
    pub fn new(waveform: WaveformType, frequency: f32, sample_rate: f32) -> Self {
        Self {
            waveform,
            frequency,
            amplitude: 1.0,
            phase: 0.0,
            sample_rate,
        }
    }
    
    /// Generate next sample
    pub fn next_sample(&mut self) -> f32 {
        let sample = match self.waveform {
            WaveformType::Sine => (self.phase * 2.0 * PI).sin(),
            WaveformType::Square => if self.phase < 0.5 { 1.0 } else { -1.0 },
            WaveformType::Sawtooth => 2.0 * self.phase - 1.0,
            WaveformType::Triangle => {
                if self.phase < 0.5 {
                    4.0 * self.phase - 1.0
                } else {
                    3.0 - 4.0 * self.phase
                }
            }
            WaveformType::Noise => rand::random::<f32>() * 2.0 - 1.0,
        };
        
        // Advance phase
        self.phase += self.frequency / self.sample_rate;
        while self.phase >= 1.0 {
            self.phase -= 1.0;
        }
        
        sample * self.amplitude
    }
}

/// Main audio generator for the game
pub struct AudioGenerator {
    sample_rate: f32,
    master_volume: f32,
    rng: DeterministicRNG,
}

impl AudioGenerator {
    pub fn new(seed: u64) -> Self {
        Self {
            sample_rate: 44100.0,
            master_volume: 0.8,
            rng: DeterministicRNG::new(seed),
        }
    }
    
    /// Generate ambient music track
    pub fn generate_ambient_track(&mut self, duration_seconds: f32) -> Vec<f32> {
        let num_samples = (self.sample_rate * duration_seconds) as usize;
        let mut output = vec![0.0; num_samples];
        
        // Create drone with multiple oscillators
        let base_freq = 110.0; // A2
        let mut oscillators = vec![
            Oscillator::new(WaveformType::Sine, base_freq, self.sample_rate),
            Oscillator::new(WaveformType::Sine, base_freq * 1.5, self.sample_rate),
            Oscillator::new(WaveformType::Triangle, base_freq * 2.0, self.sample_rate),
        ];
        
        // Set amplitudes
        oscillators[0].amplitude = 0.4;
        oscillators[1].amplitude = 0.2;
        oscillators[2].amplitude = 0.1;
        
        // Generate samples
        for sample in &mut output {
            let mut value = 0.0;
            for osc in &mut oscillators {
                value += osc.next_sample();
            }
            
            // Apply slow amplitude modulation for "breathing" effect
            let mod_freq = 0.1;
            let mod_phase = (sample as *const f32 as usize) as f32 / self.sample_rate;
            let amplitude = 0.5 + 0.3 * (mod_phase * mod_freq * 2.0 * PI).sin();
            
            *sample = value * amplitude * self.master_volume;
        }
        
        output
    }
    
    /// Generate UI sound effect
    pub fn generate_ui_click(&mut self) -> Vec<f32> {
        let duration = 0.05; // 50ms
        let num_samples = (self.sample_rate * duration) as usize;
        let mut output = vec![0.0; num_samples];
        
        let mut osc = Oscillator::new(WaveformType::Sine, 1000.0, self.sample_rate);
        
        for (i, sample) in output.iter_mut().enumerate() {
            // Exponential decay envelope
            let envelope = (-5.0 * i as f32 / num_samples as f32).exp();
            *sample = osc.next_sample() * envelope * 0.5;
        }
        
        output
    }
    
    /// Generate battle sound effect
    pub fn generate_battle_sound(&mut self) -> Vec<f32> {
        let duration = 1.0;
        let num_samples = (self.sample_rate * duration) as usize;
        let mut output = vec![0.0; num_samples];
        
        // Low frequency rumble
        let mut rumble = Oscillator::new(WaveformType::Triangle, 40.0, self.sample_rate);
        // Mid frequency clash
        let mut clash = Oscillator::new(WaveformType::Square, 200.0, self.sample_rate);
        
        for (i, sample) in output.iter_mut().enumerate() {
            let t = i as f32 / num_samples as f32;
            
            // Attack-decay envelope
            let envelope = if t < 0.1 {
                t * 10.0
            } else {
                1.0 - (t - 0.1) * 1.1
            };
            
            *sample = (rumble.next_sample() * 0.7 + clash.next_sample() * 0.3) 
                     * envelope.max(0.0) * self.master_volume;
        }
        
        output
    }
}

/// Musical scale definitions
pub mod scales {
    pub const PENTATONIC_MINOR: [i32; 5] = [0, 3, 5, 7, 10];
    pub const PENTATONIC_MAJOR: [i32; 5] = [0, 2, 4, 7, 9];
    pub const NATURAL_MINOR: [i32; 7] = [0, 2, 3, 5, 7, 8, 10];
    pub const MAJOR: [i32; 7] = [0, 2, 4, 5, 7, 9, 11];
}

/// Note frequency constants
pub mod notes {
    pub const C0: f32 = 16.35;
    pub const C4: f32 = 261.63; // Middle C
    pub const A4: f32 = 440.0;  // Concert A
}