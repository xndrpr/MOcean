use std::time::{Duration, Instant};

pub struct Cursor {
    pub x: f64,
    pub y: f64,
    pub blink_interval: Duration,
    pub last_blink: Instant,
    pub visibility: bool,
}

impl Cursor {
    pub fn new(x: f64, y: f64, interval: Duration) -> Self {
        Self {
            x: x,
            y: y,
            blink_interval: interval,
            last_blink: Instant::now(),
            visibility: true,
        }
    }
}
