use wasm_bindgen::prelude::*;
use chrono::{NaiveDate, Duration};

#[wasm_bindgen]
pub struct TravelPeriod {
    start: NaiveDate,
    end: NaiveDate,
}

#[wasm_bindgen]
impl TravelPeriod {
    #[wasm_bindgen(constructor)]
    pub fn new(start: &str, end: &str) -> TravelPeriod {
        TravelPeriod {
            start: NaiveDate::parse_from_str(start, "%Y-%m-%d").unwrap(),
            end: NaiveDate::parse_from_str(end, "%Y-%m-%d").unwrap(),
        }
    }

    pub fn days(&self) -> i64 {
        (self.end - self.start).num_days() + 1
    }
}

#[wasm_bindgen]
pub fn check_ilr_eligibility(arrival_str: &str, periods_js: js_sys::Array) -> bool {
    let arrival = NaiveDate::parse_from_str(arrival_str, "%Y-%m-%d").unwrap();
    let ilr_date = arrival + Duration::days(5*365); // approx 5 years

    // Convert JS array to Vec<TravelPeriod>
    let mut periods = Vec::new();
    for period in periods_js.iter() {
        let tp = period.unchecked_into::<TravelPeriod>();
        periods.push(tp);
    }

    // Check rolling 12-month windows (simplified: check once a month)
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
            return false;
        }
        day += Duration::days(30); // move one month at a time
    }
    true
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
