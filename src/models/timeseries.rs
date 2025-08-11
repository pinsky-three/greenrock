use std::time::Duration;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
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

pub struct FramedCandles<I: IntervalTrait, S: SymbolTrait> {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub candles: Vec<Candle>,

    pub interval: I,
    pub symbol: S,
}

pub trait IntervalTrait {
    fn to_string(&self) -> String;
    fn to_duration(&self) -> Duration;
}

pub trait SymbolTrait {
    fn to_string(&self) -> String;
}

// pub enum IntervalEnum {
//     OneSecond,
//     OneMinute,
//     FiveMinutes,
//     FifteenMinutes,
//     ThirtyMinutes,
//     OneHour,
//     FourHours,
//     EightHours,
//     TwelveHours,
//     OneDay,
//     OneWeek,
//     OneMonth,
//     OneYear,
// }
