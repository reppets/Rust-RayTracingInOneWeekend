use std::io::Write;
use crate::vec3::{Color};

/*
    Color utility functions
*/

fn to_rgb_str(value: f64) -> String {
    ((255.999 * value) as i64).to_string()
}

pub fn write_color(out: &mut dyn Write, color: &Color) {
    writeln!(out, "{} {} {}", to_rgb_str(color.0), to_rgb_str(color.1), to_rgb_str(color.2)).unwrap();
}