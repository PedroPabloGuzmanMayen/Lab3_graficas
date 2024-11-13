use nalgebra_glm::{Vec3, Mat4, Mat3, look_at, perspective};
use std::f32::consts::PI;
pub struct Uniforms {
    pub model_matrix:Mat4,
    pub view_matrix:Mat4,
    pub projection_matrix: Mat4,
    pub viewport_matrix: Mat4
}

impl Uniforms {
    pub fn new(translation: Vec3, scale:f32, rotation: Vec3, eye: &Vec3, center: &Vec3, up: &Vec3, perspective_matrix: Mat4, viewport_matrix: Mat4 ) -> Uniforms{
        let model_matrix = create_model_matrix(translation, scale, rotation);
        let view_matrix = create_view_matrix(eye, center, up);
        Uniforms {model_matrix: model_matrix, view_matrix: view_matrix, projection_matrix: perspective_matrix, viewport_matrix }
    }

    
}

pub fn create_model_matrix(translation: Vec3, scale:f32, rotation: Vec3) -> Mat4{
    let (sin_x, cos_x) = rotation.x.sin_cos();
    let (sin_y, cos_y) = rotation.y.sin_cos();
    let (sin_z, cos_z) = rotation.z.sin_cos();

    let rotation_matrix_x = Mat4::new(
        1.0, 0.0, 0.0, 0.0,
        0.0, cos_x, -sin_x, 0.0,
        0.0, sin_x, cos_x, 0.0,
        0.0, 0.0, 0.0, 1.0
    );

    let rotation_matrix_y = Mat4::new(
        cos_y, 0.0, sin_y, 0.0,
        0.0, 1.0, 0.0, 0.0,
        -sin_y, 0.0, cos_y, 0.0,
        0.0, 0.0, 0.0, 1.0
    );

    let rotation_matrix_z = Mat4::new(
        cos_z, -sin_z, 0.0, 0.0,
        sin_z, cos_z, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0
    );

    let rotation_matrix = rotation_matrix_x* rotation_matrix_y*rotation_matrix_z;

    let transform_matrix = Mat4::new(
        scale, 0.0, 0.0, translation.x,
        0.0, scale, 0.0, translation.y,
        0.0, 0.0, scale, translation.z,
        0.0, 0.0, 0.0, 1.0
    );

    transform_matrix*rotation_matrix
}

pub fn create_view_matrix(eye: &Vec3, center: &Vec3, up: &Vec3) -> Mat4{
    look_at(eye, center, up)
}

pub fn create_projection_matrix(width: f32, height: f32) -> Mat4{
    let fov = 45.0 * PI / 180.0;
    let aspect_ratio = width/height;
    let near= 0.1;
    let far = 1000.0;

    perspective(fov, aspect_ratio, near, far)

}

pub fn create_viewport_matrix(width: f32, height: f32) -> Mat4 {
    Mat4::new(
        width / 2.0, 0.0, 0.0, width / 2.0,
        0.0, -height / 2.0, 0.0, height / 2.0,
        0.0, 0.0, 128.0, 0.0,
        0.0, 0.0, 0.0, 1.0
    )
}