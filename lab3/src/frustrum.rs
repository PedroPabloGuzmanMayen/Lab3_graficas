use nalgebra_glm::{Vec3, Vec4, Mat4};
use crate::uniforms::Uniforms;

// Define frustum planes
pub struct Frustum {
    planes: [Vec4; 6], // Left, Right, Top, Bottom, Near, Far
}

impl Frustum {
    pub fn new(projection_view: &Mat4) -> Self {
        let mut planes = [Vec4::new(0.0, 0.0, 0.0, 0.0); 6];
        
        // Extract planes from projection-view matrix
        // Left plane
        planes[0] = projection_view.column(3) + projection_view.column(0);
        // Right plane
        planes[1] = projection_view.column(3) - projection_view.column(0);
        // Bottom plane
        planes[2] = projection_view.column(3) + projection_view.column(1);
        // Top plane
        planes[3] = projection_view.column(3) - projection_view.column(1);
        // Near plane
        planes[4] = projection_view.column(3) + projection_view.column(2);
        // Far plane
        planes[5] = projection_view.column(3) - projection_view.column(2);

        // Normalize planes
        for plane in planes.iter_mut() {
            let normal = Vec3::new(plane.x, plane.y, plane.z);
            let length = normal.magnitude();
            *plane /= length;
        }

        Frustum { planes }
    }

    pub fn is_sphere_visible(&self, center: &Vec3, radius: f32) -> bool {
        // Check if sphere is on positive side of all planes
        for plane in &self.planes {
            let normal = Vec3::new(plane.x, plane.y, plane.z);
            let distance = normal.dot(center) + plane.w;
            
            if distance < -radius {
                return false; // Sphere is completely outside frustum
            }
        }
        true
    }
}

// Add this function to check planet visibility
pub fn is_planet_visible(planet_position: &Vec3, planet_scale: f32, uniforms: &Uniforms) -> bool {
    // Create view-projection matrix
    let vp_matrix = uniforms.projection_matrix * uniforms.view_matrix;
    
    // Create frustum from view-projection matrix
    let frustum = Frustum::new(&vp_matrix);
    
    // Check if planet's bounding sphere is visible
    // Use planet's scale as radius (multiply by a factor if needed for better bounds)
    frustum.is_sphere_visible(planet_position, planet_scale * 1.5)
}