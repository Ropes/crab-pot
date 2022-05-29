mod tides;
mod utils;

use crate::tides::*;
//use std::error::Error;
use wasm_bindgen::prelude::*;
//use web_sys::HtmlCanvasElement;

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
    // Draw Tides on provided canvas element
    // Return Chart struct suitable for Coordinate conversion from (f32, f32)...?
    pub fn tides(canvas_id: &str, raw_tides: &str, draw_cw: bool) -> Result<Chart, JsValue> {
        let p: Predictions = serde_json::from_str(raw_tides).unwrap();
        let tv: Vec<TidePoint> = p.tide_points();
        let map_coord = tides::draw(canvas_id, tv, draw_cw).map_err(|err| err.to_string())?;
        Ok(Chart {
            convert: Box::new(move |coord| map_coord(coord).map(|(x, y)| (x.into(), y.into()))),
        })
    }

    /// This function can be used to convert screen coordinates to
    /// chart coordinates.
    pub fn coord(&self, x: i32, y: i32) -> Option<Point> {
        (self.convert)((x, y)).map(|(x, y)| Point { x, y })
    }
}
