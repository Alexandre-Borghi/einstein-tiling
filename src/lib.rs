use std::f64::consts::{PI, TAU};
use wasm_bindgen::prelude::*;
use web_sys::window;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

const DEBUG_DRAW: bool = false;

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
    ctx.set_stroke_style(&"purple".into());
    // ctx.set_stroke_style(&"#ffffff00".into());
    ctx.set_line_width(0.1);
    ctx.translate(500., 500.);
    const SCALE: f64 = 60.;
    ctx.scale(SCALE, SCALE);
    h7(&ctx, 0., 0.)?;
    Ok(())
}

fn hat(
    ctx: &CanvasRenderingContext2d,
    x: f64,
    y: f64,
    angle: f64,
    flip: bool,
) -> Result<(), JsValue> {
    ctx.save();
    ctx.translate(x, y)?;
    ctx.rotate(angle);
    if !flip {
        ctx.scale(-1., 1.)?;
    }
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
    ctx.stroke();
    if DEBUG_DRAW {
        ctx.save();
        ctx.set_fill_style(&"red".into());
        ctx.begin_path();
        ctx.ellipse(0., 0., 0.2, 0.2, 0., 0., TAU)?;
        ctx.fill();
        ctx.restore();
    }
    ctx.restore();
    Ok(())
}

fn h7(ctx: &CanvasRenderingContext2d, x: f64, y: f64) -> Result<(), JsValue> {
    ctx.save();
    ctx.translate(x, y)?;
    hat(ctx, 0., 0., PI / 3., true)?;
    hat(ctx, 0., -3.464101619, -2. * PI / 3., false)?;
    hat(ctx, 3., -1.7320502996, 2. * PI / 3., false)?;
    hat(ctx, 0., 3.4641026000, 0., false)?;
    hat(ctx, -3., 1.7320508000, PI / 3., false)?;
    hat(ctx, -3., -1.7320508000, 2. * PI / 3., false)?;
    hat(ctx, -3., -5.1961528109, PI, false)?;
    ctx.restore();
    Ok(())
}
