use crate::color::Color;
use crate::bmp::write_bmp_file;
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