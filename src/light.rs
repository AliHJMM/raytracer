use crate::math::{Color, Point3};

pub struct PointLight {
    pub position: Point3,
    pub intensity: Color, // (1,1,1) = white light
}

impl PointLight {
    // ⬅ change signature to take Point3 directly
    pub fn new(position: Point3, intensity: Color) -> Self {
        Self {
            position,
            intensity,
        }
    }
}
