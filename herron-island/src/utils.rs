use chrono::prelude::*;
use chrono::{DateTime, TimeZone, Utc};
use std::time::SystemTime;
use std::error::Error;
use simple_error::bail;

#[derive(PartialEq, Debug)]
pub enum Tide{
    High,
    Low,
}

#[derive(PartialEq, Debug)]
pub struct TidePoint {
    dt: DateTime<Utc>,
    level: f32,
    tide: Tide,
}

impl TidePoint{
    pub fn new(dt: DateTime<Utc>, level: f32, tide: Tide) -> TidePoint{
        TidePoint{
            dt: dt,
            level: level,
            tide: tide,
        }
    }
}

fn parse_tide_tuple(s: &str) -> Result<TidePoint, Box<Error>> {
    let parts: Vec<&str> = s.split('|').collect();
    return match parts.len() {
        3 => {

            let dt = Utc::now();
            let ts = parse_time_string(dt, parts[0]);
            let level = parts[1].parse::<f32>().unwrap();

            let tide =  match parts[2] {
                "low" => TidePoint::new(dt, level, Tide::Low),
                "high" => TidePoint::new(dt, level, Tide::High),
                _ => bail!("tide term invalid; not 'high'||'low'")
            };
            Ok(tide)
        }
        _ => bail!("invalid number of elements parsed from tide tuple") 
    };
}

fn parse_time_string(today: DateTime<Utc>, s: &str) -> DateTime<Utc> {
    let ts_parts: Vec<&str> = s.split_whitespace().collect();

    // Parse the Hours, Minutes, and Meridian
    let h_m: Vec<&str> = ts_parts[0].split(':').collect();
    let h: u32 = h_m[0].parse::<u32>().unwrap();
    let m: u32 = h_m[1].parse::<u32>().unwrap();
    let meridian: u32 = match ts_parts[1] {
        "PM" => 12,
        _ => 0,
    };
    let h_adjusted = h + meridian;

    //
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
    use crate::utils::*;
    use chrono::prelude::*;
    use chrono::{DateTime, TimeZone, Utc};

    const tide_data: &str =
        "3:35 AM|0.6|low 10:23 AM|12.3|high 4:06 PM|6.9|low 9:11 PM|11.7|high 9:11 PM|high";
    const single_data: &str = "3:35 AM|0.6|low";
    // full ts
    const ts_data: &str = "3:35 AM";
    const ts_pm_data: &str = "9:11 PM";
    

    #[test]
    fn test_parse_ts() {
        let today = Utc.ymd(2021, 9, 27).and_hms(0,0,0);
        let res = parse_time_string(today, ts_data);
        assert_eq!(res.to_rfc2822(), "Mon, 27 Sep 2021 03:35:00 +0000");
    }

    #[test]
    fn test_parse_ts_pm() {
        let today = Utc.ymd(2021, 9, 27).and_hms(0,0,0);
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
}

/*
fn parse_noaa_cgi(s: &str) -> Vec<Tide> {
    let data_tuples = *s.split_whitespace().collect().map().collect();
}
*/

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}
