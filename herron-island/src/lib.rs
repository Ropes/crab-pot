mod utils;
mod func_plot;
mod mandelbrot;
mod plot3d;
mod tides;

use std::error::Error;
use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;
use crate::tides::*;

extern crate web_sys;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// Call the JS alert() callback.
#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
extern "C" {
    unsafe fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(s: &str) {
    utils::set_panic_hook();
    let res = parse_noaa_tides(s).unwrap();
    let tides: String = res.iter().map(|t| t.to_string()).collect();
    log!("{:?}", tides);

    alert(s);
}


#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// Type alias for the result of a drawing function.
pub type DrawResult<T> = Result<T, Box<dyn std::error::Error>>;

/// Type used on the JS side to convert screen coordinates to chart
/// coordinates.
#[wasm_bindgen]
pub struct Chart {
    convert: Box<dyn Fn((i32, i32)) -> Option<(f64, f64)>>,
}

/// Result of screen to chart coordinates conversion.
#[wasm_bindgen]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[wasm_bindgen]
impl Chart {
    /// Draw provided power function on the canvas element using it's id.
    /// Return `Chart` struct suitable for coordinate conversion.
    pub fn power(canvas_id: &str, power: i32) -> Result<Chart, JsValue> {
        let map_coord = func_plot::draw(canvas_id, power).map_err(|err| err.to_string())?;
        Ok(Chart {
            convert: Box::new(move |coord| map_coord(coord).map(|(x, y)| (x.into(), y.into()))),
        })
    }

    // Draw Tides on provided canvas element
    // Return Chart struct suitable for Coordinate conversion from (f32, f32)...?
    pub fn tides(canvas_id: &str, raw_tides: &str) -> Result<Chart, JsValue> {
        let tv = parse_noaa_tides(raw_tides).unwrap();
        let map_coord = tides::draw(canvas_id, tv).map_err(|err| err.to_string())?;
        //let mc = tides::draw(canvas_id, tv);
        Ok(Chart{
            convert: Box::new(move |coord| map_coord(coord).map(|(x, y)| (x.into(), y.into()))),
        })
    }

    /// Draw Mandelbrot set on the provided canvas element.
    /// Return `Chart` struct suitable for coordinate conversion.
    pub fn mandelbrot(canvas: HtmlCanvasElement) -> Result<Chart, JsValue> {
        let map_coord = mandelbrot::draw(canvas).map_err(|err| err.to_string())?;
        Ok(Chart {
            convert: Box::new(map_coord),
        })
    }


    pub fn plot3d(canvas: HtmlCanvasElement, pitch: f64, yaw: f64) -> Result<(), JsValue> {
        plot3d::draw(canvas, pitch, yaw).map_err(|err| err.to_string())?;
        Ok(())
    }

    /// This function can be used to convert screen coordinates to
    /// chart coordinates.
    pub fn coord(&self, x: i32, y: i32) -> Option<Point> {
        (self.convert)((x, y)).map(|(x, y)| Point { x, y })
    }
}

