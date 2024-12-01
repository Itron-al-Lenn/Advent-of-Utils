use crate::error::AocError;
use chrono::{Datelike, FixedOffset};

fn get_ets() -> chrono::DateTime<FixedOffset> {
    chrono::DateTime::from_naive_utc_and_offset(
        chrono::Local::now().naive_utc(),
        FixedOffset::west_opt(5 * 3600).unwrap(),
    )
}

pub fn get_max_year() -> i32 {
    let ets = get_ets();
    match ets.month() {
        12 => ets.year(),
        _ => ets.year() - 1,
    }
}

pub fn get_max_day(year: i32) -> u8 {
    let ets = get_ets();
    match year {
        year if year > get_max_year() => 0,
        year if year == ets.year() && 12 == ets.month() => {
            let day = ets.day();
            day as u8
        }
        _ => 25,
    }
}

pub fn validate_year(year: i32) -> Result<(), AocError> {
    match year {
        year if year > get_max_year() => Err(AocError::InvalidYear(year)),
        year if year < 2015 => Err(AocError::InvalidYear(year)),
        _ => Ok(()),
    }
}

pub fn validate_date(year: i32, day: u8) -> Result<(), AocError> {
    validate_year(year)?;
    match day {
        day if day > get_max_day(year) => Err(AocError::InvalidDay(year, day)),
        _ => Ok(()),
    }
}
