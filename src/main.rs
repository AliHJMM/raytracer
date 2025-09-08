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
use math::reflect;
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

fn ray_color(r: &Ray, world: &impl Hittable, light: &PointLight, depth: i32) -> Color {
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0); // no contribution when we exceed bounce limit
    }

    if let Some(rec) = world.hit(r, 0.001, f64::INFINITY) {
        // Local shading
        let local = shade_lambert_with_shadow(rec.albedo, rec.normal, rec.p, light, world);

        // Reflection
        let refl = rec.reflectivity.clamp(0.0, 1.0);
        if refl > 0.0 {
            const BIAS: f64 = 1e-4;
            let reflect_dir = reflect(r.direction.unit(), rec.normal).unit();
            let reflect_ray = Ray::new(rec.p + rec.normal * BIAS, reflect_dir);
            let reflected = ray_color(&reflect_ray, world, light, depth - 1);
            return local * (1.0 - refl) + reflected * refl;
        } else {
            return local;
        }
    } // <-- this closes the if-let block

    // Sky
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
    Custom,
}

impl Default for SceneKind {
    fn default() -> Self {
        SceneKind::All
    }
}

#[derive(Default, Clone)]
struct CamOverride {
    lookfrom: Option<Point3>,
    lookat: Option<Point3>,
    vup: Option<Vec3>,
    fov: Option<f64>,
}

#[derive(Default)]
struct Args {
    scene: SceneKind,
    width: i32,
    height: i32,
    out: String,
    samples_per_pixel: i32,

    // NEW: camera override
    cam: CamOverride,

    // NEW: light override
    light_pos: Option<Point3>,
    light_int: Option<Color>,

    // NEW: custom objects (repeatable flags)
    add_spheres: Vec<(Point3, f64, Color, f64)>, // (center, radius, albedo, refl)
    add_planes: Vec<(Point3, Vec3, Color, f64)>, // (point, normal, albedo, refl)
    add_cubes: Vec<(Point3, f64, Color, f64)>,   // (center, size, albedo, refl)
    add_cylinders: Vec<(Point3, f64, f64, Color, f64)>, // (center, radius, half_h, albedo, refl)
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

fn parse_vec3(s: &str) -> Option<Vec3> {
    let s = dequote(s);
    let mut parts = s.split(',').map(|t| t.trim().parse::<f64>());
    let x = parts.next()?.ok()?;
    let y = parts.next()?.ok()?;
    let z = parts.next()?.ok()?;
    Some(Vec3::new(x, y, z))
}

fn clamp01(x: f64) -> f64 {
    x.max(0.0).min(1.0)
}

fn parse_color_clamped01(s: &str) -> Option<Color> {
    parse_vec3(s).map(|v| Color::new(clamp01(v.x), clamp01(v.y), clamp01(v.z)))
}

fn parse_color_nonneg(s: &str) -> Option<Color> {
    parse_vec3(s).map(|v| Color::new(v.x.max(0.0), v.y.max(0.0), v.z.max(0.0)))
}

fn split4(s: &str) -> Option<(&str, &str, &str, &str)> {
    let s = dequote(s);
    let parts: Vec<&str> = s.split(';').map(|t| dequote(t.trim())).collect();
    if parts.len() == 4 {
        Some((parts[0], parts[1], parts[2], parts[3]))
    } else {
        None
    }
}
fn split5(s: &str) -> Option<(&str, &str, &str, &str, &str)> {
    let s = dequote(s);
    let parts: Vec<&str> = s.split(';').map(|t| dequote(t.trim())).collect();
    if parts.len() == 5 {
        Some((parts[0], parts[1], parts[2], parts[3], parts[4]))
    } else {
        None
    }
}

fn dequote(s: &str) -> &str {
    let b = s.as_bytes();
    if s.len() >= 2
        && ((b[0] == b'"' && b[s.len() - 1] == b'"') || (b[0] == b'\'' && b[s.len() - 1] == b'\''))
    {
        &s[1..s.len() - 1]
    } else {
        s
    }
}

fn parse_args() -> Args {
    let mut scene = SceneKind::All;
    let mut width = 400;
    let mut height = 300;
    let mut out: Option<String> = None;
    let mut spp = 16;

    let mut cam = CamOverride::default();
    let mut light_pos: Option<Point3> = None;
    let mut light_int: Option<Color> = None;

    let mut add_spheres: Vec<(Point3, f64, Color, f64)> = Vec::new();
    let mut add_planes: Vec<(Point3, Vec3, Color, f64)> = Vec::new();
    let mut add_cubes: Vec<(Point3, f64, Color, f64)> = Vec::new();
    let mut add_cyls: Vec<(Point3, f64, f64, Color, f64)> = Vec::new();

    for a in std::env::args().skip(1) {
        if let Some(val0) = a.strip_prefix("--scene=") {
            let val = dequote(val0);
            scene = match val {
                "sphere" => SceneKind::Sphere,
                "cube_plane_dim" => SceneKind::CubePlaneDim,
                "all" => SceneKind::All,
                "all_alt_cam" => SceneKind::AllAltCam,
                "custom" => SceneKind::Custom,
                _ => SceneKind::All,
            };
        } else if let Some(val0) = a.strip_prefix("--res=") {
            let val = dequote(val0);
            if let Some((w, h)) = parse_resolution(val) {
                width = w;
                height = h;
            }
        } else if let Some(val0) = a.strip_prefix("--out=") {
            let val = dequote(val0);
            out = Some(val.to_string());
        } else if let Some(val0) = a.strip_prefix("--spp=") {
            let val = dequote(val0);
            if let Ok(v) = val.parse::<i32>() {
                spp = v.max(1);
            }

        // --- camera ---
        } else if let Some(val0) = a.strip_prefix("--lookfrom=") {
            let val = dequote(val0);
            if let Some(v) = parse_vec3(val) {
                cam.lookfrom = Some(Point3::new(v.x, v.y, v.z));
            }
        } else if let Some(val0) = a.strip_prefix("--lookat=") {
            let val = dequote(val0);
            if let Some(v) = parse_vec3(val) {
                cam.lookat = Some(Point3::new(v.x, v.y, v.z));
            }
        } else if let Some(val0) = a.strip_prefix("--vup=") {
            let val = dequote(val0);
            if let Some(v) = parse_vec3(val) {
                cam.vup = Some(v);
            }
        } else if let Some(val0) = a.strip_prefix("--fov=") {
            let val = dequote(val0);
            if let Ok(v) = val.parse::<f64>() {
                cam.fov = Some(v);
            }

        // --- light ---
        } else if let Some(val0) = a.strip_prefix("--light-pos=") {
            let val = dequote(val0);
            if let Some(v) = parse_vec3(val) {
                light_pos = Some(Point3::new(v.x, v.y, v.z));
            }
        } else if let Some(val0) = a.strip_prefix("--light-int=") {
            let val = dequote(val0);
            if let Some(c) = parse_color_nonneg(val) {
                light_int = Some(c);
            }

        // --- objects (repeatable) ---
        } else if let Some(val0) = a.strip_prefix("--add-sphere=") {
            let val = dequote(val0);
            if let Some((p, rad, col, refl)) = split4(val).and_then(|(p, r, c, f)| {
                Some((
                    parse_vec3(dequote(p))?,
                    r.parse::<f64>().ok()?,
                    parse_color_clamped01(dequote(c))?,
                    f.parse::<f64>().ok()?,
                ))
            }) {
                add_spheres.push((Point3::new(p.x, p.y, p.z), rad, col, refl.clamp(0.0, 1.0)));
            }
        } else if let Some(val0) = a.strip_prefix("--add-plane=") {
            let val = dequote(val0);
            if let Some((p, n, col, refl)) = split4(val).and_then(|(p, n, c, f)| {
                Some((
                    parse_vec3(dequote(p))?,
                    parse_vec3(dequote(n))?,
                    parse_color_clamped01(dequote(c))?,
                    f.parse::<f64>().ok()?,
                ))
            }) {
                add_planes.push((Point3::new(p.x, p.y, p.z), n, col, refl.clamp(0.0, 1.0)));
            }
        } else if let Some(val0) = a.strip_prefix("--add-cube=") {
            let val = dequote(val0);
            if let Some((p, size, col, refl)) = split4(val).and_then(|(p, s, c, f)| {
                Some((
                    parse_vec3(dequote(p))?,
                    s.parse::<f64>().ok()?,
                    parse_color_clamped01(dequote(c))?,
                    f.parse::<f64>().ok()?,
                ))
            }) {
                add_cubes.push((Point3::new(p.x, p.y, p.z), size, col, refl.clamp(0.0, 1.0)));
            }
        } else if let Some(val0) = a.strip_prefix("--add-cylinder=") {
            let val = dequote(val0);
            if let Some((p, rad, hh, col, refl)) = split5(val).and_then(|(p, r, hh, c, f)| {
                Some((
                    parse_vec3(dequote(p))?,
                    r.parse::<f64>().ok()?,
                    hh.parse::<f64>().ok()?,
                    parse_color_clamped01(dequote(c))?,
                    f.parse::<f64>().ok()?,
                ))
            }) {
                add_cyls.push((
                    Point3::new(p.x, p.y, p.z),
                    rad,
                    hh,
                    col,
                    refl.clamp(0.0, 1.0),
                ));
            }
        }
    }
    // <-- ADD THIS: closes `for a in std::env::args().skip(1) {`
    // (You were missing this one)

    // If user supplied any custom objects, switch to Custom scene automatically.
    if !add_spheres.is_empty()
        || !add_planes.is_empty()
        || !add_cubes.is_empty()
        || !add_cyls.is_empty()
    {
        scene = SceneKind::Custom;
    }

    let default_out = match scene {
        SceneKind::Sphere => "scene_sphere.ppm",
        SceneKind::CubePlaneDim => "scene_cube_plane_dim.ppm",
        SceneKind::All => "scene_all.ppm",
        SceneKind::AllAltCam => "scene_all_alt_cam.ppm",
        SceneKind::Custom => "scene_custom.ppm",
    }
    .to_string();

    // Return the parsed args
    Args {
        scene,
        width,
        height,
        out: out.unwrap_or(default_out),
        samples_per_pixel: spp,
        cam,
        light_pos,
        light_int,
        add_spheres,
        add_planes,
        add_cubes,
        add_cylinders: add_cyls,
    }
}

struct Scene {
    world: HittableList,
    light: PointLight,
    cam: Camera,
}

fn build_scene(args: &Args) -> Scene {
    let aspect_ratio = args.width as f64 / args.height as f64;

    // Merge scene defaults with CLI overrides
    let light_from = |default_pos: Point3, default_int: Color| -> PointLight {
        PointLight::new(
            args.light_pos.unwrap_or(default_pos),
            args.light_int.unwrap_or(default_int),
        )
    };

    let cam_from = |default_lookfrom: Point3, default_lookat: Point3, default_fov: f64| -> Camera {
        let lf = args.cam.lookfrom.unwrap_or(default_lookfrom);
        let la = args.cam.lookat.unwrap_or(default_lookat);
        let vup = args.cam.vup.unwrap_or(Vec3::new(0.0, 1.0, 0.0));
        let fov = args.cam.fov.unwrap_or(default_fov);
        Camera::new(lf, la, vup, fov, aspect_ratio)
    };

    match args.scene {
        SceneKind::Sphere => {
            let mut world = HittableList::new();
            world.add(Box::new(Plane::new(
                Point3::new(0.0, -0.5, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
                Color::new(0.82, 0.82, 0.82),
                0.15,
            )));
            world.add(Box::new(Sphere::new(
                Point3::new(0.0, 0.0, -1.3),
                0.5,
                Color::new(0.9, 0.2, 0.2),
                0.05,
            )));
            let light = light_from(Point3::new(5.0, 5.0, -2.0), Color::new(1.0, 1.0, 1.0));
            let cam = cam_from(
                Point3::new(0.0, 0.0, 0.0),
                Point3::new(0.0, 0.0, -1.0),
                90.0,
            );

            Scene { world, light, cam }
        }

        // 2) Flat plane + cube with lower brightness than sphere image
        SceneKind::CubePlaneDim => {
            let mut world = HittableList::new();

            world.add(Box::new(Plane::new(
                Point3::new(0.0, -0.5, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
                Color::new(0.82, 0.82, 0.82),
                0.05, // very subtle
            )));
            world.add(Box::new(Cube::from_center_size(
                Point3::new(0.0, -0.2, -1.3),
                0.6,
                Color::new(0.25, 0.28, 0.35),
                0.00, // matte so brightness is clearly lower than the sphere scene
            )));

            let light = light_from(Point3::new(5.0, 5.0, -2.0), Color::new(0.6, 0.6, 0.6));
            let cam = cam_from(
                Point3::new(0.0, 0.0, 0.0),
                Point3::new(0.0, -0.1, -1.3),
                90.0,
            );

            Scene { world, light, cam }
        }

        // 3) All objects
        SceneKind::All => {
            let mut world = HittableList::new();

            world.add(Box::new(Plane::new(
                Point3::new(0.0, -0.5, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
                Color::new(0.82, 0.82, 0.82),
                0.05, // subtle floor reflection
            )));
            world.add(Box::new(Sphere::new(
                Point3::new(-0.8, 0.0, -1.3),
                0.5,
                Color::new(0.9, 0.2, 0.2),
                0.10, // small glossy effect
            )));
            world.add(Box::new(Cube::from_center_size(
                Point3::new(0.3, -0.2, -1.4),
                0.6,
                Color::new(0.35, 0.42, 0.65),
                0.00, // fully matte (keeps shape visible)
            )));
            world.add(Box::new(Cylinder::new(
                Point3::new(1.4, -0.1, -1.6),
                0.3,
                0.4,
                Color::new(0.2, 0.7, 0.4),
                0.05, // very slight gloss
            )));

            let light = light_from(Point3::new(5.0, 5.0, -2.0), Color::new(1.0, 1.0, 1.0));
            let cam = cam_from(
                Point3::new(0.0, 0.0, 0.0),
                Point3::new(0.0, 0.0, -1.0),
                90.0,
            );

            Scene { world, light, cam }
        }

        // 4) All objects, different camera (alternate perspective)
        SceneKind::AllAltCam => {
            let mut world = HittableList::new();

            // Plane – subtle mirror
            // Plane – subtle mirror
            world.add(Box::new(Plane::new(
                Point3::new(0.0, -0.5, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
                Color::new(0.82, 0.82, 0.82),
                0.05,
            )));

            // Sphere – solid red, tiny gloss
            world.add(Box::new(Sphere::new(
                Point3::new(-0.8, 0.0, -1.3),
                0.5,
                Color::new(0.9, 0.2, 0.2),
                0.02, // <- tiny reflection only
            )));

            // Cube – matte
            world.add(Box::new(Cube::from_center_size(
                Point3::new(0.3, -0.2, -1.4),
                0.6,
                Color::new(0.35, 0.42, 0.65),
                0.00, // <- fully matte
            )));

            // Cylinder – light semi-gloss
            world.add(Box::new(Cylinder::new(
                Point3::new(1.4, -0.1, -1.6),
                0.3,
                0.4,
                Color::new(0.2, 0.7, 0.4),
                0.08, // <- very subtle
            )));

            let light = light_from(Point3::new(5.0, 5.0, -2.0), Color::new(1.0, 1.0, 1.0));
            // different viewpoint
            let cam = cam_from(
                Point3::new(1.6, 0.5, 1.2),
                Point3::new(0.1, -0.2, -1.5),
                75.0,
            );

            Scene { world, light, cam }
        }
        SceneKind::Custom => {
            let mut world = HittableList::new();

            for (p, n, col, refl) in &args.add_planes {
                world.add(Box::new(Plane::new(*p, (*n).unit(), *col, *refl)));
            }
            for (c, r, col, refl) in &args.add_spheres {
                world.add(Box::new(Sphere::new(*c, *r, *col, *refl)));
            }
            for (c, size, col, refl) in &args.add_cubes {
                world.add(Box::new(Cube::from_center_size(*c, *size, *col, *refl)));
            }
            for (c, rad, hh, col, refl) in &args.add_cylinders {
                world.add(Box::new(Cylinder::new(*c, *rad, *hh, *col, *refl)));
            }

            // sensible defaults if user didn't add any plane/light/cam
            if args.add_planes.is_empty() {
                world.add(Box::new(Plane::new(
                    Point3::new(0.0, -0.5, 0.0),
                    Vec3::new(0.0, 1.0, 0.0),
                    Color::new(0.82, 0.82, 0.82),
                    0.05,
                )));
            }

            let light = light_from(Point3::new(5.0, 5.0, -2.0), Color::new(1.0, 1.0, 1.0));
            let cam = cam_from(
                Point3::new(0.0, 0.5, 1.0),
                Point3::new(0.0, 0.0, -1.0),
                75.0,
            );

            Scene { world, light, cam }
        }
    }
}

fn main() {
    let max_depth = 5;
    let args = parse_args();
    eprintln!(
        "DEBUG: spheres={} planes={} cubes={} cylinders={}  cam? {}  light? {}",
        args.add_spheres.len(),
        args.add_planes.len(),
        args.add_cubes.len(),
        args.add_cylinders.len(),
        args.cam.lookfrom.is_some() as u8,
        args.light_pos.is_some() as u8
    );
    let Scene { world, light, cam } = build_scene(&args);

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
                pixel_color += ray_color(&r, &world, &light, max_depth);
            }
            write_color(&mut w, pixel_color, args.samples_per_pixel);
        }
    }
}
