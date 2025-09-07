use crate::math::{Color, Point3};

pub struct PointLight {
    pub position: Point3,
    pub intensity: Color, // RGB intensity (e.g., (1,1,1) is white light)
}

impl PointLight {
    pub fn new(position: PointLightPos, intensity: Color) -> Self {
        Self {
            position: position.0,
            intensity,
        }
    }
}

// Small helper for clarity when constructing
pub struct PointLightPos(pub Point3);
