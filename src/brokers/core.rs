use std::collections::HashMap;

use binance::model::{Order, OrderBook, TradeHistory};
use chrono::{DateTime, Utc};

pub trait Broker {
    fn balance(&self) -> HashMap<String, f64>;
    fn market_current_price(&self, symbol: &str) -> f64;
    fn candle_stream(
        &self,
        symbol: &str,
        interval: &str,
    ) -> tokio::sync::broadcast::Receiver<crate::models::timeseries::Candle>;
    fn candles(
        &self,
        symbol: &str,
        interval: &str,
        limit: u16,
        from: Option<DateTime<Utc>>,
        to: Option<DateTime<Utc>>,
    ) -> impl std::future::Future<Output = Vec<crate::models::timeseries::Candle>> + Send;
    fn open_orders(&self, symbol: &str) -> Vec<Order>;
    fn trade_history(&self, symbol: &str) -> Vec<TradeHistory>;
    fn order_book(&self, symbol: &str, depth: u64) -> OrderBook;
    fn order_book_stream(&self, symbol: &str) -> tokio::sync::broadcast::Receiver<OrderBook>;
}
