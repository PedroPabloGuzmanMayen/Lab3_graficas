use nalgebra_glm::{Vec3, Mat4, Mat3, look_at};
use minifb::{Key, Window, WindowOptions};
use std::f32::consts::PI;

mod framebuffer;
mod triangle;
mod line;
mod vertex;
mod color;
mod fragment;
mod bmp;
mod uniforms;
mod shaders;
mod obj;
mod camera;
use framebuffer::FrameBuffer;
use vertex::Vertex;
use shaders::vertex_shader;
use uniforms::Uniforms;
use triangle::triangle;
use color::Color;
use obj::Obj;
use camera::Camera;


fn render(framebuffer: &mut FrameBuffer, uniforms: &Uniforms, vertex_array: &[Vertex]) {
  // transform
  let mut transformed_vertices = Vec::with_capacity(vertex_array.len());
  for vertex in vertex_array {
    let transformed = vertex_shader(vertex, uniforms);

    transformed_vertices.push(transformed);
  }


  // Primitive Assembly Stage
  let mut triangles = Vec::new();
  for i in (0..transformed_vertices.len()).step_by(3) {
    if i + 2 < transformed_vertices.len() {
      triangles.push([
        transformed_vertices[i].clone(),
        transformed_vertices[i + 1].clone(),
        transformed_vertices[i + 2].clone(),
      ]);
    }
  }

  // Rasterization Stage
  let mut fragments = Vec::new();
  for tri in &triangles {
    fragments.extend(triangle(&tri[0], &tri[1], &tri[2]));
  }

  // render
  for fragment in fragments {
    let x = fragment.position.x as usize;
    let y = fragment.position.y as usize;
    if x < framebuffer.width && y < framebuffer.height{
      framebuffer.set_current_color(fragment.color);
      framebuffer.point(x, y, fragment.depth);
  }
}
}

fn main() {
  let window_width = 800;
  let window_height = 600;
  let framebuffer_width = 800;
  let framebuffer_height = 600;

  let mut camera = Camera::new(
    Vec3::new(0.0, 0.0, 5.0),
    Vec3::new(0.0, 0.0, 0.0),
    Vec3::new(0.0, 1.0, 0.0),
    false
  );

  let mut framebuffer = FrameBuffer::new(framebuffer_width, framebuffer_height);
  let mut window = Window::new(
    "Rust Graphics - Renderer Example",
    window_width,
    window_height,
    WindowOptions::default(),
  )
    .unwrap();

  window.set_position(500, 500);
  window.update();

  framebuffer.set_background_color(Color::new(0,51,51));

  let vertex_arrays = [
    Vertex::new_with_color(Vec3::new(20.0, 20.0, 0.0), Color::new(255, 0, 0)),
    Vertex::new_with_color(Vec3::new(20.0, 100.0, 0.0), Color::new(0, 255, 0)),
    Vertex::new_with_color(Vec3::new(100.0, 100.0, 0.0), Color::new(0, 0, 255)),
    Vertex::new_with_color(Vec3::new(100.0, 100.0, 0.0), Color::new(0, 255, 0)),
    Vertex::new_with_color(Vec3::new(20.0, 20.0, 0.0), Color::new(255, 0, 0)),
    Vertex::new_with_color(Vec3::new(100.0, 20.0, 0.0), Color::new(0, 0, 255))
  ];

  let mut translation = Vec3::new(400.0, 300.0, 0.0);
  let mut rotation = Vec3::new(0.0, 0.0, 0.0);
  let mut scale = 50.0f32;
  let obj = Obj::load("assets/ship.obj").expect("Failed t load obj");
  let array = obj.get_vertex_array();

  


  while window.is_open() {
    if window.is_key_down(Key::Escape) {
      break;
    }
    handle_input(&window, &mut translation, &mut rotation, &mut scale);

    framebuffer.clear();
    let uniform = Uniforms::new(translation, scale, rotation, &camera.eye, &camera.center, &camera.up);

    render(&mut framebuffer, &uniform, &array);

    window
      .update_with_buffer(&framebuffer.cast_buffer(), framebuffer_width, framebuffer_height)
      .unwrap();
  }
}

fn handle_input(window: &Window, translation: &mut Vec3, rotation: &mut Vec3, scale: &mut f32) {
  if window.is_key_down(Key::Right) {
    translation.x += 10.0;
  }
  if window.is_key_down(Key::Left) {
    translation.x -= 10.0;
  }
  if window.is_key_down(Key::Up) {
    translation.y -= 10.0;
  }
  if window.is_key_down(Key::Down) {
    translation.y += 10.0;
  }
  if window.is_key_down(Key::S) {
    *scale += 0.1;
  }
  if window.is_key_down(Key::A) {
    *scale -= 0.1;
  }
  if window.is_key_down(Key::Q) {
    rotation.z -= PI / 10.0;
  }
  if window.is_key_down(Key::W){
    rotation.x += PI /10.0;

  }

  if window.is_key_down(Key::D){
    rotation.y += PI /10.0;
  }
}
