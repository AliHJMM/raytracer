use crate::hittable::{HitRecord, Hittable};
use crate::math::{Color, Point3, Vec3};
use crate::ray::Ray;

pub struct Aabb {
    pub min: Point3,
    pub max: Point3,
}

impl Aabb {
    pub fn new(min: Point3, max: Point3) -> Self {
        Self { min, max }
    }

    // Slab test: returns (tmin, face_normal) for the first hit within [t_min, t_max]
    fn hit(&self, r: &Ray, mut t_min: f64, mut t_max: f64) -> Option<(f64, Vec3)> {
        let mut face_normal = Vec3::new(0.0, 0.0, 0.0);

        // For each axis, update interval and record which face we entered from
        for axis in 0..3 {
            let (origin, dir, minb, maxb, normal_neg, normal_pos) = match axis {
                0 => (
                    r.origin.x,
                    r.direction.x,
                    self.min.x,
                    self.max.x,
                    Vec3::new(-1.0, 0.0, 0.0),
                    Vec3::new(1.0, 0.0, 0.0),
                ),
                1 => (
                    r.origin.y,
                    r.direction.y,
                    self.min.y,
                    self.max.y,
                    Vec3::new(0.0, -1.0, 0.0),
                    Vec3::new(0.0, 1.0, 0.0),
                ),
                _ => (
                    r.origin.z,
                    r.direction.z,
                    self.min.z,
                    self.max.z,
                    Vec3::new(0.0, 0.0, -1.0),
                    Vec3::new(0.0, 0.0, 1.0),
                ),
            };

            if dir.abs() < 1e-12 {
                // Ray parallel to slabs: must be inside this axis range
                if origin < minb || origin > maxb {
                    return None;
                }
                continue;
            }

            let inv = 1.0 / dir;
            let mut t0 = (minb - origin) * inv;
            let mut t1 = (maxb - origin) * inv;
            let mut enter_normal = normal_neg;
            let _exit_normal = normal_pos;

            if t0 > t1 {
                std::mem::swap(&mut t0, &mut t1);
                enter_normal = -enter_normal; // flipped because we swapped
            }

            if t0 > t_min {
                t_min = t0;
                face_normal = enter_normal;
            }
            if t1 < t_max {
                t_max = t1;
            }

            if t_max <= t_min {
                return None;
            }
        }

        Some((t_min, face_normal))
    }
}

pub struct Cube {
    pub bounds: Aabb,
    pub albedo: Color,
    pub reflectivity: f64,
}

impl Cube {
    /// Construct from center and edge size
    pub fn from_center_size(center: Point3, size: f64, albedo: Color, reflectivity: f64) -> Self {
        let h = size * 0.5;
        let min = Point3::new(center.x - h, center.y - h, center.z - h);
        let max = Point3::new(center.x + h, center.y + h, center.z + h);
        Self {
            bounds: Aabb::new(min, max),
            albedo,
            reflectivity,
        }
    }
}

impl Hittable for Cube {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if let Some((t, outward_normal)) = self.bounds.hit(r, t_min, t_max) {
            let p = r.at(t);
            return Some(HitRecord::with_face_normal(
                r,
                p,
                outward_normal,
                t,
                self.albedo,
                self.reflectivity,
            ));
        }
        None
    }
}
