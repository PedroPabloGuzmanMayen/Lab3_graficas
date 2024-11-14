use nalgebra_glm::{Vec2};
use crate::color::Color;

pub struct Fragment {
    pub position: Vec2,
    pub intensity: f32,
    pub depth: f32,
}

impl Fragment {
    pub fn new(x: f32, y: f32, intensity: f32, depth: f32) -> Self {
        Fragment {
            position: Vec2::new(x, y),
            intensity,
            depth,
        }
    }
}