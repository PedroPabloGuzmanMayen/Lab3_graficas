use std::fmt;
use std::ops::{Add, Mul};

#[derive(Debug, Copy, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Color {
        Color {
            r: r.clamp(0, 255),
            g: g.clamp(0, 255),
            b: b.clamp(0, 255),
        }
    }

    pub fn from_hex(hex: u32) -> Color {
        let r = ((hex >> 16) & 0xFF) as u8;
        let g = ((hex >> 8) & 0xFF) as u8;
        let b = (hex & 0xFF) as u8;
        Color { r, g, b }
    }

    pub fn lerp(&self, other: &Color, t:f32) -> Self{
        let t = t.clamp(0.0, 1.0);
        Color{
            r: (self.r as f32 + (other.r as f32 - self.r as f32) * t).round() as u8,
            g: (self.g as f32 + (other.g as f32 - self.g as f32) * t).round() as u8,
            b: (self.b as f32 + (other.b as f32 - self.b as f32) * t).round() as u8,
        }
    }

    pub fn is_black(&self) -> bool {
        self.r == 0 && self.g == 0 && self.b == 0
    }

    pub fn blend_normal(&self, blend: &Color) -> Color {
        if blend.is_black() {*self} else {*blend}
    }
    pub fn blend_multiply(&self, blend: &Color) -> Color {
        Color::new(
            ((self.r as f32 * blend.r as f32) / 255.0) as u8,
            ((self.g as f32 * blend.g as f32) / 255.0) as u8,
            ((self.b as f32 * blend.b as f32) / 255.0) as u8
        )
    }
    pub fn blend_lighten(&self, blend: &Color) -> Color {
        Color::new(
            self.r.max(blend.r),
            self.g.max(blend.g),
            self.b.max(blend.b)
        )
    }

    pub fn to_hex(&self) -> u32 {
        ((self.r as u32) << 16) | ((self.g as u32) << 8) | (self.b as u32)
    }
}

impl Add for Color {
    type Output = Color;

    fn add(self, color2: Color) -> Color {
        let r = self.r.saturating_add(color2.r);
        let g = self.g.saturating_add(color2.g);
        let b = self.b.saturating_add(color2.b);

        Color { r, g, b }
    }
}

impl Mul<f32> for Color {
    type Output = Color;

    fn mul(self, factor: f32) -> Color {
        let r = (self.r as f32 * factor).clamp(0.0, 255.0) as u8;
        let g = (self.g as f32 * factor).clamp(0.0, 255.0) as u8;
        let b = (self.b as f32 * factor).clamp(0.0, 255.0) as u8;
        Color { r, g, b }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Color(r: {}, g: {}, b: {})", self.r, self.g, self.b)
    }
}


