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

    // gamma 2.0
    r = r.sqrt();
    g = g.sqrt();
    b = b.sqrt();

    let to_byte = |c: f64| (c.clamp(0.0, 0.999) * 256.0) as i32;
    writeln!(w, "{} {} {}", to_byte(r), to_byte(g), to_byte(b)).unwrap();
}

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
    // sky
    let unit_dir = r.direction.unit();
    let t = 0.5 * (unit_dir.y + 1.0);
    (Color::new(1.0, 1.0, 1.0) * (1.0 - t)) + (Color::new(0.5, 0.7, 1.0) * t)
}

#[derive(Clone, Copy)]
enum SceneKind {
    Sphere,
    CubePlaneDim,
    All,
    AllAltCam,
}

struct Args {
    scene: SceneKind,
    width: i32,
    height: i32,
    out: String,
    samples_per_pixel: i32,
}

fn parse_resolution(s: &str) -> Option<(i32, i32)> {
    let lower = s.to_lowercase(); // keep the String alive
    let parts: Vec<&str> = lower.split('x').collect();
    if parts.len() != 2 {
        return None;
    }
    let w = parts[0].parse::<i32>().ok()?;
    let h = parts[1].parse::<i32>().ok()?;
    Some((w.max(1), h.max(1)))
}

fn parse_args() -> Args {
    let mut scene = SceneKind::All;
    let mut width = 400;
    let mut height = 300;
    let mut out: Option<String> = None;
    let mut spp = 16;

    for a in std::env::args().skip(1) {
        if let Some(val) = a.strip_prefix("--scene=") {
            scene = match val {
                "sphere" => SceneKind::Sphere,
                "cube_plane_dim" => SceneKind::CubePlaneDim,
                "all" => SceneKind::All,
                "all_alt_cam" => SceneKind::AllAltCam,
                _ => SceneKind::All,
            };
        } else if let Some(val) = a.strip_prefix("--res=") {
            if let Some((w, h)) = parse_resolution(val) {
                width = w;
                height = h;
            }
        } else if let Some(val) = a.strip_prefix("--out=") {
            out = Some(val.to_string());
        } else if let Some(val) = a.strip_prefix("--spp=") {
            if let Ok(v) = val.parse::<i32>() {
                spp = v.max(1);
            }
        }
    }

    let default_out = match scene {
        SceneKind::Sphere => "scene_sphere.ppm",
        SceneKind::CubePlaneDim => "scene_cube_plane_dim.ppm",
        SceneKind::All => "scene_all.ppm",
        SceneKind::AllAltCam => "scene_all_alt_cam.ppm",
    }
    .to_string();

    Args {
        scene,
        width,
        height,
        out: out.unwrap_or(default_out),
        samples_per_pixel: spp,
    }
}

struct Scene {
    world: HittableList,
    light: PointLight,
    cam: Camera,
    aspect_ratio: f64,
}

fn build_scene(kind: SceneKind, width: i32, height: i32) -> Scene {
    let aspect_ratio = width as f64 / height as f64;

    match kind {
        // 1) Sphere-only scene
        SceneKind::Sphere => {
            let mut world = HittableList::new();
            world.add(Box::new(Plane::new(
                Point3::new(0.0, -0.5, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
                Color::new(0.82, 0.82, 0.82),
            )));
            world.add(Box::new(Sphere::new(
                Point3::new(0.0, 0.0, -1.3),
                0.5,
                Color::new(0.9, 0.2, 0.2),
            )));

            let light = PointLight::new(Point3::new(5.0, 5.0, -2.0), Color::new(1.0, 1.0, 1.0));

            let lookfrom = Point3::new(0.0, 0.0, 0.0);
            let lookat = Point3::new(0.0, 0.0, -1.0);
            let cam = Camera::new(
                lookfrom,
                lookat,
                Vec3::new(0.0, 1.0, 0.0),
                90.0,
                aspect_ratio,
            );

            Scene {
                world,
                light,
                cam,
                aspect_ratio,
            }
        }

        // 2) Flat plane + cube with lower brightness than sphere image
        SceneKind::CubePlaneDim => {
            let mut world = HittableList::new();
            world.add(Box::new(Plane::new(
                Point3::new(0.0, -0.5, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
                Color::new(0.82, 0.82, 0.82),
            )));
            world.add(Box::new(Cube::from_center_size(
                Point3::new(0.0, -0.2, -1.3),
                0.6,
                Color::new(0.25, 0.28, 0.35), // darker cube albedo
            )));

            // Dimmer light than sphere scene to satisfy "lower brightness"
            let light = PointLight::new(Point3::new(5.0, 5.0, -2.0), Color::new(0.6, 0.6, 0.6));

            let lookfrom = Point3::new(0.0, 0.0, 0.0);
            let lookat = Point3::new(0.0, -0.1, -1.3);
            let cam = Camera::new(
                lookfrom,
                lookat,
                Vec3::new(0.0, 1.0, 0.0),
                90.0,
                aspect_ratio,
            );

            Scene {
                world,
                light,
                cam,
                aspect_ratio,
            }
        }

        // 3) All objects
        SceneKind::All => {
            let mut world = HittableList::new();
            world.add(Box::new(Plane::new(
                Point3::new(0.0, -0.5, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
                Color::new(0.82, 0.82, 0.82),
            )));
            world.add(Box::new(Sphere::new(
                Point3::new(-0.8, 0.0, -1.3),
                0.5,
                Color::new(0.9, 0.2, 0.2),
            )));
            world.add(Box::new(Cube::from_center_size(
                Point3::new(0.3, -0.2, -1.4),
                0.6,
                Color::new(0.35, 0.42, 0.65),
            )));
            world.add(Box::new(Cylinder::new(
                Point3::new(1.4, -0.1, -1.6),
                0.3,
                0.4,
                Color::new(0.2, 0.7, 0.4),
            )));

            let light = PointLight::new(Point3::new(5.0, 5.0, -2.0), Color::new(1.0, 1.0, 1.0));

            let cam = Camera::new(
                Point3::new(0.0, 0.0, 0.0),
                Point3::new(0.0, 0.0, -1.0),
                Vec3::new(0.0, 1.0, 0.0),
                90.0,
                aspect_ratio,
            );

            Scene {
                world,
                light,
                cam,
                aspect_ratio,
            }
        }

        // 4) All objects, different camera (alternate perspective)
        SceneKind::AllAltCam => {
            let mut world = HittableList::new();
            world.add(Box::new(Plane::new(
                Point3::new(0.0, -0.5, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
                Color::new(0.82, 0.82, 0.82),
            )));
            world.add(Box::new(Sphere::new(
                Point3::new(-0.8, 0.0, -1.3),
                0.5,
                Color::new(0.9, 0.2, 0.2),
            )));
            world.add(Box::new(Cube::from_center_size(
                Point3::new(0.3, -0.2, -1.4),
                0.6,
                Color::new(0.35, 0.42, 0.65),
            )));
            world.add(Box::new(Cylinder::new(
                Point3::new(1.4, -0.1, -1.6),
                0.3,
                0.4,
                Color::new(0.2, 0.7, 0.4),
            )));

            let light = PointLight::new(Point3::new(5.0, 5.0, -2.0), Color::new(1.0, 1.0, 1.0));

            // shifted & elevated viewpoint
            let lookfrom = Point3::new(1.6, 0.5, 1.2);
            let lookat = Point3::new(0.1, -0.2, -1.5);
            let cam = Camera::new(
                lookfrom,
                lookat,
                Vec3::new(0.0, 1.0, 0.0),
                75.0,
                aspect_ratio,
            );

            Scene {
                world,
                light,
                cam,
                aspect_ratio,
            }
        }
    }
}

fn main() {
    let args = parse_args();
    let Scene {
        world, light, cam, ..
    } = build_scene(args.scene, args.width, args.height);

    // Output
    let file = File::create(&args.out).expect("Failed to create file");
    let mut w = BufWriter::new(file);
    writeln!(w, "P3").unwrap();
    writeln!(w, "{} {}", args.width, args.height).unwrap();
    writeln!(w, "255").unwrap();

    // Render
    for j in (0..args.height).rev() {
        for i in 0..args.width {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _s in 0..args.samples_per_pixel {
                let u = (i as f64 + math::random_f64()) / (args.width - 1) as f64;
                let v = (j as f64 + math::random_f64()) / (args.height - 1) as f64;
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &world, &light);
            }
            write_color(&mut w, pixel_color, args.samples_per_pixel);
        }
    }
}
