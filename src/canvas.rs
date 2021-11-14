use std::ops::{Add, Mul};

use serde::Deserialize;
use wasm_bindgen::{Clamped, JsValue};
use web_sys::ImageData;

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b, a: 255 }
    }

    pub fn black() -> Self {
        Self::new(0, 0, 0)
    }

    pub fn white() -> Self {
        Self::new(255, 255, 255)
    }
}

fn mul(lhs: u8, rhs: f64) -> u8 {
    let product = lhs as f64 * rhs;
    if product >= 255.0 {
        255
    } else if product <= 0.0 {
        0
    } else {
        product as u8
    }
}

impl Add<Color> for Color {
    type Output = Color;

    fn add(self, rhs: Color) -> Self::Output {
        Color {
            r: self.r.saturating_add(rhs.r),
            g: self.g.saturating_add(rhs.g),
            b: self.b.saturating_add(rhs.b),
            a: self.a,
        }
    }
}

impl Mul<f64> for &Color {
    type Output = Color;

    fn mul(self, rhs: f64) -> Self::Output {
        Color {
            r: mul(self.r, rhs),
            g: mul(self.g, rhs),
            b: mul(self.b, rhs),
            a: self.a,
        }
    }
}

impl Mul<f64> for Color {
    type Output = Color;

    fn mul(self, rhs: f64) -> Self::Output {
        Color {
            r: mul(self.r, rhs),
            g: mul(self.g, rhs),
            b: mul(self.b, rhs),
            a: self.a,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Canvas {
    pub width: i32,
    pub height: i32,
    pixels: Vec<u8>,
}

impl Canvas {
    pub fn new(width: i32, height: i32) -> Self {
        let size = width * height * 4;
        Self {
            width,
            height,
            pixels: vec![0; size as usize],
        }
    }

    pub fn put_pixel(&mut self, x: i32, y: i32, color: &Color) {
        let canvas_x = (self.width / 2) + x;
        let canvas_y = (self.height / 2) + y;
        let index = ((canvas_y * self.width + canvas_x) * 4) as usize;

        self.pixels[index] = color.r;
        self.pixels[index + 1] = color.g;
        self.pixels[index + 2] = color.b;
        self.pixels[index + 3] = color.a;
    }

    pub fn draw(&mut self) -> Result<ImageData, JsValue> {
        ImageData::new_with_u8_clamped_array_and_sh(
            Clamped(&mut self.pixels),
            self.width as u32,
            self.height as u32,
        )
    }
}
