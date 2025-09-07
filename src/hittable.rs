use crate::math::{Color, Point3, Vec3};
use crate::ray::Ray;

#[derive(Clone, Copy, Debug)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub albedo: Color,
    pub reflectivity: f64,
}

impl HitRecord {
    pub fn with_face_normal(
        r: &Ray,
        p: Point3,
        outward_normal: Vec3,
        t: f64,
        albedo: Color,
        reflectivity: f64,
    ) -> Self {
        let front = Vec3::dot(r.direction, outward_normal) < 0.0;
        let normal = if front {
            outward_normal
        } else {
            -outward_normal
        };
        Self {
            p,
            normal,
            t,
            albedo,
            reflectivity,
        }
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }
    pub fn add(&mut self, obj: Box<dyn Hittable>) {
        self.objects.push(obj);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut hit_any: Option<HitRecord> = None;

        for obj in &self.objects {
            if let Some(rec) = obj.hit(r, t_min, closest_so_far) {
                closest_so_far = rec.t;
                hit_any = Some(rec);
            }
        }
        hit_any
    }
}
