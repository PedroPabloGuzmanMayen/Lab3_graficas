use crate::color::Color;
use crate::bmp::write_bmp_file;
use fastnoise_lite::{FastNoiseLite, NoiseType, FractalType, CellularDistanceFunction, CellularReturnType};
pub struct FrameBuffer {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<Color>,
    pub zbuffer: Vec<f32>,
    pub background_color: Color,
    pub current_color: Color
}

impl FrameBuffer {
    pub fn new (width: usize, height: usize) -> FrameBuffer {
        let default_color = Color::new(255,255,255);
        let buffer = vec![default_color; width*height];
        let zbuffer = vec![f32::INFINITY; width*height];
        FrameBuffer {
            width,
            height,
            buffer,
            zbuffer,
            background_color: default_color,
            current_color: default_color
        }
    }

    pub fn blend_with(&mut self, other: &FrameBuffer) {
        // Verify dimensions match
        assert_eq!(self.width, other.width, "Cannot blend framebuffers of different widths");
        assert_eq!(self.height, other.height, "Cannot blend framebuffers of different heights");

        for y in 0..self.height {
            for x in 0..self.width {
                let index = y * self.width + x;
                
                // If the other buffer's z-value is smaller (closer to camera) 
                // and not background, use its color
                if other.zbuffer[index] < self.zbuffer[index] && 
                   other.buffer[index].to_hex() != other.background_color.to_hex() {
                    self.buffer[index] = other.buffer[index];
                    self.zbuffer[index] = other.zbuffer[index];
                }
            }
        }
    }

    pub fn clear_with_pattern(&mut self) {
        let mut noise = FastNoiseLite::new();
        noise.set_noise_type(Some(NoiseType::Cellular));
        noise.set_frequency(Some(0.1)); // Adjust the frequency to control star distribution

        for y in 0..self.height {
            for x in 0..self.width {
                let fx = x as f32 / self.width as f32;
                let fy = y as f32 / self.height as f32;

                // Generate noise value for the pixel
                let value = noise.get_noise_2d(fx, fy);

                // Threshold to decide if the pixel is a star or background
                if value > -0.77115 { // Adjust this threshold for star density
                    self.buffer[y * self.width + x] = Color::new(255, 255, 255); // White star
                } else {
                    self.buffer[y * self.width + x] = Color::new(0, 0, 0); // Black sky
                }

                self.zbuffer[y * self.width + x] = f32::INFINITY; // Reset z-buffer
            }
        }
    }

    pub fn clear(&mut self){
        self.buffer.fill(self.background_color);
        for depth in self.zbuffer.iter_mut(){
            *depth = f32::INFINITY;
        }
    }

    pub fn point(&mut self, x:usize, y:usize, depth: f32){

        if x < self.width && y < self.height {
            let index = y * self.width + x;
            if self.zbuffer[index] > depth {
                self.buffer[index] = self.current_color;
                self.zbuffer[index] = depth;
            }
        }
    }

    pub fn set_background_color(&mut self, color:Color){
        self.background_color = color;
    }

    pub fn get_color(&mut self, x:usize, y:usize) -> Color {
        self.buffer[self.width * y + x]
    }

    pub fn set_current_color(&mut self, color:Color){
        self.current_color = color;
    }
    pub fn write_to_bmp(&self, file_path: &str) -> std::io::Result<()> {
        let buffer: Vec<u32> = self.buffer.iter().map(|c| c.to_hex()).collect();
        write_bmp_file(file_path, &buffer, self.width, self.height)
    }

    pub fn cast_buffer(&self) -> Vec<u32> {
        let mut casted_vector: Vec<u32> = Vec::with_capacity(self.buffer.len());  
        for color in &self.buffer {
            casted_vector.push(color.to_hex());
        }
        casted_vector
    }
}