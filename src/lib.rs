pub mod canvas;
pub mod ray_tracer;
mod utils;

use crate::canvas::Canvas;
use ray_tracer::tracer::{render, Sphere, Viewport};
use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
// #[cfg(feature = "wee_alloc")]
// #[global_allocator]
// static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn draw(
    ctx: &CanvasRenderingContext2d,
    width: u32,
    height: u32,
    viewport_js: JsValue,
    spheres_js: JsValue,
) -> Result<(), JsValue> {
    let viewport: Viewport = serde_wasm_bindgen::from_value(viewport_js)?;
    let spheres: Vec<Sphere> = serde_wasm_bindgen::from_value(spheres_js)?;
    let mut canvas = Canvas::new(width as i32, height as i32);
    render(&mut canvas, &viewport, spheres);
    ctx.put_image_data(&canvas.draw()?, 0.0, 0.0)
}
