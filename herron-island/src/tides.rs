use crate::utils::*;
use crate::DrawResult;
use chrono::prelude::*;
use chrono::{DateTime, Duration, Local, TimeZone, Utc};
use plotters::element::*;
use plotters::{self};
use plotters::{prelude::*, style::RGBAColor};
use plotters_canvas::CanvasBackend;
use serde::{Deserialize, Serialize};
use std::error::Error;
use wasm_bindgen::prelude::*;
//use image::{imageops::FilterType, ImageFormat};

use std::fs::File;
use std::io::BufReader;

extern crate web_sys;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log_wasm {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[cfg(target_family = "unix")]
macro_rules! log {
    ( $( $t:tt )* ) => {
        println!( $( $t )* );
    }
}

// Call the JS alert() callback.
#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
extern "C" {
    unsafe fn alert(s: &str);
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// Draw power function f(x) = x^power.
pub fn draw(
    canvas_id: &str,
    tv: Vec<TidePoint>,
    draw_cw: bool,
) -> DrawResult<impl Fn((i32, i32)) -> Option<(f32, f32)>> {
    let backend = CanvasBackend::new(canvas_id).expect("cannot find canvas");
    let root = backend.into_drawing_area();
    let label_style = TextStyle::from(("sans-serif", 10).into_font()).color(&WHITE);
    root.fill(&RGBColor(54, 95, 145))?;

    // Find the local date, flatten to naive date, then convert it to a UTC date.
    let now = Local::now();
    let l = now
        .with_hour(0)
        .unwrap()
        .with_minute(0)
        .unwrap()
        .with_second(0)
        .unwrap();
    let neh = l.naive_local();
    let today = Utc.from_local_datetime(&neh).unwrap();
    log_wasm!("Local: {:?} Naive:{:?}", now, neh);

    let chart_top = 20f32;
    let chart_bottom = -8f32;
    let mut chart = ChartBuilder::on(&root)
        .margin(10)
        //.caption(format!("Sea Level"), font)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0f32..24f32, chart_bottom..chart_top)?;

    chart
        .configure_mesh()
        .x_labels(5)
        .x_label_style(&WHITE)
        .x_desc(format!("{}", today.format("%Y-%m-%d Tides")))
        .y_labels(10)
        .y_label_style(label_style)
        .y_desc("Sea Level")
        .draw()?;

    let xys = coordinates_from_prediction(tv.to_owned(), today);
    log_wasm!("xys read: {:?}", xys.len());
    chart.draw_series(AreaSeries::new(
        xys.iter().filter_map(|(x, y)| {
            if *x > 0f32 && *x < 24f32 {
                return Some((x.clone(), y.clone()));
            }
            return None;
        }),
        -10.0,
        RGBColor(139, 166, 214).mix(0.5),
    ))?;
    /*
    chart.draw_series(LineSeries::new(
        xys.iter().filter_map(|(x, y)| {
            if *x > 0f32 && *x < 24f32 {
                return Some((x.clone(), y.clone()));
            }
            return None;
        }),
        &BLUE,
    ))?;
    */

    // Draw vertical line to show current time
    let x_val = now.hour() as f32 + (now.minute() as f32 / 60f32);
    let x_split = 0.05f32;
    let y_val = xys
        .iter()
        .find_map(|(x, y)| if x > &x_val { Some(y) } else { None })
        .unwrap();
    let mut xs: Vec<f32> = Vec::new();
    xs.push(x_val);

    chart.draw_series(xs.iter().map(|x| {
        Rectangle::new(
            [(x - &x_split, chart_bottom), (x + &x_split, *y_val)],
            //RGBColor(127, 255, 127).filled(),
            RGBColor(54, 200, 95).filled(),
        )
    }))?;

    // Draw visual points to indicate when and tide level
    let valid_tp: Vec<&TidePoint> = tv
        .iter()
        .filter_map(|x| {
            if x.dt > today && x.dt < today + Duration::days(1) {
                return Some(x);
            }
            None
        })
        .collect();

    chart.draw_series(valid_tp.iter().map(|t| {
        let (x, y) = t.to_xy();
        return Circle::new((x, y), 4, ShapeStyle::from(&WHITE));
    }))?;
    log_wasm!("circles drawn: tv: {:?}", tv.len());

    let mut poly_vec = Vec::<(f32, f32)>::new();
    tv.iter().for_each(|t| {
        let z = t.to_xy();
        poly_vec.push(z);
    });

    let point_style = TextStyle::from(("sans-serif", 15).into_font()).color(&WHITE);
    chart.draw_series(PointSeries::of_element(
        valid_tp.to_owned().iter().map(|t| t.to_xy()),
        3,
        ShapeStyle::from(&CYAN).filled(),
        &|coord, size, style| {
            let hour = coord.0 as i32;
            let minutes = ((coord.0.as_f64() - hour.as_f64()) * 60f64).round();
            return EmptyElement::at(coord)
                + Circle::new((0, 0), size, style)
                + Text::new(
                    format!("[{:02}:{:02}] {:.1}", hour, minutes, coord.1),
                    (-40, 15),
                    &point_style,
                );
        },
    ))?;

    // Draw the Charlie Wells
    /* Seems not possible with Plotters 3.x, may have worked in 2.x
    let (w, h) = chart.plotting_area().dim_in_pixel();
    let image = image::load(
        BufReader::new(
            File::open("www/charlie-wells.svg").map_err(|e| {
                eprintln!("Unable to open file charlie-wells.svg");
                e
            })?),
        ImageFormat::Png,
    )?
    .resize_exact(w - w / 10, h - h / 10, FilterType::Nearest);
    */

    // Draw Charlie Wells manually
    if draw_cw {
        //let ch_scale = 0.3f32;
        let ch_scale = 0.5f32;
        let oy = *y_val+(1.75f32*ch_scale);

        let mut xs: Vec<f32> = Vec::new();
        xs.push(x_val);

        // Draw cabin(white)
        chart.draw_series(xs.iter().map(|x| {
            let ox: f32 = *x;
            let cabin_width = 0.8f32;
            Rectangle::new(
                [
                    (ox - (&cabin_width) * ch_scale, oy - (1.0f32) * ch_scale),
                    (ox + (&cabin_width) * ch_scale, oy + (1.0f32) * ch_scale),
                ],
                RGBColor(255, 255, 255).filled(),
            )
        }))?;

        // Draw Deck(Red...)
        chart.draw_series(xs.iter().map(|x| {
            let ox: f32 = *x;
            let deck_width = 1.50f32;
            Polygon::new(
                [
                    (ox - (&deck_width * ch_scale), oy - (1.0f32 * ch_scale)),
                    (ox + (&deck_width * ch_scale), oy - (1.0f32 * ch_scale)),
                    (ox + (&deck_width - 0.2f32) * ch_scale, oy),
                    (ox - (&deck_width - 0.2f32) * ch_scale, oy),
                ],
                RGBColor(230, 0,0).filled(),
            )
        }))?;

        // Draw Hull(dark red)
        chart.draw_series(xs.iter().map(|x| {
            let ox: f32 = *x;
            let hull_width = 1.75f32;
            Polygon::new(
                [
                    (ox - (&hull_width) * ch_scale, oy - (1.0f32) * ch_scale),
                    (
                        ox - ((&hull_width - 0.25f32) * ch_scale),
                        oy - (2.0f32 * ch_scale),
                    ),
                    (
                        ox + ((&hull_width - 0.25f32) * ch_scale),
                        oy - (2.0f32 * ch_scale),
                    ),
                    (ox + ((&hull_width) * ch_scale), oy - ((1.0f32) * ch_scale)),
                ],
                RGBColor(72, 0, 0).filled(),
            )
        }))?;

        // Draw Bridge
        chart.draw_series(xs.iter().map(|x| {
            let ox: f32 = *x;
            let bridge_width = 0.25f32;
            Polygon::new(
                [
                    (ox - (&bridge_width * ch_scale), oy + (1.0f32 * ch_scale)),
                    (ox + (&bridge_width * ch_scale), oy + (1.0f32 * ch_scale)),
                    (ox + (&bridge_width * ch_scale), oy + (1.5f32 * ch_scale)),
                    (
                        ox + ((&bridge_width + 0.14f32) * ch_scale),
                        oy + (2.5f32 * ch_scale),
                    ),
                    (
                        ox - ((&bridge_width + 0.14f32) * ch_scale),
                        oy + (2.5f32 * ch_scale),
                    ),
                    (ox - (&bridge_width) * ch_scale, oy + (1.5f32 * ch_scale)),
                ],
                HSLColor(200.0, 200.0, 200.0).filled(),
            )
        }))?;

        // Draw flag
        // -- White background
        let flag_width = 0.6f32;
        let flag_floor = 3.0f32;
        chart.draw_series(xs.iter().map(|x| {
            let ox: f32 = *x;
            Rectangle::new(
                [
                    (ox, oy + (flag_floor * ch_scale)),
                    (
                        ox + (&flag_width * ch_scale),
                        oy + ((flag_floor + 1.0f32) * ch_scale),
                    ),
                ],
                RGBColor(255, 255, 255).filled(),
            )
        }))?;

        // -- red stripes
        chart.draw_series(xs.iter().map(|x| {
            let ox: f32 = *x;
            Rectangle::new(
                [
                    (
                        ox + (&flag_width * ch_scale),
                        oy + ((flag_floor + 1.0f32) * ch_scale),
                    ),
                    (ox, oy + ((flag_floor + 0.8f32) * ch_scale)),
                ],
                RGBColor(255, 0, 0).filled(),
            )
        }))?;

        chart.draw_series(xs.iter().map(|x| {
            let ox: f32 = *x;
            Rectangle::new(
                [
                    (
                        ox + (&flag_width * ch_scale),
                        oy + ((flag_floor + 0.6f32) * ch_scale),
                    ),
                    (ox, oy + ((flag_floor + 0.4f32) * ch_scale)),
                ],
                RGBColor(255, 0, 0).filled(),
            )
        }))?;

        /*
        chart.draw_series(xs.iter().map(|x| {
            let ox: f32 = *x;
            Rectangle::new(
                [
                    (
                        ox + (&flag_width * ch_scale),
                        oy + ((flag_floor + 0.4f32) * ch_scale),
                    ),
                    (ox, oy + ((flag_floor + 0.2f32) * ch_scale)),
                ],
                RGBColor(255, 0, 0).filled(),
            )
        }))?;
        */

        chart.draw_series(xs.iter().map(|x| {
            let ox: f32 = *x;
            Rectangle::new(
                [
                    (
                        ox + (&flag_width * ch_scale),
                        oy + ((flag_floor + 0.2f32) * ch_scale),
                    ),
                    (ox, oy + ((flag_floor) * ch_scale)),
                ],
                RGBColor(255, 0, 0).filled(),
            )
        }))?;

        // -- blue patches
        chart.draw_series(xs.iter().map(|x| {
            let ox: f32 = *x;
            Rectangle::new(
                [
                    (ox, oy + ((flag_floor + 1.0f32) * ch_scale)),
                    (
                        ox + (0.3f32 * ch_scale),
                        oy + ((flag_floor + 0.6f32) * ch_scale),
                    ),
                ],
                RGBColor(0, 0, 255).filled(),
            )
        }))?;
    }

    root.present()?;
    return Ok(chart.into_coord_trans());
}

fn coordinates_from_prediction(tv: Vec<TidePoint>, today: DateTime<Utc>) -> Vec<(f32, f32)> {
    let chart_start = today - Duration::hours(12);
    let chart_end = chart_start + Duration::days(2);

    // Tide local maximums
    let mut xs: Vec<f32> = Vec::new();
    let mut ys: Vec<f32> = Vec::new();

    // Filter down to current day +-12 hours
    let valid_tp: Vec<&TidePoint> = tv
        .iter()
        .filter_map(|x| {
            if x.dt > chart_start && x.dt < chart_end {
                return Some(x);
            }
            None
        })
        .collect();

    valid_tp.iter().for_each(|t| {
        let (x, y) = t.to_xy();
        if t.dt < today {
            let x_adj = x - 24f32;
            xs.push(x_adj);
        } else if t.dt > today + Duration::hours(24) {
            let x_adj = 24f32 + x;
            xs.push(x_adj);
        } else {
            xs.push(x);
        }
        ys.push(y);
    });

    // Graphed X,Y vector points
    let mut tide_x: Vec<f32> = Vec::new();
    let mut tide_y: Vec<f32> = Vec::new();

    for i in 1..xs.len() {
        let x_origin = xs[i - 1]; // x0 ...
        let y_origin = ys[i - 1]; // y0 ...
        tide_x.push(x_origin);
        tide_y.push(y_origin);

        let y_delta = ys[i] - y_origin;
        let x_delta = xs[i] - x_origin;

        let x_inc = x_delta / 25f32;

        let mut x_step = x_origin + x_inc;
        while x_step < xs[i] {
            tide_x.push(x_step);
            // calculate Y
            // --------------------------
            let x_percentage = (x_step - x_origin) / x_delta;
            let to_cosine = (x_percentage * std::f32::consts::PI) + std::f32::consts::PI;
            let y_multiplier = (to_cosine.cos() + 1f32) / 2f32;
            let y_val = (y_multiplier * y_delta) + y_origin;
            tide_y.push(y_val);
            x_step += x_inc;
        }
    }

    let mut ret = Vec::<(f32, f32)>::new();
    for (_, (x, y)) in tide_x.iter().zip(tide_y.iter()).enumerate() {
        ret.push((x.clone(), y.clone()));
    }

    ret
}

#[derive(Serialize, Deserialize, Debug)]
struct JsonTide {
    t: String,
    v: String,

    #[serde(rename = "type")]
    tide: String,
}

impl JsonTide {
    pub fn to_tide_point(&self) -> Result<TidePoint, Box<dyn Error>> {
        let t = chrono::NaiveDateTime::parse_from_str(self.t.as_str(), "%Y-%m-%d %H:%M")?;
        let h = self.v.parse::<f32>()?;
        let tide = match self.tide.as_str() {
            "H" => Tide::High,
            "L" => Tide::Low,
            _ => Tide::None,
        };

        let n = Utc::now();
        let dt = DateTime::from_utc(t, *n.offset());

        Ok(TidePoint {
            dt: dt,
            level: h,
            tide: tide,
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Predictions {
    predictions: Vec<JsonTide>,
}

impl Predictions {
    pub fn to_string(&self) -> String {
        format!("{:?}", self)
    }

    pub fn tide_points(&self) -> Vec<TidePoint> {
        let tidepoints: Vec<TidePoint> = self
            .predictions
            .iter()
            .map(|t| t.to_tide_point().unwrap())
            .collect();

        tidepoints
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Tide {
    High,
    Low,
    None,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TidePoint {
    dt: DateTime<Utc>,
    level: f32,
    tide: Tide,
}

impl TidePoint {
    pub fn new(dt: DateTime<Utc>, level: f32, tide: Tide) -> TidePoint {
        TidePoint {
            dt: dt,
            level: level,
            tide: tide,
        }
    }

    pub fn to_string(&self) -> String {
        return format!("{:?}::{}:{}", self.tide, self.dt, self.level);
    }

    pub fn to_xy(&self) -> (f32, f32) {
        let y = self.level.as_f64();
        let x = match self.dt.minute() {
            0 => self.dt.hour().as_f64(),
            _ => self.dt.hour().as_f64() + (self.dt.minute().as_f64() / 60f64),
        };

        (x as f32, y as f32)
    }
}

#[cfg(test)]
mod tests {
    use crate::tides::*;
    use chrono::prelude::*;
    use chrono::{DateTime, TimeZone, Utc};

    const predicted_json_data: &str = r#"{ "predictions" : [{"t":"2022-01-10 05:05", "v":"5.086", "type":"L"},{"t":"2022-01-10 11:32", "v":"14.668", "type":"H"},{"t":"2022-01-10 19:03", "v":"2.498", "type":"L"},{"t":"2022-01-11 01:42", "v":"10.228", "type":"H"},{"t":"2022-01-11 06:15", "v":"6.854", "type":"L"},{"t":"2022-01-11 12:11", "v":"14.150", "type":"H"},{"t":"2022-01-11 19:51", "v":"1.503", "type":"L"},{"t":"2022-01-12 03:19", "v":"11.508", "type":"H"},{"t":"2022-01-12 07:45", "v":"8.101", "type":"L"},{"t":"2022-01-12 12:51", "v":"13.639", "type":"H"},{"t":"2022-01-12 20:33", "v":"0.666", "type":"L"}]}"#;

    const prediction: &str = r#"{ "predictions" : [{"t":"2022-05-27 03:49", "v":"14.036", "type":"H"},{"t":"2022-05-27 11:02", "v":"-0.058", "type":"L"},{"t":"2022-05-27 17:51", "v":"12.111", "type":"H"},{"t":"2022-05-27 23:02", "v":"5.967", "type":"L"},{"t":"2022-05-28 04:17", "v":"13.733", "type":"H"},{"t":"2022-05-28 11:32", "v":"-0.922", "type":"L"},{"t":"2022-05-28 18:42", "v":"12.991", "type":"H"},{"t":"2022-05-28 23:53", "v":"6.817", "type":"L"},{"t":"2022-05-29 04:44", "v":"13.355", "type":"H"},{"t":"2022-05-29 12:01", "v":"-1.516", "type":"L"},{"t":"2022-05-29 19:25", "v":"13.647", "type":"H"}
    ]}"#;

    #[test]
    fn json_parse() {
        let p: Predictions = serde_json::from_str(predicted_json_data).unwrap();
        println!("{:?}", p);

        let tidepoints: Vec<TidePoint> = p
            .predictions
            .iter()
            .map(|t| t.to_tide_point().unwrap())
            .collect();
        tidepoints.iter().for_each(|t| println!("{:?}", t));

        let x = "2022-01-11 19:51";
        let today = chrono::NaiveDateTime::parse_from_str(x, "%Y-%m-%d %H:%M").unwrap();
        let n = Utc::from_local_datetime(&Utc, &today)
            .unwrap()
            .with_hour(0)
            .unwrap()
            .with_minute(0)
            .unwrap()
            .with_second(0)
            .unwrap();
        //let dt = DateTime::from_utc(today, *n.offset());

        println!("today: {} utc-dt: {}", today, n);
        let chart_points = coordinates_from_prediction(tidepoints, n);
        println!("{:?}", chart_points);
    }

    #[test]
    fn noaa_prediction_datetime_parse() {
        let x = "2022-01-11 19:51";
        let t = chrono::NaiveDateTime::parse_from_str(x, "%Y-%m-%d %H:%M");

        match t {
            Err(e) => println!("error: {}", e),
            Ok(t) => println!("{:?}", t),
        }
    }

    #[test]
    fn local_ne_utc() {
        let l = Local::now();
        let u = Utc::now();
        println!("local: {} utc: {}", l, u);
        assert_ne!(l.timestamp(), u.timestamp())
    }
}
