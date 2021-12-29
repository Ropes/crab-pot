mod utils;

use chrono::prelude::*;
use chrono::{DateTime, Utc};
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

#[derive(PartialEq, Debug)]
pub enum Tide {
    High,
    Low,
}

#[derive(Debug, PartialEq)]
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

fn parse_tide_tuple(s: &str) -> Result<TidePoint, Box<dyn Error>> {
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
            log!("BAILING ON: {:?}", s);
            bail!("invalid number of elements parsed from tide tuple")
        }
    };
}

fn parse_time_string(today: DateTime<Utc>, s: &str) -> DateTime<Utc> {
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
    use crate::*;
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
