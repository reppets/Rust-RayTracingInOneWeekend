use std::io::Write;
use crate::vec3::{Color};

/*
    Color utility functions
*/

fn clamp(v: f64, min: f64, max: f64) -> f64 {
    if v < min {min} else if v > max {max} else {v}
}

pub fn write_color(out: &mut dyn Write, color: &Color, samples_per_pixel: i64) {
    let scale = 1.0 / samples_per_pixel as f64;
    let clamped = |c: f64| -> String {
        ((256.0 * clamp(c * scale, 0.0, 0.999)) as i64).to_string()
    };
    writeln!(out, "{} {} {}", clamped(color.0), clamped(color.1), clamped(color.2)).unwrap();
}