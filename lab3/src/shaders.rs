use nalgebra_glm::{Vec3, Vec4};
use crate::vertex::Vertex;
use crate::uniforms::Uniforms;
use crate::fragment::Fragment;
use crate::color::Color;

pub fn vertex_shader(vertex: &Vertex, uniforms: &Uniforms) -> Vertex {
    let position = Vec4::new(
        vertex.position.x,
        vertex.position.y,
        vertex.position.z,
        1.0
    );
    let transformed = uniforms.viewport_matrix * uniforms.projection_matrix * uniforms.view_matrix * uniforms.model_matrix * position;

    let w = transformed.w;
    if w == 0.0 {
        return Vertex {
            position: vertex.position,
            normal: vertex.normal,
            tex_coords: vertex.tex_coords,
            color: vertex.color,
            transformed_position: Vec3::zeros(),
            transformed_normal: vertex.normal,
        };
    }

    let transformed_position = Vec3::new(
        transformed.x/w,
        transformed.y / w,
        transformed.z /w
    );

    Vertex {
        position: vertex.position,
        normal: vertex.normal,
        tex_coords: vertex.tex_coords,
        color: vertex.color,
        transformed_position,
        transformed_normal: vertex.normal
    }
}


pub fn fragment_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color{
    let stripe_width = 0.5; // Adjust the width of the stripes as needed
    let x = fragment.vertex_position.x;
    let mut color = Color::new(255,255,255);

    if (x / stripe_width) as i32 % 2 == 0 {
        color = Color::new(255, 0, 0); // Red stripe
    } else {
        color = Color::new(0, 0, 255); // Blue stripe
    }
    color * fragment.intensity
}
