use wasm_bindgen::prelude::*;
use web_sys::window;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

const DEBUG_DRAW: bool = true;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    let window = window().unwrap();
    let document = window.document().unwrap();
    let canvas = document
        .query_selector("canvas")?
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()?;
    let ctx = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()?;
    ctx.set_fill_style(&"white".into());
    hat(&ctx, 200., 200., 50., 1.)?;
    Ok(())
}

fn hat(
    ctx: &CanvasRenderingContext2d,
    x: f64,
    y: f64,
    scale: f64,
    angle: f64,
) -> Result<(), JsValue> {
    ctx.save();
    ctx.translate(x, y)?;
    ctx.rotate(angle);
    ctx.scale(scale, scale)?;
    ctx.begin_path();
    ctx.move_to(0., 0.);
    // https://github.com/christianp/aperiodic-monotile/blob/9844d8b093b54b4c3e13a2ca43fb26d437f3c68b/hat-monotile.svg
    // The following are the absolute coordinates computed from the relative coordinates in the svg.
    ctx.line_to(0., -1.73205081);
    ctx.line_to(-1., -1.73205081);
    ctx.line_to(-1.5, -2.59807);
    ctx.line_to(-3., -1.73205081);
    ctx.line_to(-3., 0.);
    ctx.line_to(-4., 0.);
    ctx.line_to(-4.5, 0.8660254);
    ctx.line_to(-3., 1.7320508);
    ctx.line_to(-1.5, 0.8660254);
    ctx.line_to(-1., 1.7320508);
    ctx.line_to(1., 1.7320508);
    ctx.line_to(1.5, 0.8660254);
    ctx.close_path();
    ctx.fill();
    if DEBUG_DRAW {
        ctx.save();
        ctx.set_fill_style(&"red".into());
        ctx.begin_path();
        ctx.ellipse(0., 0., 0.2, 0.2, 0., 0., std::f64::consts::TAU)?;
        ctx.fill();
        ctx.restore();
    }
    ctx.restore();
    Ok(())
}
