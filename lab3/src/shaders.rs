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
        return combined_shader(fragment, uniforms)
    }
    else if (planet_option == 3.0){
        return combined_gas_planet_with_rings_shader(fragment, uniforms)
    }
    else if (planet_option == 4.0){
        return asteroid_shader(fragment, uniforms)
    }
    else if (planet_option == 5.0){
        return diamond_planet_shader(fragment, uniforms)
    }
    else if (planet_option == 6.0){
        return combined_skin_vein_shader(fragment, uniforms)
    }
    else if (planet_option == 7.0){
        return light_city_with_rain_shader(fragment, uniforms)
    }
    else{
        return Color::new(0,0,0) 
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
    //Simula mover la lava de la estrella con el tiempo (10pts)
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


   // Color based on the noise value to simulate asteroid surface
   let final_color = if noise_value > 0.4 {
       Color::new(0,195,255) // Gray color for rocky surface
   } else {
        Color::new(0,246,255)
   };

   final_color
}

pub fn gas_planet_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color{
    let zoom = 100.0;
    let x = fragment.vertex_position.x;
    let y = fragment.vertex_position.y;
    let noise_value = uniforms.noise.get_noise_2d(
        (-x * zoom) ,
        (-y * zoom) 
    );
 
    let final_color = if noise_value > 0.4 {
        Color::new(164, 63, 47) // Gray color for rocky surface
    } else {
         Color::new(145,170,95)
    };
 
    final_color
}

//Sistema de anillos (20 pts)
pub fn ring_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let ring_center_x = 0.0; // Center of the ring system
    let ring_center_y = 0.0;
    let ring_width = 0.02; // Width of the ring bands
    let ring_spacing = 0.05; // Spacing between bands

    // Convert fragment position to polar coordinates relative to the ring center
    let dx = fragment.vertex_position.x - ring_center_x;
    let dy = fragment.vertex_position.y - ring_center_y;
    let distance = (dx * dx + dy * dy).sqrt();

    // Create concentric bands
    let band_value = (distance / ring_spacing).fract(); // Normalize within band spacing
    let is_ring_band = band_value < ring_width;

    if is_ring_band {
        // Return ring color
        Color::new(200, 200, 200) // A light gray color for the ring bands
    } else {
        // Transparent background (skipped color logic as you have no alpha)
        Color::new(0, 0, 0) // Assume black as the transparent equivalent
    }
}

pub fn combined_gas_planet_with_rings_shader(
    fragment: &Fragment,
    uniforms: &Uniforms,
) -> Color {
    // Gas planet shader
    let gas_planet_color = gas_planet_shader(fragment, uniforms);

    // Ring shader
    let ring_color = ring_shader(fragment, uniforms);

    // Combine: Check if the fragment is part of the ring
    if ring_color.to_hex() != Color::new(0, 0, 0).to_hex() {
        ring_color // Render the ring color
    } else {
        gas_planet_color
    }
}



pub fn skin_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color{
    let zoom = 100.0;
    let x = fragment.vertex_position.x;
    let y = fragment.vertex_position.y;
    let noise_value = uniforms.noise.get_noise_2d(
        (-x * zoom) ,
        (-y * zoom) 
    );

 
    let final_color = if noise_value > 0.4 {
        Color::new(255,195,160) // Gray color for rocky surface
    } else {
         Color::new(255,217,194)
    };
 
    final_color
}

pub fn vein_shader(fragment: &Fragment, _uniforms: &Uniforms) -> Color {
    let x = fragment.vertex_position.x;
    let y = fragment.vertex_position.y;

    // Sine wave for veins
    let frequency = 10.0; // Adjust for vein density
    let amplitude = 0.2; // Adjust for vein width
    let sine_wave = (y * frequency).sin() * amplitude;

    // Threshold for vein width
    let vein_threshold = 0.05;

    if (x - sine_wave).abs() < vein_threshold {
        // Red tone for veins
        Color::new(180, 50, 50) // Subtle red for veins
    } else {
        // Return a "neutral" color when not in a vein region
        Color::new(0, 0, 0) // Black or neutral contribution
    }
}


pub fn combined_skin_vein_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    // Get base skin color
    let skin_color = skin_shader(fragment, uniforms);

    // Get vein color
    let vein_color = vein_shader(fragment, uniforms);

    // Blend the colors based on vein presence
    let is_in_vein = vein_color.to_hex() != Color::new(0, 0, 0).to_hex();

    if is_in_vein {
        // Blend skin and vein colors
        Color::new(
            ((skin_color.r as u16 + vein_color.r as u16) /2) as u8,
            ((skin_color.g as u16 + vein_color.g as u16) /2) as u8,
            ((skin_color.b as u16 + vein_color.b as u16)/2) as u8,
        )
    } else {
        // Default to skin color
        skin_color
    }
}


//Luna a un planeta
pub fn combined_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    // Rock surface shader parameters
    let zoom = 100.0;
    let x = fragment.vertex_position.x;
    let y = fragment.vertex_position.y;
    let noise = uniforms.noise.get_noise_2d(x * zoom, y * zoom);

    // Define colors for the rocky surface
    let rock_color = if noise > 0.65 {
        Color::new(201, 208, 173) // Light rock color
    } else {
        Color::new(62, 47, 37) // Dark rock color
    };

    // Moving circle shader parameters
    let circle_radius = 0.01; // Adjust size of the circle
    let speed = 0.01; // Adjust speed of horizontal movement
    let time = uniforms.time;

    // Calculate the circle's moving position
    let circle_x = (time * speed).sin() * 0.8; // Moves in the range of [-0.8, 0.8]
    let circle_y = 0.0; // Keep the circle centered vertically

    // Calculate the distance from the current fragment to the circle's center
    let dx = x - circle_x;
    let dy = y - circle_y;
    let distance_from_circle_center = (dx * dx + dy * dy).sqrt();

    // Check if the fragment is inside the circle
    let is_inside_circle = distance_from_circle_center < circle_radius;

    // Define color for the moving circle
    let circle_color = Color::new(255, 0, 0); // White circle

    // Blend the rock surface with the moving circle
    if is_inside_circle {
        circle_color
    } else {
        rock_color
    }
}


pub fn light_city_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color{
    let zoom = 100.0;
    let x = fragment.vertex_position.x;
    let y = fragment.vertex_position.y;
    let noise = uniforms.noise.get_noise_2d(x * zoom, y * zoom);

    // Define colors for the rocky surface
    let final_color = if noise > 0.65 {
        Color::new(201, 208, 173) // Light rock color
    } else {
        Color::new(62, 47, 37) // Dark rock color
    };

    final_color
}

//Simulación de lluvia y vientos (10 puntos)
pub fn light_city_with_rain_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let zoom = 100.0;
    let x = fragment.vertex_position.x;
    let y = fragment.vertex_position.y;

    // Base city light shader
    let city_noise = uniforms.noise.get_noise_2d(x * zoom, y * zoom);
    let base_color = if city_noise > 0.65 {
        Color::new(253, 170, 72) // Light rock color
    } else {
        Color::new(71, 71, 71) // Dark rock color
    };

    // Rain effect
    let time = uniforms.time; // Uniform to animate the rain
    let rain_intensity = 0.6; // Brightness of the rain streaks
    let rain_speed = 10.0; // Speed of the rain fall
    let streak_width = 0.02; // Thickness of each streak

    // Calculate rain streaks based on vertical position and animation
    let streak_position = (x * 10.0 + time * rain_speed) % 1.0; // Repeating streak pattern
    let is_rain = (streak_position - y * 5.0).abs() < streak_width;

    // Define rain color
    let rain_color = if is_rain {
        (Color::new(200, 200, 255) * (rain_intensity)) // Light blue for rain
    } else {
        (Color::new(0, 0, 0) * 0.0) 
    };

    // Combine base color with rain effect
    base_color.blend_normal(&rain_color)
}


