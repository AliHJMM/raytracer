mod hittable;
mod light;
mod math;
mod plane;
mod ray;
mod sphere;

use std::fs::File;
use std::io::{BufWriter, Write};

use hittable::{Hittable, HittableList};
use light::PointLight;
use math::{Color, Point3, Vec3};
use plane::Plane;
use ray::Ray;
use sphere::Sphere;

fn write_color(w: &mut BufWriter<File>, pixel_color: Color, samples_per_pixel: i32) {
    let scale = 1.0 / samples_per_pixel as f64;
    let mut r = pixel_color.x * scale;
    let mut g = pixel_color.y * scale;
    let mut b = pixel_color.z * scale;

    // Gamma-correct for gamma=2.0
    r = r.sqrt();
    g = g.sqrt();
    b = b.sqrt();

    let to_byte = |c: f64| (c.clamp(0.0, 0.999) * 256.0) as i32;
    writeln!(w, "{} {} {}", to_byte(r), to_byte(g), to_byte(b)).unwrap();
}

// Simple ambient + diffuse (Lambert)
// No shadows yet; we'll add them next step.
fn shade_lambert_with_shadow(
    hit_color: Color,
    normal: Vec3,
    p: Point3,
    light: &PointLight,
    world: &impl Hittable,
) -> Color {
    let ambient = 0.12;

    // Shadow ray setup
    let to_light_vec = light.position - p;
    let light_dist = to_light_vec.length();
    let to_light_dir = to_light_vec / light_dist;

    // Small bias to avoid self-shadowing acne
    const SHADOW_EPS: f64 = 1e-4;
    let shadow_origin = p + normal * SHADOW_EPS;
    let shadow_ray = Ray::new(shadow_origin, to_light_dir);

    // If anything blocks the light before it reaches the point, it's in shadow
    let in_shadow = world
        .hit(&shadow_ray, SHADOW_EPS, light_dist - SHADOW_EPS)
        .is_some();

    let diffuse = if in_shadow {
        0.0
    } else {
        f64::max(0.0, Vec3::dot(normal, to_light_dir))
    };

    let lighting = ambient + diffuse;
    (hit_color * lighting) * light.intensity
}

fn ray_color(r: &Ray, world: &impl Hittable, light: &PointLight) -> Color {
    if let Some(rec) = world.hit(r, 0.001, f64::INFINITY) {
        return shade_lambert_with_shadow(rec.albedo, rec.normal, rec.p, light, world);
    }
    // Sky
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

    // World (sphere + ground plane)
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
        Color::new(0.9, 0.2, 0.2), // red sphere
    )));
    world.add(Box::new(Plane::new(
        Point3::new(0.0, -0.5, 0.0), // y = -0.5 plane
        Vec3::new(0.0, 1.0, 0.0),    // upward normal
        Color::new(0.8, 0.8, 0.8),   // grey ground
    )));

    // Light (white, above-right)
    let light = PointLight::new(Point3::new(5.0, 5.0, -2.0), Color::new(1.0, 1.0, 1.0));

    // Output file
    let file = File::create("output.ppm").expect("Failed to create file");
    let mut w = BufWriter::new(file);

    // PPM header
    writeln!(w, "P3").unwrap();
    writeln!(w, "{} {}", image_width, image_height).unwrap();
    writeln!(w, "255").unwrap();

    // Render
    let samples_per_pixel: i32 = 16; // try 16–64 for smoother edges

    // Render
    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _s in 0..samples_per_pixel {
                let u = (i as f64 + math::random_f64()) / (image_width - 1) as f64;
                let v = (j as f64 + math::random_f64()) / (image_height - 1) as f64;

                let dir = lower_left_corner + horizontal * u + vertical * v - origin;
                let r = Ray::new(origin, dir);

                let sample = ray_color(&r, &world, &light);
                pixel_color += sample;
            }
            write_color(&mut w, pixel_color, samples_per_pixel);
        }
    }
}
