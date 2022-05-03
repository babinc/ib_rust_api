use core::fmt;
use std::fmt::Formatter;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
pub enum BarColors {
    Green,
    Yellow,
    Red
}

#[derive(Debug, Clone, Serialize)]
pub struct Bar {
    pub time_str: String,
    pub time_int: i64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
    pub wap: f64,
    pub count: f64,
    pub color: Option<BarColors>
}

impl Bar {
    pub fn new() -> Self {
        Bar {
            time_str: "".to_string(),
            time_int: 0,
            open: 0.0,
            high: 0.0,
            low: 0.0,
            close: 0.0,
            volume: 0.0,
            wap: 0.0,
            count: 0.0,
            color: None
        }
    }
}

impl fmt::Display for Bar {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut color_str = "".to_string();
        if self.color.is_none() {
            color_str = "None".to_string();
        }
        if let Some(color) = &self.color {
            match color {
                BarColors::Green => color_str = "Green".to_string(),
                BarColors::Yellow => color_str = "Yellow".to_string(),
                BarColors::Red => color_str = "Red".to_string()
            }
        }

        write!(f, "time_str: {}, time_int, {}, open: {}, high: {}, low: {}, close: {}, volume: {}, wap: {}, count: {}, color: {}", self.time_str, self.time_int, self.open, self.high, self.low, self.close, self.volume, self.wap, self.count, color_str)
    }
}