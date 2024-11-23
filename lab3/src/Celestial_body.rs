use crate::obj::Obj;
use nalgebra_glm::{Vec3, Mat4, Mat3, look_at};
use crate::vertex::Vertex;
pub struct CelestialBody {
    pub vertices: Vec<Vertex>,
    pub translation: Vec3,
    pub rotation: Vec3,
    pub scale: f32,
    pub orbit_speed: f32,
    pub orbit_radius: f32,
    pub orbit_center: Vec3,
    pub shader_option: f32
}

impl CelestialBody {
    pub fn new(obj_path: &str, traslation: Vec3, rotation: Vec3, orbit_radius: f32, scale: f32, orbit_speed: f32, orbit_center: Vec3, shader_option: f32) -> Self {
        let obj = Obj::load(obj_path).expect("Failed to load obj");
        CelestialBody {
            vertices: obj.get_vertex_array(),
            translation: traslation,
            rotation: rotation,
            scale,
            orbit_speed,
            orbit_radius,
            orbit_center,
            shader_option
        }
    }

    pub fn update(&mut self, time: f32) {
        let angle = time * self.orbit_speed;
        self.translation.x = self.orbit_center.x + self.orbit_radius * angle.cos();
        self.translation.z = self.orbit_center.z + self.orbit_radius * angle.sin();
        self.rotation.y += 0.01;
    }
}