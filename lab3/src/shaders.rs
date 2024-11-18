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


pub fn fragment_shader(fragment: &Fragment, uniforms: &Uniforms, planet_option: f32) -> Color {

    if (planet_option == 1.0){
        return sun_shader(fragment, uniforms)
    }
    else if (planet_option == 2.0){
        return rocks_sufrace_shader(fragment, uniforms)
    }
    else if (planet_option == 4.0){
        return asteroid_shader(fragment, uniforms)
    }
    else if (planet_option == 5.0){
        return diamond_planet_shader(fragment, uniforms)
    }
    else{
        let stripe_width = 0.5; // Adjust the width of the stripes as needed
        let x = fragment.vertex_position.x;
        let red = Color::new(255, 0, 0); // Red stripe
        let blue = Color::new(0, 0, 255); // Blue stripe

        let t = ((x / stripe_width) as i32 % 2) as f32;
        let color = Color::lerp(&red, &blue, t);
        return color * fragment.intensity  
    }
}


pub fn rocks_sufrace_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let zoom = 100;
    let x = fragment.vertex_position.x;
    let y = fragment.vertex_position.y;
    let noise = uniforms.noise.get_noise_2d((x)* zoom as f32, (y) * zoom as f32);
    
    let final_color = if noise > 0.65 { Color::new(201, 208, 173)} else {Color::new(62, 47, 37)};
    final_color

}

pub fn sun_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let zoom = 50;
    let x = fragment.vertex_position.x;
    let y = fragment.vertex_position.y;
    let noise = uniforms.noise.get_noise_2d((x)* zoom as f32 + uniforms.time, (y) * zoom as f32 + uniforms.time );
    let final_color = if noise > 0.5 { Color::new(255,102,0)} else {Color::new(242,242,23)};
    final_color
}

pub fn asteroid_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color{
   // Scale coordinates to adjust noise texture size
   let zoom = -1500.0; // Adjust zoom to control the texture scale
   let x = fragment.vertex_position.x;
   let y = fragment.vertex_position.y;
   
   // Get 2D noise based on the fragment's position and time
   let noise_value = uniforms.noise.get_noise_2d(
       (-x * zoom) ,
       (-y * zoom) 
   );
   


   // Color based on the noise value to simulate asteroid surface
   let final_color = if noise_value > -0.80 {
       Color::new(105, 105, 105) // Gray color for rocky surface
   } else {
       Color::new(70,70,70) // Darker color for rough patches
   };

   final_color
}

pub fn diamond_planet_shader(fragment: &Fragment, uniforms: &Uniforms)-> Color{
   let zoom = 700.0; // Adjust zoom to control the texture scale
   let x = fragment.vertex_position.x;
   let y = fragment.vertex_position.y;
   
   // Get 2D noise based on the fragment's position and time
   let noise_value = uniforms.noise.get_noise_2d(
       (-x * zoom) ,
       (-y * zoom) 
   );

   println!("Noise value: {}", noise_value);


   // Color based on the noise value to simulate asteroid surface
   let final_color = if noise_value > 0.4 {
       Color::new(105, 105, 105) // Gray color for rocky surface
   } else {
        Color::new(0,255,0)
   };

   final_color
}