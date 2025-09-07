use crate::hittable::{HitRecord, Hittable};
use crate::math::{Color, Point3, Vec3};
use crate::ray::Ray;

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub albedo: Color,
    pub reflectivity: f64, // NEW
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, albedo: Color, reflectivity: f64) -> Self {
        Self {
            center,
            radius,
            albedo,
            reflectivity,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin - self.center;
        let a = Vec3::dot(r.direction, r.direction);
        let half_b = Vec3::dot(oc, r.direction);
        let c = Vec3::dot(oc, oc) - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrt_d = discriminant.sqrt();

        let mut root = (-half_b - sqrt_d) / a;
        if root < t_min || root > t_max {
            root = (-half_b + sqrt_d) / a;
            if root < t_min || root > t_max {
                return None;
            }
        }

        let p = r.at(root);
        let outward_normal = (p - self.center) / self.radius;
        Some(HitRecord::with_face_normal(
            r,
            p,
            outward_normal,
            root,
            self.albedo,
            self.reflectivity,
        ))
    }
}
