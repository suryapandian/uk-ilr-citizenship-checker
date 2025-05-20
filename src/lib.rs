use wasm_bindgen::prelude::*;
use chrono::{NaiveDate, Duration};

pub struct TravelPeriod {
    start: NaiveDate,
    end: NaiveDate,
}

impl TravelPeriod {
    pub fn new(start: &str, end: &str) -> Result<Self, chrono::ParseError> {
        Ok(Self {
            start: NaiveDate::parse_from_str(start, "%Y-%m-%d")?,
            end: NaiveDate::parse_from_str(end, "%Y-%m-%d")?,
        })
    }
}

#[wasm_bindgen]
pub fn check_ilr_eligibility(arrival_str: &str, travel_periods: js_sys::Array) -> Result<bool, JsValue> {
    let arrival = NaiveDate::parse_from_str(arrival_str, "%Y-%m-%d")
        .map_err(|e| JsValue::from_str(&format!("Invalid arrival date: {}", e)))?;

    let ilr_date = arrival + Duration::days(5 * 365); // approx 5 years

    // Parse travel_periods from JS Array< [start: string, end: string] >
    let mut periods = Vec::new();
    for val in travel_periods.iter() {
        let arr = js_sys::Array::from(&val);
        if arr.length() != 2 {
            return Err(JsValue::from_str("Each travel period must be an array of two strings"));
        }
        let start = arr.get(0).as_string().ok_or_else(|| JsValue::from_str("Start date must be a string"))?;
        let end = arr.get(1).as_string().ok_or_else(|| JsValue::from_str("End date must be a string"))?;

        let tp = TravelPeriod::new(&start, &end).map_err(|e| JsValue::from_str(&format!("Invalid date format: {}", e)))?;
        periods.push(tp);
    }

    // Check rolling 12-month windows
    let mut day = arrival;
    while day <= ilr_date {
        let window_start = day - Duration::days(365);
        let mut days_abroad = 0;
        for p in &periods {
            if p.end >= window_start && p.start <= day {
                let overlap_start = if p.start < window_start { window_start } else { p.start };
                let overlap_end = if p.end > day { day } else { p.end };
                days_abroad += (overlap_end - overlap_start).num_days() + 1;
            }
        }
        if days_abroad > 180 {
            return Ok(false);
        }
        day += Duration::days(30);
    }
    Ok(true)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ilr_eligibility() {
        let arrival = "2021-11-01";
        let periods = vec![
            TravelPeriod::new("2022-10-28", "2023-02-13"),
            TravelPeriod::new("2023-11-24", "2024-03-13"),
            TravelPeriod::new("2024-12-09", "2025-03-29"),
        ];
        let mut js_periods = js_sys::Array::new();
        for p in &periods {
            js_periods.push(&p.into());
        }
        assert!(check_ilr_eligibility(arrival, js_periods));
    }
}
