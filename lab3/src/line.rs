use nalgebra_glm::{Vec2, Vec3};
use crate::framebuffer::FrameBuffer;
use crate::vertex::Vertex;
use crate::fragment::Fragment;
use crate::color::Color;

pub fn line(a: &Vertex, b:&Vertex) -> Vec<Fragment> {
    let mut fragments = Vec::new();
    let mut x = a.position.x as isize;
    let mut y = a.position.y as isize;
    let mut dx = (b.position.x as isize - a.position.x as isize).abs();
    let mut dy = (b.position.y as isize - a.position.y as isize).abs();
    let sx = if a.position.x < b.position.x {1} else {-1};
    let sy = if a.position.y < b.position.y {1} else {-1};
    let mut err = if dx > dy { dx / 2 } else { -dy / 2 };

    loop {
        fragments.push(Fragment::new(x as f32, y as f32, 0.0, 0.0, Vec3::new(0.0, 0.0, 0.0)));
        if x == b.position.x as isize && y == b.position.y as isize { break; }
        let e2 = err;
        if e2 > -dx {
            err -= dy;
            x += sx;
        }
        if e2 < dy {
            err += dx;
            y += sy;
        }
    }

    fragments
}