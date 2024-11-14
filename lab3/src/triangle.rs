use nalgebra_glm::{Vec2, Vec3, dot};
use crate::color::Color;
use crate::framebuffer::FrameBuffer;
use crate::vertex::Vertex;
use crate::fragment::Fragment;
use crate::line::line;

pub fn edge_function(a: &Vec3, b: &Vec3, c:&Vec3) -> f32{
    (c.x -a.x) * (b.y -a.y) - (c.y -a.y) * (b.x -a.x)

}

pub fn calculate_bounding_box(v1: &Vec3, v2:&Vec3, v3:&Vec3)-> (i32, i32, i32, i32){
    let min_x = v1.x.min(v2.x).min(v3.x).floor() as i32;
    let min_y = v1.y.min(v2.y).min(v3.y).floor() as i32;
    let max_x = v1.x.max(v2.x).max(v3.x).floor() as i32;
    let max_y = v1.y.max(v2.y).max(v3.y).floor() as i32;
    (min_x, min_y, max_x, max_y)
}

pub fn barycentric_coordinates(a: &Vec3, b: &Vec3, c: &Vec3, p: &Vec3, area: f32) -> (f32, f32, f32) {
    (edge_function(b, c, p) / area, edge_function(c, a, p) / area, edge_function(a, b,p) / area)

}

pub fn triangle(v1: &Vertex, v2: &Vertex, v3: &Vertex) -> Vec<Fragment> {
    let mut fragments = Vec::new();
    let (a,b,c) = (v1.transformed_position, v2.transformed_position, v3.transformed_position);
    let (min_x, min_y, max_x, max_y) = calculate_bounding_box(&a, &b, &c);
    let area = edge_function(&a, &b, &c);
    let light_dir = Vec3::new(0.0,0.0, -1.0).normalize();
    let color_a = Color::new(255,0,0);
    let color_b = Color::new(0,255,0);
    let color_c = Color::new(0,0,255);
    for y in min_y..max_y{
        for x in min_x..max_x{
            let point = Vec3::new(x as f32, y as f32, 0.0);
            
            let (u,v,w) = barycentric_coordinates(&a, &b, &c, &point, area);
            let depth = a.z * u + b.z*v + c.z*w;
            let base_color = Color::new(100, 100, 100);
            //let base_color = color_a * u+ color_b * v + color_c * w;
           
            if u >= 0.0 && u <= 1.0 && v >= 0.0 && v<= 1.0  && w >= 0.0 && w <= 1.0{
                let normal = v1.normal * u + v2.normal * v + v3.normal * w;
                let normal = normal.normalize();
                let intensity = dot(&normal, &light_dir);
                /* 
                if intensity < 0.0{
                    continue;
                }
                */
                let lit_color = base_color * intensity;
                
                fragments.push(Fragment::new(point.x, point.y, intensity, depth));
            }
        }
    }

    fragments
}