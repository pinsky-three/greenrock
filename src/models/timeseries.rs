use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Candle {
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
    pub timestamp: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeSeries {
    pub candles: Vec<Candle>,
}
