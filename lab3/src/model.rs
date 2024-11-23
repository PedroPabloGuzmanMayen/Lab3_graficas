use nalgebra_glm::{Vec3, Mat4};
use crate::obj::Obj;
use crate::color::Color;
use fastnoise_lite::{FastNoiseLite, NoiseType, FractalType, CellularDistanceFunction, CellularReturnType};
use crate::vertex::Vertex;
use crate::uniforms::Uniforms;
use crate::fragment::Fragment;

pub struct Model {
    pub model: Obj,
    pub fragment_shader: fn(fragment: &Fragment, uniforms: &Uniforms) -> Color,
    pub traslation: Vec3,
    pub rotation: Vec3,
    pub scale: f32,
    pub traslation_speed: f32,
    pub rotation_speed: f32,
    pub noise: FastNoiseLite

}

impl Model {
    pub fn new(model: Obj, 
        fragment_shader: fn(fragment: &Fragment, uniforms: &Uniforms) -> Color, 
        traslation: Vec3, 
        rotation: Vec3, 
        scale:f32, 
        traslation_speed: f32, 
        rotation_speed: f32, 
        noise: FastNoiseLite) -> Model{
        Model {
            model,
            fragment_shader,
            traslation,
            rotation,
            scale,
            traslation_speed,
            rotation_speed,
            noise
        }
    }
}
