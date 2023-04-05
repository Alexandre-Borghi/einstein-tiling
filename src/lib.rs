use wasm_bindgen::prelude::*;
use web_sys::window;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

#[wasm_bindgen(start)]
pub fn main() {
    let window = window().unwrap();
    let document = window.document().unwrap();
    let canvas = document
        .query_selector("canvas")
        .unwrap()
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap();
    let ctx = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();
    ctx.set_fill_style(&"white".into());
    ctx.fill_rect(10., 10., 200., 200.);
}
