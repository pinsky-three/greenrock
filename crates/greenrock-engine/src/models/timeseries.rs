use std::{collections::HashMap, time::Duration};

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
    pub ts: DateTime<Utc>,
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

pub struct CandleRing {
    cap: usize,
    buf: Vec<Option<Candle>>,             // fixed storage
    start: usize,                         // index of oldest
    len: usize,                           // number of valid items
    index: HashMap<DateTime<Utc>, usize>, // ts -> slot
}

impl CandleRing {
    pub fn new(cap: usize) -> Self {
        assert!(cap > 0);
        Self {
            cap,
            buf: vec![None; cap],
            start: 0,
            len: 0,
            index: HashMap::with_capacity(cap * 2),
        }
    }

    /// Insert or update by timestamp. Overwrites oldest when at capacity.
    pub fn upsert(&mut self, c: Candle) {
        if let Some(&slot) = self.index.get(&c.ts) {
            self.buf[slot] = Some(c); // update in place
            return;
        }

        let slot = if self.len < self.cap {
            // append at end
            let slot = (self.start + self.len) % self.cap;
            self.len += 1;
            slot
        } else {
            // evict oldest (at start)
            let slot = self.start;
            // remove old index entry
            if let Some(old) = self.buf[slot].as_ref() {
                self.index.remove(&old.ts);
            }
            self.start = (self.start + 1) % self.cap;
            slot
        };

        self.index.insert(c.ts, slot);
        self.buf[slot] = Some(c);
    }

    pub fn get(&self, ts: DateTime<Utc>) -> Option<&Candle> {
        self.index
            .get(&ts)
            .and_then(|&slot| self.buf[slot].as_ref())
    }

    pub fn last(&self) -> Option<&Candle> {
        if self.len == 0 {
            return None;
        }
        let last_idx = (self.start + self.len - 1) % self.cap;
        self.buf[last_idx].as_ref()
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Chronological iterator (oldest → newest).
    pub fn iter(&self) -> impl Iterator<Item = &Candle> {
        (0..self.len).filter_map(move |i| {
            let idx = (self.start + i) % self.cap;
            self.buf[idx].as_ref()
        })
    }

    pub fn snapshot(&self) -> Vec<Candle> {
        self.iter().cloned().collect()
    }
}
