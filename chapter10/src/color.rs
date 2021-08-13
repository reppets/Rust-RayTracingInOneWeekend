use std::io::Write;
use crate::vec3::{Color, Vec3};

/*
    Color utility functions
*/

fn clamp(v: f64, min: f64, max: f64) -> f64 {
    if v < min {min} else if v > max {max} else {v}
}

pub fn write_color(out: &mut dyn Write, color: &Color, samples_per_pixel: i64) {
    let scale = 1.0 / samples_per_pixel as f64;
    let corrected_color = Vec3((color.0 * scale).sqrt(), (color.1 * scale).sqrt(), (color.2 * scale).sqrt());
    let clamped = |c: f64| -> String {
        ((256.0 * clamp(c, 0.0, 0.999)) as i64).to_string()
    };
    writeln!(out, "{} {} {}", clamped(corrected_color.0), clamped(corrected_color.1), clamped(corrected_color.2)).unwrap();
}