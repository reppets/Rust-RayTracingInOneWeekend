#![allow(unused)]

mod vec3;
mod color;

use std::fmt;
use std::io::Write;
use crate::vec3::{Vec3, Color};

fn main() {
    let image_width = 256;
    let image_height = 256;

    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    for y in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {}", y);
        for x in 0..image_width {
            let mut pixel_color = Vec3(
                x as f64 / (image_width - 1) as f64,
                y as f64 / (image_height - 1) as f64,
                0.25);
            pixel_color /= 2.0;
            color::write_color(&mut std::io::stdout(), &pixel_color)
        }
    }
    eprintln!("\nDone.");
}

