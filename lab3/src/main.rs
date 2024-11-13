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
use uniforms::{Uniforms, create_projection_matrix, create_viewport_matrix};
use triangle::triangle;
use color::Color;
use obj::Obj;
use camera::Camera;

fn render(framebuffer: &mut FrameBuffer, uniforms: &Uniforms, vertex_array: &[Vertex]) {


  // Transform vertices
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
  for (index, tri) in triangles.iter().enumerate() {
      let tri_fragments = triangle(&tri[0], &tri[1], &tri[2]);

      fragments.extend(tri_fragments);
  }


  // Render
  let mut drawn_fragments = 0;
  for fragment in fragments {
      let x = fragment.position.x as usize;
      let y = fragment.position.y as usize;

      // Bounds check
      if x < framebuffer.width && y < framebuffer.height {
          framebuffer.set_current_color(fragment.color);
          framebuffer.point(x, y, fragment.depth);
          drawn_fragments += 1;
      }
  }

}


fn main() {
    let window_width = 800;
    let window_height = 600;
    let framebuffer_width = 800;
    let framebuffer_height = 600;

    println!("--- Initializing Camera ---");
    let mut camera = Camera::new(
        Vec3::new(5.0, 5.0, 1.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        false,
    );

    println!("--- Creating Projection Matrix ---");
    let projection_matrix = create_projection_matrix(window_width as f32, window_height as f32);
    let viewport_matrix = create_viewport_matrix(framebuffer_width as f32, framebuffer_height as f32);

    println!("--- Initializing Framebuffer ---");
    let mut framebuffer = FrameBuffer::new(framebuffer_width, framebuffer_height);
    let mut window = Window::new(
        "Rust Graphics - Renderer Example",
        window_width,
        window_height,
        WindowOptions::default(),
    )
    .expect("Failed to create window");

    window.set_position(500, 500);
    window.update();

    framebuffer.set_background_color(Color::new(0, 51, 51));

    println!("--- Loading OBJ File ---");
    let obj = Obj::load("assets/ship.obj").expect("Failed to load obj");
    let array = obj.get_vertex_array();
    println!("Loaded {} vertices from OBJ file", array.len());

    let mut translation = Vec3::new(0.0, 0.0, 0.0);
    let mut rotation = Vec3::new(0.0, 0.0, 0.0);
    let mut scale = 1.0f32;

    while window.is_open() {
        if window.is_key_down(Key::Escape) {
            println!("Escape key pressed, exiting...");
            break;
        }

        handle_input(&window, &mut translation, &mut rotation, &mut scale, &mut camera);
        framebuffer.clear();

        let uniform = Uniforms::new(
            translation,
            scale,
            rotation,
            &camera.eye,
            &camera.center,
            &camera.up,
            projection_matrix,
            viewport_matrix,
        );


        render(&mut framebuffer, &uniform, &array);

        window
            .update_with_buffer(&framebuffer.cast_buffer(), framebuffer_width, framebuffer_height)
            .expect("Failed to update window");
    }

}

fn handle_input(window: &Window, translation: &mut Vec3, rotation: &mut Vec3, scale: &mut f32, camera: &mut Camera) {
    let movement_speed = 0.1;
    let rotation_speed = PI / 50.0;
    let zoom_speed = 0.6;
    let mut movement = Vec3::new(0.0, 0.0, 0.0);

    if window.is_key_down(Key::Left) {

        camera.orbit(rotation_speed, 0.0);
    }
    if window.is_key_down(Key::Right) {

        camera.orbit(-rotation_speed, 0.0);
    }
    if window.is_key_down(Key::W) {

        camera.orbit(0.0, -rotation_speed);
    }
    if window.is_key_down(Key::S) {

        camera.orbit(0.0, rotation_speed);
    }
    if window.is_key_down(Key::Up) {

        camera.zoom(zoom_speed);
    }
    if window.is_key_down(Key::Down) {

        camera.zoom(-zoom_speed);
    }

    if window.is_key_down(Key::A){
        movement.x -= movement_speed;
    }
    if window.is_key_down(Key::D){
        movement.x += movement_speed;
    }
    if window.is_key_down(Key::Q){
        movement.y += movement_speed;
    }
    if window.is_key_down(Key::E){
        movement.y -= movement_speed;
    }

    if movement.magnitude() > 0.0 {
        camera.move_center(movement);
    }
}
