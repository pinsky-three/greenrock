use std::collections::HashMap;
use std::env;

use binance::model::{KlineSummaries, Order, OrderBook, TradeHistory};
use binance::{account::Account, api::Binance, market::Market};
use chrono::{DateTime, Utc};
use tracing::{error, info};

use crate::brokers::core::Broker;
use crate::models::timeseries::Candle;

#[derive(Clone)]
pub struct BinanceBroker {}

use futures_util::{SinkExt, StreamExt};
use serde::Deserialize;
use tokio::sync::broadcast;
use tokio::time::{Duration, sleep};
use tokio_tungstenite::tungstenite::Message;

impl Broker for BinanceBroker {
    fn balance(&self) -> HashMap<String, f64> {
        // Note: This method should ideally be async, but the trait requires sync
        // The caller should wrap this in spawn_blocking
        let api_key = env::var("BINANCE_API_KEY").ok();
        let secret_key = env::var("BINANCE_SECRET_KEY").ok();

        if api_key.is_none() || secret_key.is_none() {
            error!("Binance API credentials not found");
            return HashMap::new();
        }

        let account: Account = Binance::new(api_key, secret_key);

        match account.get_account() {
            Ok(answer) => {
                let res = answer
                    .balances
                    .iter()
                    .filter(|instrument| instrument.free.parse::<f64>().unwrap_or(0.0) > 0.0)
                    .collect::<Vec<_>>();

                res.iter()
                    .map(|instrument| {
                        (
                            instrument.asset.clone(),
                            instrument.free.parse::<f64>().unwrap_or(0.0),
                        )
                    })
                    .collect()
            }
            Err(e) => {
                error!("Failed to get balance: {}", e);
                HashMap::new()
            }
        }
    }

    fn market_current_price(&self, symbol: &str) -> f64 {
        let market: Market = Binance::new(None, None);
        match market.get_price(symbol) {
            Ok(price) => price.price,
            Err(e) => {
                error!("Failed to get market price for {}: {}", symbol, e);
                0.0
            }
        }
    }

    fn candle_stream(
        &self,
        symbol: &str,
        interval: &str,
    ) -> broadcast::Receiver<crate::models::timeseries::Candle> {
        let (tx, rx) = broadcast::channel::<Candle>(1024);
        let symbol = symbol.to_lowercase();

        let interval = interval.to_string();

        tokio::spawn(async move {
            let mut backoff = Duration::from_secs(1);
            let max_backoff = Duration::from_secs(60);

            loop {
                let url = format!("wss://stream.binance.com:9443/ws/{symbol}@kline_{interval}");

                match tokio_tungstenite::connect_async(&url).await {
                    Ok((mut ws, _resp)) => {
                        // Reset backoff on successful connect
                        backoff = Duration::from_secs(1);

                        while let Some(msg) = ws.next().await {
                            match msg {
                                Ok(Message::Text(text)) => {
                                    if let Ok(candle) = parse_kline(&text) {
                                        let _ = tx.send(candle);
                                    }
                                }
                                Ok(Message::Binary(_)) => {}
                                Ok(Message::Ping(p)) => {
                                    let _ = ws.send(Message::Pong(p)).await;
                                }
                                Ok(Message::Pong(_)) => {}
                                Ok(Message::Close(_)) => break,
                                Ok(Message::Frame(_)) => {}
                                Err(e) => {
                                    eprintln!("binance ws error: {e}");
                                    break;
                                }
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("binance connect error: {e}");
                    }
                }

                sleep(backoff).await;
                backoff = (backoff * 2).min(max_backoff);
            }
        });

        rx
    }

    async fn candles(
        &self,
        symbol: &str,
        interval: &str,
        limit: u16,
        from: Option<DateTime<Utc>>,
        to: Option<DateTime<Utc>>,
    ) -> Vec<Candle> {
        // let market: Market = Binance::new(None, None);

        let symbol = symbol.to_string();
        let interval = interval.to_string();

        tokio::task::spawn_blocking(move || {
            let start_ms = from.map(|f| f.timestamp_millis() as u64);
            let end_ms = to.map(|t| t.timestamp_millis() as u64);

            let symbol = symbol.to_uppercase();

            if let Some(start_ms) = start_ms
                && let Some(end_ms) = end_ms
            {
                info!("fetching candles for {symbol} from {start_ms} to {end_ms}");
            } else {
                info!("fetching latest {limit} candles for {symbol}");
            }

            let market: Market = Binance::new(None, None);

            match market.get_klines(&symbol, &interval, limit, start_ms, end_ms) {
                Ok(KlineSummaries::AllKlineSummaries(summaries)) => summaries
                    .into_iter()
                    .map(|k| Candle {
                        open: k.open.parse().unwrap_or(0.0),
                        high: k.high.parse().unwrap_or(0.0),
                        low: k.low.parse().unwrap_or(0.0),
                        close: k.close.parse().unwrap_or(0.0),
                        volume: k.volume.parse().unwrap_or(0.0),
                        timestamp: k.close_time,
                        ts: DateTime::from_timestamp_millis(k.close_time).unwrap(),
                    })
                    .collect(),
                Err(e) => {
                    error!("failed to fetch klines: {e}");
                    Vec::new()
                }
            }
        })
        .await
        .unwrap()
    }

    fn open_orders(&self, symbol: &str) -> Vec<Order> {
        let api_key = env::var("BINANCE_API_KEY").ok();
        let secret_key = env::var("BINANCE_SECRET_KEY").ok();

        if api_key.is_none() || secret_key.is_none() {
            error!("Binance API credentials not found");
            return Vec::new();
        }

        let account: Account = Binance::new(api_key, secret_key);
        match account.get_open_orders(symbol) {
            Ok(orders) => orders,
            Err(e) => {
                error!("Failed to get open orders: {}", e);
                Vec::new()
            }
        }
    }

    fn trade_history(&self, symbol: &str) -> Vec<TradeHistory> {
        let api_key = env::var("BINANCE_API_KEY").ok();
        let secret_key = env::var("BINANCE_SECRET_KEY").ok();

        if api_key.is_none() || secret_key.is_none() {
            error!("Binance API credentials not found");
            return Vec::new();
        }

        let account: Account = Binance::new(api_key, secret_key);
        match account.trade_history(symbol) {
            Ok(history) => history,
            Err(e) => {
                error!("Failed to get trade history: {}", e);
                Vec::new()
            }
        }
    }

    fn order_book(&self, symbol: &str, depth: u64) -> OrderBook {
        let market: Market = Binance::new(None, None);
        match market.get_custom_depth(symbol, depth) {
            Ok(book) => book,
            Err(e) => {
                error!("Failed to get order book: {}", e);
                OrderBook {
                    last_update_id: 0,
                    bids: vec![],
                    asks: vec![],
                }
            }
        }
    }

    fn order_book_stream(&self, symbol: &str) -> broadcast::Receiver<OrderBook> {
        let (tx, rx) = broadcast::channel::<OrderBook>(1024);
        let symbol = symbol.to_lowercase();

        let market: Market = Binance::new(None, None);
        let _stream = market.get_depth(symbol.clone());

        tokio::spawn(async move {
            let mut backoff = Duration::from_secs(1);
            let max_backoff = Duration::from_secs(60);

            loop {
                let url = format!("wss://stream.binance.com:9443/ws/{symbol}@depth");

                match tokio_tungstenite::connect_async(&url).await {
                    Ok((mut ws, _resp)) => {
                        // Reset backoff on successful connect
                        backoff = Duration::from_secs(1);

                        while let Some(msg) = ws.next().await {
                            match msg {
                                Ok(Message::Text(text)) => {
                                    if let Ok(book) = parse_order_book(&text) {
                                        let _ = tx.send(book);
                                    }
                                }
                                Ok(Message::Binary(_)) => {}
                                Ok(Message::Ping(p)) => {
                                    let _ = ws.send(Message::Pong(p)).await;
                                }
                                Ok(Message::Pong(_)) => {}
                                Ok(Message::Close(_)) => break,
                                Ok(Message::Frame(_)) => {}
                                Err(e) => {
                                    eprintln!("binance ws error: {e}");
                                    break;
                                }
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("binance connect error: {e}");
                    }
                }

                sleep(backoff).await;
                backoff = (backoff * 2).min(max_backoff);
            }
        });

        rx
    }
}

impl BinanceBroker {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for BinanceBroker {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Deserialize)]
struct WsEnvelope {
    // #[serde(default)]
    // stream: Option<String>,
    #[serde(default)]
    data: Option<KlineData>,
    #[serde(rename = "k", default)]
    k_inline: Option<KlineInner>,
    // Depth update fields
    #[serde(rename = "e", default)]
    event_type: Option<String>,
    // #[serde(rename = "E", default)]
    // event_time: Option<u64>,
    // #[serde(rename = "T", default)]
    // transaction_time: Option<u64>,
    // #[serde(rename = "s", default)]
    // symbol: Option<String>,
    // #[serde(rename = "U", default)]
    // first_update_id: Option<u64>,
    #[serde(rename = "u", default)]
    final_update_id: Option<u64>,
    // #[serde(rename = "pu", default)]
    // prev_final_update_id: Option<u64>,
    #[serde(rename = "b", default)]
    bids: Option<Vec<Vec<String>>>,
    #[serde(rename = "a", default)]
    asks: Option<Vec<Vec<String>>>,
}

#[derive(Deserialize)]
struct KlineData {
    #[serde(rename = "k")]
    k: KlineInner,
}

#[derive(Deserialize)]
struct KlineInner {
    // #[serde(rename = "s")]
    // symbol: String,
    // #[serde(rename = "t")]
    // open_time: u64,
    #[serde(rename = "T")]
    close_time: u64,
    #[serde(rename = "o")]
    open: String,
    #[serde(rename = "h")]
    high: String,
    #[serde(rename = "l")]
    low: String,
    #[serde(rename = "c")]
    close: String,
    #[serde(rename = "v")]
    volume: String,
    // #[serde(rename = "x")]
    // is_final: bool,
}

fn parse_order_book(text: &str) -> Result<OrderBook, serde_json::Error> {
    let env: WsEnvelope = serde_json::from_str(text)?;

    // Check if this is a depth update event
    if env.event_type.as_deref() == Some("depthUpdate") {
        let bids = env
            .bids
            .unwrap_or_default()
            .into_iter()
            .filter_map(|bid| {
                if bid.len() >= 2 {
                    Some(binance::model::Bids {
                        price: bid[0].parse().unwrap_or(0.0),
                        qty: bid[1].parse().unwrap_or(0.0),
                    })
                } else {
                    None
                }
            })
            .collect();

        let asks = env
            .asks
            .unwrap_or_default()
            .into_iter()
            .filter_map(|ask| {
                if ask.len() >= 2 {
                    Some(binance::model::Asks {
                        price: ask[0].parse().unwrap_or(0.0),
                        qty: ask[1].parse().unwrap_or(0.0),
                    })
                } else {
                    None
                }
            })
            .collect();

        return Ok(OrderBook {
            last_update_id: env.final_update_id.unwrap_or(0),
            bids,
            asks,
        });
    }

    // Fallback for other formats (shouldn't happen with depth stream)
    use serde::de::Error;
    Err(serde_json::Error::custom("Invalid depth update format"))
}

fn parse_kline(text: &str) -> Result<Candle, serde_json::Error> {
    let env: WsEnvelope = serde_json::from_str(text)?;
    let k = env.data.map(|d| d.k).or(env.k_inline).expect("kline");
    Ok(Candle {
        open: k.open.parse().unwrap_or(0.0),
        high: k.high.parse().unwrap_or(0.0),
        low: k.low.parse().unwrap_or(0.0),
        close: k.close.parse().unwrap_or(0.0),
        volume: k.volume.parse().unwrap_or(0.0),
        // Use close time in ms to align with binance semantics
        timestamp: k.close_time as i64,
        ts: DateTime::from_timestamp_millis(k.close_time as i64).unwrap(),
    })
}
