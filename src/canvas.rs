use wasm_bindgen::{Clamped, JsValue};
use web_sys::ImageData;

use crate::ray_tracer::color::Color;

#[derive(Debug, Clone)]
pub struct Canvas {
    pub width: i32,
    pub height: i32,
    pixels: Vec<Color>,
}

impl Canvas {
    pub fn new(width: i32, height: i32) -> Self {
        let size = width * height;
        Self {
            width,
            height,
            pixels: vec![Color::white(); size as usize],
        }
    }

    fn index_at(&self, x: i32, y: i32) -> usize {
        let canvas_x = (self.width / 2) + x;
        let canvas_y = (self.height / 2) + y;

        (canvas_y * self.width + canvas_x) as usize
    }

    pub fn put_pixel(&mut self, x: i32, y: i32, color: &Color) {
        let index = self.index_at(x, y);
        self.pixels[index] = color.clone();
    }

    pub fn pixel_at(&self, x: i32, y: i32) -> Color {
        let index = self.index_at(x, y);
        self.pixels[index]
    }

    pub fn draw(&self) -> Result<ImageData, JsValue> {
        let mut image: Vec<u8> = self.pixels.iter().flat_map(|c| c.as_u8_array()).collect();
        ImageData::new_with_u8_clamped_array_and_sh(
            Clamped(&mut image),
            self.width as u32,
            self.height as u32,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creating_a_canvas() {
        let c = Canvas::new(10, 20);
        
        assert_eq!(c.width, 10);
        assert_eq!(c.height, 20);
    }

    #[test]
    fn test_writing_pixels_to_a_canvas() {
        let mut c = Canvas::new(10, 20);
        let red = Color::new(1.0, 0.0, 0.0);

        c.put_pixel(2, 3, &red);

        assert_eq!(c.pixel_at(2, 3), red);
    }



}
