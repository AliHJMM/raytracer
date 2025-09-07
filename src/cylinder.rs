use crate::hittable::{HitRecord, Hittable};
use crate::math::{Color, Point3, Vec3};
use crate::ray::Ray;

pub struct Cylinder {
    pub center: Point3, // center of the cylinder (at mid-height)
    pub radius: f64,
    pub half_height: f64, // height/2 along +Y / -Y
    pub albedo: Color,
    pub reflectivity: f64,
}

impl Cylinder {
    pub fn new(
        center: Point3,
        radius: f64,
        half_height: f64,
        albedo: Color,
        reflectivity: f64,
    ) -> Self {
        Self {
            center,
            radius,
            half_height,
            albedo,
            reflectivity,
        }
    }
}

impl Hittable for Cylinder {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        // Transform ray into cylinder's local space (axis = Y, centered at self.center)
        let ro = r.origin - self.center;
        let rd = r.direction;

        let mut t_hit = t_max;
        let mut hit_rec: Option<HitRecord> = None;

        // ----- Side intersection: x^2 + z^2 = r^2 -----
        // Solve (ro.x + t*rd.x)^2 + (ro.z + t*rd.z)^2 = radius^2
        let a = rd.x * rd.x + rd.z * rd.z;
        if a.abs() > 1e-12 {
            let b = 2.0 * (ro.x * rd.x + ro.z * rd.z);
            let c = ro.x * ro.x + ro.z * ro.z - self.radius * self.radius;
            let disc = b * b - 4.0 * a * c;
            if disc >= 0.0 {
                let sqrt_d = disc.sqrt();

                // try near root first
                let mut try_root = |t: f64| {
                    if t >= t_min && t <= t_hit {
                        let y = ro.y + t * rd.y;
                        if y >= -self.half_height - 1e-9 && y <= self.half_height + 1e-9 {
                            let p = r.at(t);
                            // local normal (x, 0, z) normalized
                            let outward = Vec3::new(
                                (ro.x + t * rd.x) / self.radius,
                                0.0,
                                (ro.z + t * rd.z) / self.radius,
                            )
                            .unit();
                            let rec = HitRecord::with_face_normal(
                                r,
                                p,
                                outward,
                                t,
                                self.albedo,
                                self.reflectivity,
                            );
                            t_hit = t;
                            hit_rec = Some(rec);
                        }
                    }
                };

                let t1 = (-b - sqrt_d) / (2.0 * a);
                try_root(t1);
                let t2 = (-b + sqrt_d) / (2.0 * a);
                try_root(t2);
            }
        }

        // ----- Caps: planes at y = ±half_height -----
        let check_cap = |y_cap: f64,
                         r: &Ray,
                         ro: Vec3,
                         rd: Vec3,
                         this: &Cylinder,
                         t_min: f64,
                         t_hit: &mut f64,
                         hit_rec: &mut Option<HitRecord>| {
            const EPS: f64 = 1e-12;
            if rd.y.abs() < EPS {
                return;
            } // ray parallel to cap plane
            let t = (y_cap - ro.y) / rd.y;
            if t < t_min || t > *t_hit {
                return;
            }
            let x = ro.x + t * rd.x;
            let z = ro.z + t * rd.z;
            if x * x + z * z <= this.radius * this.radius + 1e-9 {
                let p = r.at(t);
                // outward normal: up for top cap, down for bottom cap
                let outward = if y_cap > 0.0 {
                    Vec3::new(0.0, 1.0, 0.0)
                } else {
                    Vec3::new(0.0, -1.0, 0.0)
                };
                let rec =
                    HitRecord::with_face_normal(r, p, outward, t, this.albedo, this.reflectivity);
                *t_hit = t;
                *hit_rec = Some(rec);
            }
        };

        check_cap(
            self.half_height,
            r,
            ro,
            rd,
            self,
            t_min,
            &mut t_hit,
            &mut hit_rec,
        );
        check_cap(
            -self.half_height,
            r,
            ro,
            rd,
            self,
            t_min,
            &mut t_hit,
            &mut hit_rec,
        );

        hit_rec
    }
}
