use super::{AocOption, Parts};
use std::time::Duration;

pub struct AocResult {
    day: u8,
    part: Parts,
    result: AocOption,
    time: Option<Duration>,
}

impl AocResult {
    pub fn new(day: u8, part: u8, result: AocOption) -> Self {
        Self {
            day,
            part: Parts::new(part).unwrap(),
            result,
            time: None,
        }
    }

    pub fn with_timing(mut self, duration: Duration) -> Self {
        self.time = Some(duration);
        self
    }

    pub fn day(&self) -> u8 {
        self.day
    }
    pub fn part(&self) -> Parts {
        self.part.clone()
    }
    pub fn result(&self) -> &AocOption {
        &self.result
    }
    pub fn time(&self) -> Option<Duration> {
        self.time
    }
}
