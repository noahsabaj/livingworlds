//! Easing functions for smooth animations
//!
//! Based on Robert Penner's easing equations and CSS easing functions

/// Linear interpolation (no easing)
pub fn linear(t: f32) -> f32 {
    t
}

/// Ease in (slow start, accelerating)
pub fn ease_in(t: f32) -> f32 {
    t * t
}

/// Ease out (fast start, decelerating)
pub fn ease_out(t: f32) -> f32 {
    1.0 - (1.0 - t) * (1.0 - t)
}

/// Ease in and out (slow start and end)
pub fn ease_in_out(t: f32) -> f32 {
    if t < 0.5 {
        2.0 * t * t
    } else {
        1.0 - (-2.0 * t + 2.0).powi(2) / 2.0
    }
}

/// Bounce effect at the end
pub fn bounce(t: f32) -> f32 {
    const N1: f32 = 7.5625;
    const D1: f32 = 2.75;

    let t = t.clamp(0.0, 1.0);

    if t < 1.0 / D1 {
        N1 * t * t
    } else if t < 2.0 / D1 {
        let t = t - 1.5 / D1;
        N1 * t * t + 0.75
    } else if t < 2.5 / D1 {
        let t = t - 2.25 / D1;
        N1 * t * t + 0.9375
    } else {
        let t = t - 2.625 / D1;
        N1 * t * t + 0.984375
    }
}

/// Elastic spring effect
pub fn elastic(t: f32) -> f32 {
    const C4: f32 = 2.0 * std::f32::consts::PI / 3.0;

    if t == 0.0 {
        0.0
    } else if t == 1.0 {
        1.0
    } else {
        -(2.0_f32.powf(-10.0 * t)) * ((t * 10.0 - 10.75) * C4).sin() + 1.0
    }
}

/// Back effect (overshoot then return)
pub fn back_in_out(t: f32) -> f32 {
    const C1: f32 = 1.70158;
    const C2: f32 = C1 * 1.525;

    if t < 0.5 {
        ((2.0 * t).powi(2) * ((C2 + 1.0) * 2.0 * t - C2)) / 2.0
    } else {
        ((2.0 * t - 2.0).powi(2) * ((C2 + 1.0) * (t * 2.0 - 2.0) + C2) + 2.0) / 2.0
    }
}

/// Cubic bezier curve for custom easing
pub fn cubic_bezier(x1: f32, y1: f32, x2: f32, y2: f32, t: f32) -> f32 {
    // Simplified cubic bezier approximation
    // For accurate implementation, would need Newton-Raphson iteration
    let t2 = t * t;
    let t3 = t2 * t;

    let mt = 1.0 - t;
    let mt2 = mt * mt;
    let mt3 = mt2 * mt;

    // B(t) = (1-t)³·P0 + 3(1-t)²t·P1 + 3(1-t)t²·P2 + t³·P3
    // Where P0=(0,0), P1=(x1,y1), P2=(x2,y2), P3=(1,1)
    3.0 * mt2 * t * y1 + 3.0 * mt * t2 * y2 + t3
}

/// Exponential ease in
pub fn exp_in(t: f32) -> f32 {
    if t == 0.0 {
        0.0
    } else {
        2.0_f32.powf(10.0 * t - 10.0)
    }
}

/// Exponential ease out
pub fn exp_out(t: f32) -> f32 {
    if t == 1.0 {
        1.0
    } else {
        1.0 - 2.0_f32.powf(-10.0 * t)
    }
}

/// Circular ease in (quarter circle)
pub fn circ_in(t: f32) -> f32 {
    1.0 - (1.0 - t * t).sqrt()
}

/// Circular ease out (quarter circle)
pub fn circ_out(t: f32) -> f32 {
    (1.0 - (1.0 - t) * (1.0 - t)).sqrt()
}

/// Sine wave ease in
pub fn sine_in(t: f32) -> f32 {
    1.0 - ((t * std::f32::consts::PI / 2.0).cos())
}

/// Sine wave ease out
pub fn sine_out(t: f32) -> f32 {
    (t * std::f32::consts::PI / 2.0).sin()
}

/// Quartic ease in (t^4)
pub fn quart_in(t: f32) -> f32 {
    t * t * t * t
}

/// Quartic ease out
pub fn quart_out(t: f32) -> f32 {
    1.0 - (1.0 - t).powi(4)
}

/// Quintic ease in (t^5)
pub fn quint_in(t: f32) -> f32 {
    t * t * t * t * t
}

/// Quintic ease out
pub fn quint_out(t: f32) -> f32 {
    1.0 - (1.0 - t).powi(5)
}