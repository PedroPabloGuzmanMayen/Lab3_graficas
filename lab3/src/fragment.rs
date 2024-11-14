use nalgebra_glm::{Vec2, Vec3};
use crate::{color::Color, vertex};

pub struct Fragment {
    pub position: Vec2,
    pub intensity: f32,
    pub depth: f32,
    pub vertex_position: Vec3
}

impl Fragment {
    pub fn new(x: f32, y: f32, intensity: f32, depth: f32, vertex_position: Vec3) -> Self {
        Fragment {
            position: Vec2::new(x, y),
            intensity,
            depth,
            vertex_position
        }
    }
}