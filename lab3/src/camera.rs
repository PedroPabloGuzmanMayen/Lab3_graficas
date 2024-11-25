use nalgebra_glm::{Vec3, rotate_vec3};
use std::f32::consts::PI;

pub struct Camera {
    pub eye: Vec3,
    pub center: Vec3,
    pub up: Vec3,
    pub has_changed: bool

}

impl Camera {
    pub fn new(eye: Vec3, center: Vec3, up: Vec3, has_changed: bool) -> Self {
        Camera {
            eye,
            center,
            up,
            has_changed
        }
    }

    pub fn basis_change(&self, vector: &Vec3) -> Vec3 {
        let forward = -(self.center - self.eye).normalize();
        let right = forward.cross(&self.up).normalize();
        let up = right.cross(&forward).normalize();

        let rotated = vector.x * right + vector.y*up + vector.z*forward;


        rotated.normalize()

    }

    pub fn orbit(&mut self, delta_yaw: f32, delta_pitch: f32){
        let radius_vector = self.eye - self.center;
        let radius = radius_vector.magnitude();
        
        let current_yaw = radius_vector.z.atan2(radius_vector.x);
        let radius_xz = (radius_vector.x* radius_vector.x + radius_vector.z * radius_vector.z).sqrt();
        let current_pitch = (-radius_vector.y).atan2(radius_xz);
        let new_yaw = (current_yaw + delta_yaw) % (2.0 * PI);
        let new_pitch = (current_pitch + delta_pitch).clamp(-PI/ 2.0 +0.1, PI/ 2.0-0.1);

        let new_eye = self.center + Vec3::new(
            radius * new_yaw.cos() * new_pitch.cos(),
            -radius * new_pitch.sin(),
            radius * new_yaw.sin() * new_pitch.cos()
        );
        self.eye = new_eye;

        self.has_changed = true;

    }

    
    pub fn zoom(&mut self, mut delta_zoom: f32) {
        let direction = (self.center - self.eye).normalize();
        if direction == Vec3::new(0.0, 0.0, 0.0) {
            delta_zoom = 0.0;
        }
    
        // Calculate current distance from center
        let current_distance = (self.center - self.eye).magnitude();
        
        // Límites de zoom, esto se hace para evitar atravesar planetas (10 puntos)
        const MIN_DISTANCE: f32 = 10.0;  
        const MAX_DISTANCE: f32 = 100.0; 
        
        // Calculate new potential distance
        let new_distance = current_distance - delta_zoom;
        
        
        if new_distance >= MIN_DISTANCE && new_distance <= MAX_DISTANCE {
            self.eye += direction * delta_zoom;
            self.has_changed = true;
        }
    }

    pub fn move_center(&mut self, direction: Vec3) {
        // Simply add the direction vector to both the center and eye positions
        self.center += direction;
        self.eye += direction;
        self.has_changed = true;
    }

    pub fn check_if_changed(&mut self) -> bool {
        if self.has_changed {
            self.has_changed = false;
            true
        } else {
            false
        }
    }
}