use std::fs::File;
use std::io::{BufWriter, Write};

fn main() {
    // Image dimensions
    let width = 256;
    let height = 256;

    // Open file for writing
    let file = File::create("output.ppm").expect("Failed to create file");
    let mut writer = BufWriter::new(file);

    // Write PPM header
    writeln!(writer, "P3").unwrap();
    writeln!(writer, "{} {}", width, height).unwrap();
    writeln!(writer, "255").unwrap();

    // Generate gradient (horizontal: red, vertical: green, diagonal mix: blue)
    for j in (0..height).rev() {
        for i in 0..width {
            let r = (i as f64 / (width - 1) as f64 * 255.0) as i32;
            let g = (j as f64 / (height - 1) as f64 * 255.0) as i32;
            let b = (0.25 * 255.0) as i32; // constant blue

            writeln!(writer, "{} {} {}", r, g, b).unwrap();
        }
    }
}
