use nalgebra_glm::{Vec3, Mat4, Vec4};

pub struct Frustum {
    planes: [Vec4; 6], // 6 planes: left, right, top, bottom, near, far
}

impl Frustum {
    pub fn new(view_matrix: &Mat4, projection_matrix: &Mat4) -> Self {
        let vp = projection_matrix * view_matrix;
        let planes = [
            (vp.column(3) + vp.column(0)).normalize(), // Left
            (vp.column(3) - vp.column(0)).normalize(), // Right
            (vp.column(3) + vp.column(1)).normalize(), // Bottom
            (vp.column(3) - vp.column(1)).normalize(), // Top
            (vp.column(3) + vp.column(2)).normalize(), // Near
            (vp.column(3) - vp.column(2)).normalize(), // Far
        ];

        Frustum { planes }
    }

    pub fn is_point_inside(&self, point: &Vec3) -> bool {
        for plane in &self.planes {
            let distance = plane.x * point.x + plane.y * point.y + plane.z * point.z + plane.w;
            if distance < 0.0 {
                return false;
            }
        }
        true
    }

    pub fn is_sphere_inside(&self, center: &Vec3, radius: f32) -> bool {
        for plane in &self.planes {
            let distance = plane.x * center.x + plane.y * center.y + plane.z * center.z + plane.w;
            if distance < -radius {
                return false;
            }
        }
        true
    }
}
