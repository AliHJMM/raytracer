use crate::hittable::{HitRecord, Hittable};
use crate::math::{Color, Point3, Vec3};
use crate::ray::Ray;

pub struct Plane {
    pub point: Point3, // a point on the plane
    pub normal: Vec3,  // plane normal (doesn't need to be unit; we'll normalize)
    pub albedo: Color,
}

impl Plane {
    pub fn new(point: Point3, normal: Vec3, albedo: Color) -> Self {
        Self {
            point,
            normal: normal.unit(),
            albedo,
        }
    }
}

impl Hittable for Plane {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        // Ray-plane: t = ((p0 - ro) · n) / (rd · n)
        let denom = Vec3::dot(r.direction, self.normal);
        if denom.abs() < 1e-6 {
            return None;
        } // parallel

        let t = Vec3::dot(self.point - r.origin, self.normal) / denom;
        if t < t_min || t > t_max {
            return None;
        }

        let p = r.at(t);
        Some(HitRecord::with_face_normal(
            r,
            p,
            self.normal,
            t,
            self.albedo,
        ))
    }
}
