pub mod canvas;
pub mod ray_tracer;
mod utils;

use crate::canvas::Canvas;
use ray_tracer::tracer::render;
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
pub fn draw(ctx: &CanvasRenderingContext2d, width: u32, height: u32) -> Result<(), JsValue> {
    let mut canvas = Canvas::new(width as i32, height as i32);
    render(&mut canvas);
    ctx.put_image_data(&canvas.draw()?, 0.0, 0.0)
}
