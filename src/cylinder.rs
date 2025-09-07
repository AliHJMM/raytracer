use crate::hittable::{HitRecord, Hittable};
use crate::math::{Color, Point3, Vec3};
use crate::ray::Ray;

/// Finite cylinder aligned with the Y axis, with circular caps.
/// Defined by `center` (at the middle of its height), radius, and half-height `h`.
pub struct Cylinder {
    pub center: Point3,
    pub radius: f64,
    pub half_height: f64,
    pub albedo: Color,
}

impl Cylinder {
    /// Create a cylinder centered at `center`, with total height = 2*h.
    pub fn new(center: Point3, radius: f64, half_height: f64, albedo: Color) -> Self {
        Self {
            center,
            radius,
            half_height,
            albedo,
        }
    }

    #[inline]
    fn y_min(&self) -> f64 {
        self.center.y - self.half_height
    }
    #[inline]
    fn y_max(&self) -> f64 {
        self.center.y + self.half_height
    }
}

impl Hittable for Cylinder {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let eps = 1e-6;

        // Transform ray into cylinder's local space (centered on self.center).
        let ro = r.origin - self.center;
        let rd = r.direction;

        let mut t_hit = t_max;
        let mut hit_rec: Option<HitRecord> = None;

        // ---- 1) Side (infinite cylinder on XZ, clamp y later) ----
        // (ro.x + t*rd.x)^2 + (ro.z + t*rd.z)^2 = r^2
        let a = rd.x * rd.x + rd.z * rd.z;
        if a.abs() > eps {
            let b = ro.x * rd.x + ro.z * rd.z; // using half-b form
            let c = ro.x * ro.x + ro.z * ro.z - self.radius * self.radius;
            let disc = b * b - a * c;
            if disc >= 0.0 {
                let sqrt_d = disc.sqrt();
                // nearest root
                for root in [(-b - sqrt_d) / a, (-b + sqrt_d) / a] {
                    if root > t_min && root < t_hit {
                        let p_local = ro + rd * root;
                        if p_local.y >= self.y_min() - self.center.y - eps
                            && p_local.y <= self.y_max() - self.center.y + eps
                        {
                            let p_world = r.at(root);
                            let outward = Vec3::new(
                                p_world.x - self.center.x,
                                0.0,
                                p_world.z - self.center.z,
                            )
                            .unit();
                            let rec =
                                HitRecord::with_face_normal(r, p_world, outward, root, self.albedo);
                            t_hit = root;
                            hit_rec = Some(rec);
                        }
                    }
                }
            }
        }

        // ---- 2) Caps (disks at y = y_min, y = y_max) ----
        // Intersect with plane y = y_cap, then check distance to center in XZ.
        if rd.y.abs() > eps {
            for (y_cap, normal) in [
                (self.y_min(), Vec3::new(0.0, -1.0, 0.0)),
                (self.y_max(), Vec3::new(0.0, 1.0, 0.0)),
            ] {
                let t = (y_cap - r.origin.y) / rd.y;
                if t > t_min && t < t_hit {
                    let p = r.at(t);
                    let dx = p.x - self.center.x;
                    let dz = p.z - self.center.z;
                    if dx * dx + dz * dz <= self.radius * self.radius + 1e-12 {
                        let rec = HitRecord::with_face_normal(r, p, normal, t, self.albedo);
                        t_hit = t;
                        hit_rec = Some(rec);
                    }
                }
            }
        }

        hit_rec
    }
}
