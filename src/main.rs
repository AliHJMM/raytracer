mod camera;
mod cube;
mod cylinder;
mod hittable;
mod light;
mod math;
mod plane;
mod ray;
mod sphere;

use std::fs::File;
use std::io::{BufWriter, Write};

use camera::Camera;
use cube::Cube;
use cylinder::Cylinder;
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

// Hard-shadow Lambert
fn shade_lambert_with_shadow(
    hit_color: Color,
    normal: Vec3,
    p: Point3,
    light: &PointLight,
    world: &impl Hittable,
) -> Color {
    let ambient = 0.12;

    let to_light_vec = light.position - p;
    let light_dist = to_light_vec.length();
    let to_light_dir = to_light_vec / light_dist;

    const SHADOW_EPS: f64 = 1e-4;
    let shadow_origin = p + normal * SHADOW_EPS;
    let shadow_ray = Ray::new(shadow_origin, to_light_dir);
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
    // Sky gradient
    let unit_dir = r.direction.unit();
    let t = 0.5 * (unit_dir.y + 1.0);
    (Color::new(1.0, 1.0, 1.0) * (1.0 - t)) + (Color::new(0.5, 0.7, 1.0) * t)
}

fn main() {
    // Image
    let aspect_ratio = 4.0 / 3.0;
    let image_width: i32 = 400;
    let image_height: i32 = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel: i32 = 16; // 16–64 recommended

    // ----- Camera (you can change these to get the second perspective) -----
    let lookfrom = Point3::new(0.0, 0.0, 0.0);
    let lookat = Point3::new(0.0, 0.0, -1.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let vfov_deg = 90.0;
    let cam = Camera::new(lookfrom, lookat, vup, vfov_deg, aspect_ratio);

    // World (sphere + ground plane)
    // World (sphere + ground plane + cube)
    let mut world = HittableList::new();
    world.add(Box::new(Plane::new(
        Point3::new(0.0, -0.5, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        Color::new(0.8, 0.8, 0.8),
    )));
    // Sphere (left)
    world.add(Box::new(Sphere::new(
        Point3::new(-0.8, 0.0, -1.3),
        0.5,
        Color::new(0.9, 0.2, 0.2),
    )));

    // Cube (center)
    world.add(Box::new(Cube::from_center_size(
        Point3::new(0.3, -0.2, -1.4),
        0.6,
        Color::new(0.35, 0.42, 0.65),
    )));

    // Cylinder (right)
    world.add(Box::new(Cylinder::new(
        Point3::new(1.4, -0.1, -1.6),
        0.3,
        0.4,
        Color::new(0.2, 0.7, 0.4),
    )));

    // Light (white)
    let light = PointLight::new(Point3::new(5.0, 5.0, -2.0), Color::new(1.0, 1.0, 1.0));

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
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _s in 0..samples_per_pixel {
                let u = (i as f64 + math::random_f64()) / (image_width - 1) as f64;
                let v = (j as f64 + math::random_f64()) / (image_height - 1) as f64;
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &world, &light);
            }
            write_color(&mut w, pixel_color, samples_per_pixel);
        }
    }
}
