use std::collections::HashMap;

use chrono::{DateTime, Utc};

pub trait Broker {
    fn balance(&self) -> HashMap<String, f64>;
    fn market_current_price(&self, symbol: &str) -> f64;
    fn candle_stream(
        &self,
        symbol: &str,
        interval: &str,
    ) -> tokio::sync::broadcast::Receiver<crate::models::timeseries::Candle>;
    async fn candles(
        &self,
        symbol: &str,
        interval: &str,
        limit: u16,
        from: Option<DateTime<Utc>>,
        to: Option<DateTime<Utc>>,
    ) -> Vec<crate::models::timeseries::Candle>;
}
