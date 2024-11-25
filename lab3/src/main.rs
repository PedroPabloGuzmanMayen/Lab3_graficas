use nalgebra_glm::{Vec3, Mat4, Mat3, look_at, Vec4, Vec2};
use minifb::{Key, Window, WindowOptions, MouseButton, MouseMode};
use std::{f32::consts::PI};
use fastnoise_lite::{FastNoiseLite, NoiseType, FractalType, CellularDistanceFunction, CellularReturnType};
use image::{open, DynamicImage, GenericImageView};

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
mod frustrum;
mod Celestial_body;
mod music;
mod texture;
mod skybox;

use music::AudioPlayer;
use Celestial_body::CelestialBody;
use framebuffer::FrameBuffer;
use frustrum::{Frustum, is_planet_visible};
use vertex::Vertex;
use shaders::{vertex_shader, fragment_shader, sun_shader, combined_shader};
use uniforms::{Uniforms, create_projection_matrix, create_viewport_matrix, create_model_matrix, create_view_matrix};
use triangle::triangle;
use color::Color;
use obj::Obj;
use camera::Camera;
use skybox::Skybox;
use rayon::prelude::*;
use std::sync::Mutex;
use std::sync::Arc;

const ROTATION_SPEED: f32 = PI/450.0;

//Angle: el ángulo en el que se bica el planeta con respecto al centro del mundo
//Radius: El radio de la trayectoria de la órbita del planeta

pub fn traslaton_movement(angle: f32, radius: f32) -> (f32, f32){
    (radius * angle.cos(), radius * angle.sin())
}

fn create_noise(option: usize) -> FastNoiseLite{
    let mut noise = FastNoiseLite::new();
    //Estrella del sistema
    if (option == 1){
        noise.set_noise_type(Some(NoiseType::Perlin)); 
        noise.set_fractal_type(Some(FractalType::Ridged)); 
        noise.set_frequency(Some(0.02)); 
        noise.set_fractal_octaves(Some(5)); 
        noise.set_fractal_gain(Some(0.5)); 
        noise.set_fractal_lacunarity(Some(2.0)); 
        noise.set_seed(Some(42));
    }
    //Planeta rocoso
    if (option == 2){
        noise.set_noise_type(Some(NoiseType::Perlin)); 
        noise.set_fractal_type(Some(FractalType::Ridged)); 
        noise.set_frequency(Some(0.1)); 
        noise.set_fractal_octaves(Some(5)); 
        noise.set_fractal_gain(Some(0.6)); 
        noise.set_fractal_lacunarity(Some(2.5)); 
        noise.set_seed(Some(42)); 

    }
    //Planeta gaseoso
    if (option == 3){
        noise.set_noise_type(Some(NoiseType::Perlin));
        noise.set_fractal_type(Some(FractalType::Ridged));
        noise.set_frequency(Some(0.05)); 
        noise.set_fractal_octaves(Some(6)); 
        noise.set_fractal_gain(Some(0.5)); 
        noise.set_fractal_lacunarity(Some(2.0)); 
        noise.set_seed(Some(42));

    }
    //Asteroide (otro cuerpo celeste, 10 pts)
    if (option == 4){

        noise.set_fractal_type(Some(FractalType::FBm)); 
        noise.set_frequency(Some(0.05)); 
        noise.set_fractal_octaves(Some(6)); 
        noise.set_fractal_gain(Some(0.4));
        noise.set_fractal_lacunarity(Some(2.2)); 
        noise.set_noise_type(Some(NoiseType::Cellular)); 
        noise.set_frequency(Some(0.08));
        noise.set_seed(Some(123))

    }
    //Planeta de diamante
    if (option == 5){
        noise.set_noise_type(Some(NoiseType::Perlin)); 
        noise.set_fractal_type(Some(FractalType::Ridged)); 
        noise.set_frequency(Some(0.02)); 
        noise.set_fractal_octaves(Some(5)); 
        noise.set_fractal_gain(Some(0.5)); 
        noise.set_fractal_lacunarity(Some(2.0)); 
        noise.set_seed(Some(42));
    }
    //Planeta de piel humana
    if (option == 6){
        noise.set_noise_type(Some(NoiseType::OpenSimplex2)); 
        noise.set_fractal_type(Some(FractalType::FBm)); 
        noise.set_frequency(Some(0.3)); // Adjust for smoother, finer texture
        noise.set_fractal_octaves(Some(6)); 
        noise.set_fractal_gain(Some(0.4)); // Lower gain for subtle transitions
        noise.set_fractal_lacunarity(Some(2.0)); 
        noise.set_seed(Some(101));

    }
    //Planeta con ciudades
    if (option == 7){
        noise.set_noise_type(Some(NoiseType::Perlin));
        noise.set_fractal_type(Some(FractalType::Ridged));
        noise.set_frequency(Some(0.02));
        noise.set_fractal_octaves(Some(5));
        noise.set_fractal_gain(Some(0.5));
        noise.set_fractal_lacunarity(Some(2.0));
        noise.set_seed(Some(42));

    }

    

    noise
}

fn render(framebuffer: &mut FrameBuffer, uniforms: &Uniforms, vertex_array: &[Vertex], option: usize) {
  //skybox.render(framebuffer, &uniforms);
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
  for fragment in fragments {
      let x = fragment.position.x as usize;
      let y = fragment.position.y as usize;

      // Bounds check
      if x < framebuffer.width && y < framebuffer.height {
        let shaded_color = fragment_shader(&fragment, uniforms, option as f32);
          framebuffer.set_current_color(shaded_color);
          framebuffer.point(x, y, fragment.depth);
      }
  }

}

fn render_bodies_parallel(
    celestial_bodies: &mut Vec<CelestialBody>,
    framebuffer: &mut FrameBuffer,
    uniform: &Arc<Mutex<Uniforms>>,
    camera: &Camera,
    time: f32
) {
    // First update all positions in parallel - this doesn't need the uniform
    celestial_bodies.par_iter_mut().for_each(|body| {
        body.initial_angle += ROTATION_SPEED;
        let (x, y) = traslaton_movement(body.initial_angle, body.orbit_radius);
        body.translation.x = x;
        body.translation.z = y;
        body.rotation.y += body.orbit_speed;
    });

    // Create thread-local framebuffers
    let body_renders: Vec<Option<FrameBuffer>> = celestial_bodies
        .par_iter()
        .map(|body| {
            let planet_position = Vec3::new(body.translation.x, body.translation.y, body.translation.z);
            
            // Lock uniform just long enough to check visibility
            /* 
            let is_visible = {
                let uniform_guard = uniform.lock().unwrap();
                is_planet_visible(&planet_position, body.scale, &uniform_guard)
            };

            if !is_visible {
                return None;
            }
            */
            // Create a local framebuffer for this planet
            let mut local_framebuffer = FrameBuffer::new(framebuffer.width, framebuffer.height);
            
            // Lock uniform for rendering
            {
                let mut uniform_guard = uniform.lock().unwrap();
                uniform_guard.model_matrix = create_model_matrix(
                    body.translation,
                    body.scale,
                    body.rotation
                );
                uniform_guard.view_matrix = create_view_matrix(&camera.eye, &camera.center, &camera.up);
                uniform_guard.time = time + 1.0;
                uniform_guard.noise = create_noise(body.shader_option as usize);

                render(&mut local_framebuffer, &uniform_guard, &body.vertices, body.shader_option as usize);
            }
            
            Some(local_framebuffer)
        })
        .collect();

    // Merge all framebuffers
    for body_framebuffer in body_renders.into_iter().flatten() {
        framebuffer.blend_with(&body_framebuffer);
    }
}



fn main() {
    let window_width = 1000;
    let window_height = 1000;
    let framebuffer_width = 1000;
    let framebuffer_height = 1000;
    let mut option = 0;
    let skybox = Skybox::new(5000);

    let music = AudioPlayer::new("assets/observatory.mp3");
    music.play();

    let mut angle:f32 = 0.0;
    let mut last_mouse_pos = (0.0, 0.0);
    let mut mouse_sensitivity = 0.005; 
    let mut zoom_sensitivity = 0.1;


    let mut time:f32 = 0.0;

    let mut noise = create_noise(1);

    let obj1 = Obj::load("assets/sphere.obj").expect("Failed to load obj");
    let obj2 = Obj::load("assets/sphere.obj").expect("Failed to load obj");

    let mut camera = Camera::new(
        Vec3::new(5.0, 55.0, 0.0), 
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        false,
    );
    let projection_matrix = create_projection_matrix(window_width as f32, window_height as f32);
    let viewport_matrix = create_viewport_matrix(framebuffer_width as f32, framebuffer_height as f32);
    let frustrum = Frustum::new(&projection_matrix);

    let uniform = Arc::new(Mutex::new(Uniforms::new(
        Mat4::identity(),
        Mat4::identity(),
        projection_matrix,
        viewport_matrix,
        time,
        noise
    )));

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

    framebuffer.set_background_color(Color::new(0, 0, 0));

    let obj = Obj::load("assets/sphere.obj").expect("Failed to load obj");
    let array = obj.get_vertex_array();

    //obj_path: &str, traslation: Vec3, rotation: Vec3, orbit_radius: f32, scale: f32, orbit_speed: f32, orbit_center: Vec3
    let mut translation = Vec3::new(0.0, 0.0, 0.0);
    let mut rotation = Vec3::new(0.0, 0.0, 0.0);
    let mut scale = 1.0f32;
    let mut celestial_bodies = vec![
        CelestialBody::new("assets/sphere.obj", Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0), 0.0, 4.0, 0.0, Vec3::new(0.0,0.0,0.0), 1.0, 0.0), 
        CelestialBody::new("assets/sphere.obj", Vec3::new(4.0, 5.0, 0.0), Vec3::new(0.0, 0.0, 0.0), 4.0, 1.0, 0.01, Vec3::new(0.0,0.0,0.0), 2.0, 0.0),
        CelestialBody::new("assets/sphere.obj", Vec3::new(8.0, 10.0, 0.0), Vec3::new(0.0, 0.0, 0.0), 12.0, 2.0, 0.01, Vec3::new(0.0,0.0,0.0), 3.0, PI/2.0),
        CelestialBody::new("assets/sphere.obj", Vec3::new(12.0, 15.0, 0.0), Vec3::new(0.0, 0.0, 0.0), 24.0, 2.0, 0.01, Vec3::new(0.0,0.0,0.0), 4.0, (3.0*PI)/2.0),
        CelestialBody::new("assets/sphere.obj", Vec3::new(16.0, 20.0, 0.0), Vec3::new(0.0, 0.0, 0.0), 30.0, 3.0, 0.01, Vec3::new(0.0,0.0,0.0), 5.0, PI * 1.0),
        CelestialBody::new("assets/sphere.obj", Vec3::new(20.0, 25.0, 0.0), Vec3::new(0.0, 0.0, 0.0), 36.0, 1.0, 0.01, Vec3::new(0.0,0.0,0.0), 6.0, PI/3.0),
        CelestialBody::new("assets/sphere.obj", Vec3::new(24.0, 30.0, 0.0), Vec3::new(0.0, 0.0, 0.0), 45.0, 3.0, 0.01, Vec3::new(0.0,0.0,0.0), 7.0, PI/5.0)

        
    ];

    while window.is_open() {
        
        if window.is_key_down(Key::Escape) {
            println!("Escape key pressed, exiting...");
            break;
        }


        if window.is_key_down(Key::Enter){
            camera.eye = Vec3::new(5.8, 100.0, 0.0);
            camera.center = Vec3::new(0.0, 0.0, 0.0);
            camera.up = Vec3::new(0.0, 1.0, 0.0);
        }

        handle_input(&window, &mut translation, &mut rotation, &mut scale, &mut camera, &mut last_mouse_pos);
        framebuffer.clear();
        {
            let uniform_guard = uniform.lock().unwrap();
            skybox.render(&mut framebuffer, &uniform_guard, camera.eye);
        }
       
        time += 1.0;

        // Render bodies in parallel
        render_bodies_parallel(
            &mut celestial_bodies,
            &mut framebuffer,
            &uniform,
            &camera,
            time
        );


        window
            .update_with_buffer(&framebuffer.cast_buffer(), framebuffer_width, framebuffer_height)
            .expect("Failed to update window");
    }

}

fn handle_input(window: &Window, translation: &mut Vec3, rotation: &mut Vec3, scale: &mut f32, camera: &mut Camera, last_mouse_pos: &mut (f32, f32)) {
    let movement_speed = 0.3;
    let rotation_speed = PI / 50.0;
    let zoom_speed = 0.3;
    let mut movement = Vec3::new(0.0, 0.0, 0.0);
    //Movimiento de la cámara con mouse 10 puntos)
    if let Some((x, y)) = window.get_mouse_pos(MouseMode::Discard) {
        
        let dx = x as f32 - last_mouse_pos.0;
        let dy = y as f32 - last_mouse_pos.1;
        
        
        if dx != 0.0 || dy != 0.0 {
            camera.orbit(-dx * 0.001, -dy * 0.001);
        }

       
        *last_mouse_pos = (x as f32, y as f32);
    }
    //Zoom con scroll (10 puntos)
    if let Some((_scroll_x, scroll_y)) = window.get_scroll_wheel() {
        if scroll_y != 0.0 {
            camera.zoom(scroll_y * 0.003);
        }
    }

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
        println!("Pressed A");
        
        movement.x -= movement_speed;
        println!("Movement x: {}", movement.x);
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

    
    if movement.magnitude() != 0.0 {
        camera.move_center(movement);
        println!("Camera center {}", camera.center);
    }
}
