use crate::utils::*;
use crate::DrawResult;
use bacon_sci::interp::lagrange;
use bacon_sci::polynomial::Polynomial;
use chrono::prelude::*;
use chrono::{DateTime, Duration, Utc};
use plotters;
use plotters::coord::types::RangedCoordf32;
use plotters::prelude::*;
use plotters_canvas::CanvasBackend;
use simple_error::bail;
use std::error::Error;
use wasm_bindgen::prelude::*;

extern crate web_sys;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

// Call the JS alert() callback.
#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
extern "C" {
    unsafe fn alert(s: &str);
}

#[wasm_bindgen]
pub fn tide_alert(s: &str) {
    set_panic_hook();
    let res = parse_noaa_tides(s).unwrap();
    let tides: String = res.iter().map(|t| t.to_string()).collect();
    unsafe {
        log!("{:?}", tides);
    }

    unsafe {
        alert(s);
    }
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
) -> DrawResult<impl Fn((i32, i32)) -> Option<(f32, f32)>> {
    let backend = CanvasBackend::new(canvas_id).expect("cannot find canvas");
    let root = backend.into_drawing_area();
    let font: FontDesc = ("sans-serif", 20.0).into();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .margin(20)
        .caption(format!("Tides on McMicken Island"), font)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0f32..24f32, -8f32..20f32)?;

    chart.configure_mesh().x_labels(3).y_labels(3).draw()?;

    // Dot and label each high/low tide from TidePoint
    /*
    let dot_and_label = |x: f32, y: f32| {
        return EmptyElement::at((x, y))
            + Circle::new((0, 0), 3, ShapeStyle::from(&BLACK).filled())
            + Text::new(
                format!("({:.2},{:.2})", x, y),
                (10, 0),
                ("sans-serif", 15.0).into_font(),
            );
    };
    chart.draw_series(LineSeries::new(
        tv.iter().map(|t| {
            let (x, y) = t.to_xy();
            (x, y)
        }),
        &BLACK,
    ))?;
    */

    chart.draw_series(tv.iter().map(|t| {
        let (x, y) = t.to_xy();
        return Circle::new((x, y), 3, ShapeStyle::from(&RED));
    }))?;
    let mut poly_vec = Vec::<(f32, f32)>::new();
    tv.iter().for_each(|t| {
        let z = t.to_xy();
        poly_vec.push(z);
    });

    let XYs = points_from_vec(tv.to_owned());
    log!("XYs read: {:?}", XYs.len());
    chart.draw_series(LineSeries::new(
        XYs.iter().map(|(x, y)| (x.clone(), y.clone())),
        &BLUE,
    ))?;

    chart.draw_series(PointSeries::of_element(
        tv.to_owned().iter().map(|t| t.to_xy()),
        5,
        ShapeStyle::from(&RED).filled(),
        &|coord, size, style| {
            EmptyElement::at(coord)
                + Circle::new((0, 0), size, style)
                + Text::new(format!("{:?}", coord), (0, 15), ("sans-serif", 15))
        },
    ))?;

    /*
    chart.draw_series(PointSeries::of_element(
        tv.iter().map(|t| {
            let (x, y) = t.to_xy();
            (x, y)
        }),
        5,
        &RED,
        |c, s, style| {
            return &EmptyElement::at(c)
        },
    ))?;
    */

    /*
    chart.draw_series(tv.iter().map(|t| {
        let (x, y) = t.to_xy();
        let txt = Text::new(
            format!("({:.2},{:.2})", x, y),
            (10, 0),
            ("sans-serif", 15.0).into_font(),
        );
        txt
    }))?;
    */

    root.present()?;
    return Ok(chart.into_coord_trans());
}

fn points_from_vec(tv: Vec<TidePoint>) -> Vec<(f32, f32)> {
    // Interpolate series of points between Tide points
    // TODO: Approximate the previous days final tide, and first tide of next day
    let today_start = tv[0].dt.date().clone().and_hms(0, 0, 0);
    let tomorrow_start = today_start + Duration::days(1);
    let first_t = tv[0].dt;
    let last_t = tv[3].dt;
    let zero_to_first = first_t - today_start;
    let last_to_24 = tomorrow_start - last_t;
    let tide_space = zero_to_first + last_to_24;

    unsafe {
        log!("next tide timing: {:?} for {:?}", tide_space, today_start);
    }
    let yesterday_last_tide = TidePoint::new(tv[0].dt - tide_space, tv[3].level, tv[3].tide);
    let tomorrow_first_tide = TidePoint::new(tv[3].dt + tide_space, tv[0].level, tv[0].tide);
    unsafe {
        log!("yesterday tide: {:?}", yesterday_last_tide);
        log!("tomorrow tide: {:?}", tomorrow_first_tide);
    }

    let mut poly_vec = Vec::<&TidePoint>::new();
    tv.iter().for_each(|t| poly_vec.push(t));

    let mut xs: Vec<f32> = Vec::new();
    let mut ys: Vec<f32> = Vec::new();
    // Insert prev day's tide
    let (x, y) = yesterday_last_tide.to_xy();
    xs.push(x - 24.0f32);
    ys.push(y);
    poly_vec.iter().for_each(|t| {
        let (x, y) = t.to_xy();
        xs.push(x);
        ys.push(y);
    });
    // Push next day's tide
    let (x, y) = tomorrow_first_tide.to_xy();
    xs.push(x + 24.0f32);
    ys.push(y);
    xs.iter().for_each(|x| log!("x: {:?}", x));

    let mut tideX: Vec<f32> = Vec::new();
    let mut tideY: Vec<f32> = Vec::new();

    for i in 1..xs.len() {
        let x_origin = xs[i - 1]; // x0 ...
        let y_origin = ys[i - 1]; // y0 ...
        log!("x_origin: {:?} y_origin: {:?}", x_origin, y_origin);
        tideX.push(x_origin);
        tideY.push(y_origin);

        let y_delta = ys[i] - y_origin;
        let x_delta = xs[i] - x_origin;
        log!("x_delta: {:?} y_delta: {:?}", x_delta, y_delta);

        let x_inc = x_delta / 25f32;

        let mut x_step = x_origin + x_inc;
        while x_step < xs[i] {
            tideX.push(x_step);
            // calculate Y
            // --------------------------
            let x_percentage = ((x_step - x_origin) / x_delta);
            let to_cosine = ((x_percentage * std::f32::consts::PI) + std::f32::consts::PI);
            let y_multiplier = (to_cosine.cos() + 1f32) / 2f32;
            let y_val = (y_multiplier * y_delta) + y_origin;
            tideY.push(y_val);
            //log!("x: {:?}, y: {:?}", x_step, y_val);
            x_step += x_inc;
        }
    }

    let mut ret = Vec::<(f32, f32)>::new();
    for (_, (x, y)) in tideX.iter().zip(tideY.iter()).enumerate() {
        ret.push((x.clone(), y.clone()));
    }

    // calculate the prev-next day tide X offsets and pre-append to the xs,ys
    //let poly = lagrange(&tideX, &tideY, 1e-6).unwrap();
    //poly
    ret
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Tide {
    High,
    Low,
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

// Query: https://tidesandcurrents.noaa.gov/cgi-bin/stationtideinfo.cgi?Stationid=9446583
// Example data: "3:35 AM|0.6|low 10:23 AM|12.3|high 4:06 PM|6.9|low 9:11 PM|11.7|high 9:11 PM|high";
/*
3:08 AM|0.5|low
10:21 AM|15.1|high
4:57 PM|6.4|low
9:18 PM|9.9|high
9:18 PM|high*/
pub fn parse_noaa_tides(s: &str) -> Result<Vec<TidePoint>, Box<dyn Error>> {
    let split_strings: Vec<&str> = s.lines().collect();

    split_strings.iter().map(|p| println!("{}", p)).count();

    let tide_parts: Vec<TidePoint> = split_strings
        .iter()
        .filter_map(|part| parse_tide_tuple(part).ok())
        .collect();

    Ok(tide_parts)
}

//pub fn unwrap_noaa_tides(s: &str) -> &str {}

pub fn parse_tide_tuple(s: &str) -> Result<TidePoint, Box<dyn Error>> {
    let parts: Vec<&str> = s.trim().split('|').collect();
    return match parts.len() {
        3 => {
            let dt = Utc::now();
            let ts = parse_time_string(dt, parts[0]);
            let level = parts[1].parse::<f32>()?;

            let tide = match parts[2] {
                "low" => TidePoint::new(ts, level, Tide::Low),
                "high" => TidePoint::new(ts, level, Tide::High),
                _ => bail!("tide term invalid; not 'high'||'low'"),
            };
            Ok(tide)
        }
        _ => {
            //log!("BAILING ON: {:?}", s);
            bail!("invalid number of elements parsed from tide tuple")
        }
    };
}

pub fn parse_time_string(today: DateTime<Utc>, s: &str) -> DateTime<Utc> {
    let ts_parts: Vec<&str> = s.split_whitespace().collect();

    // Parse the Hours, Minutes, and Meridian
    let h_m: Vec<&str> = ts_parts[0].split(':').collect();
    let h: u32 = match h_m[0].parse::<u32>().unwrap() {
        12 => 0,
        _ => h_m[0].parse::<u32>().unwrap(),
    };
    let m: u32 = h_m[1].parse::<u32>().unwrap();
    let meridian: u32 = match ts_parts[1] {
        "PM" => 12,
        _ => 0,
    };
    let h_adjusted = h + meridian;

    /*log!(
        "parse_time{:?}: {:?} parts:{:?} h_adjusted: {:?}",
        today,
        s,
        ts_parts,
       h_adjusted,
    );*/

    let dt = today
        .with_hour(h_adjusted)
        .unwrap()
        .with_minute(m)
        .unwrap()
        .with_second(0)
        .unwrap()
        .with_nanosecond(0)
        .unwrap();
    dt
}
#[cfg(test)]
mod tests {
    use crate::tides::*;
    use chrono::prelude::*;
    use chrono::{DateTime, TimeZone, Utc};

    //const tide_data: &str ="3:35 AM|0.6|low 10:23 AM|12.3|high 4:06 PM|6.9|low 9:11 PM|11.7|high 9:11 PM|high";
    const tide_data: &str = r#"3:08 AM|0.5|low 
10:21 AM|15.1|high 
4:57 PM|6.4|low 
9:18 PM|9.9|high 
9:18 PM|high"#;
    const special_12_data: &str = r#"12:15 AM|9.7|high 
5:46 AM|4.8|low 
12:14 PM|15.1|high 
7:32 PM|1.9|low 
7:32 PM|low"#;
    const weird_12_data: &str = r#"12:15 AM|9.7|high 
5:46 AM|4.8|low 
12:14 PM|15.1|high 
7:32 PM|1.9|low 
1:49 AM|high|NH"#;

    const single_data: &str = "3:35 AM|0.6|low";
    const tide_single: &str = "10:21 AM|15.1|high ";

    // full ts
    const ts_data: &str = "3:35 AM";
    const ts_pm_data: &str = "9:11 PM";

    #[test]
    fn test_parse_ts() {
        let today = Utc.ymd(2021, 9, 27).and_hms(0, 0, 0);
        let res = parse_time_string(today, ts_data);
        assert_eq!(res.to_rfc2822(), "Mon, 27 Sep 2021 03:35:00 +0000");
    }

    #[test]
    fn test_parse_ts_pm() {
        let today = Utc.ymd(2021, 9, 27).and_hms(0, 0, 0);
        let res = parse_time_string(today, ts_pm_data);
        assert_eq!(res.to_rfc2822(), "Mon, 27 Sep 2021 21:11:00 +0000");
    }
    #[test]
    fn test_single_parse_ok() {
        let ret = parse_tide_tuple(single_data);
        assert!(ret.is_ok());
        let val = ret.unwrap();
        assert_eq!(val.level, 0.6);
        assert_eq!(val.tide, Tide::Low);
    }

    #[test]
    fn test_single_tide_ok() {
        let ret = parse_tide_tuple(tide_single);
        assert!(ret.is_ok());
        let val = ret.unwrap();
        assert_eq!(val.level, 15.1);
        assert_eq!(val.tide, Tide::High);
    }

    #[test]
    fn test_full_parse() {
        let res = parse_noaa_tides(tide_data);
        assert!(res.is_ok());

        let tides = res.unwrap();
        println!("{:?}", tides);
        //assert_eq!(t.level, 15.1);
    }

    #[test]
    fn test_0000_12_case() {
        let res = parse_noaa_tides(special_12_data);
        assert!(res.is_ok());

        let tides = res.unwrap();
        println!("{:?}", tides);
    }
    #[test]
    fn test_weird_12_case() {
        let res = parse_noaa_tides(weird_12_data);
        assert!(res.is_ok());

        let tides = res.unwrap();
        println!("{:?}", tides);
    }
}

/*
fn parse_noaa_cgi(s: &str) -> Vec<Tide> {
    let data_tuples = *s.split_whitespace().collect().map().collect();
}
*/
