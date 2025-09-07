mod hittable;
mod math;
mod ray;
mod sphere;

use std::fs::File;
use std::io::{BufWriter, Write};

use hittable::{Hittable, HittableList};
use math::{Color, Point3, Vec3};
use ray::Ray;
use sphere::Sphere;

fn write_color(w: &mut BufWriter<File>, pixel_color: Color) {
    let r = (pixel_color.x.clamp(0.0, 1.0) * 255.999) as i32;
    let g = (pixel_color.y.clamp(0.0, 1.0) * 255.999) as i32;
    let b = (pixel_color.z.clamp(0.0, 1.0) * 255.999) as i32;
    writeln!(w, "{r} {g} {b}").unwrap();
}

fn ray_color(r: &Ray, world: &impl Hittable) -> Color {
    if let Some(rec) = world.hit(r, 0.001, f64::INFINITY) {
        // Visualize normals: map [-1,1] -> [0,1]
        return (rec.normal + Color::new(1.0, 1.0, 1.0)) * 0.5;
    }
    let unit_dir = r.direction.unit();
    let t = 0.5 * (unit_dir.y + 1.0);
    (Color::new(1.0, 1.0, 1.0) * (1.0 - t)) + (Color::new(0.5, 0.7, 1.0) * t)
}

fn main() {
    // Image
    let aspect_ratio = 4.0 / 3.0;
    let image_width: i32 = 400;
    let image_height: i32 = (image_width as f64 / aspect_ratio) as i32;

    // Camera (simple pinhole facing -Z)
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    // World
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    // (Optional ground for later) world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Output file
    let file = File::create("output.ppm").expect("Failed to create file");
    let mut w = BufWriter::new(file);

    // PPM header
    writeln!(w, "P3").unwrap();
    writeln!(w, "{} {}", image_width, image_height).unwrap();
    writeln!(w, "255").unwrap();

    // Render
    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let u = i as f64 / (image_width - 1) as f64;
            let v = j as f64 / (image_height - 1) as f64;

            let dir = lower_left_corner + horizontal * u + vertical * v - origin;
            let r = Ray::new(origin, dir);

            let pixel_color = ray_color(&r, &world);
            write_color(&mut w, pixel_color);
        }
    }
}
