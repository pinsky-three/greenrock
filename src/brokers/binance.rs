use std::collections::HashMap;
use std::env;

use binance::{account::Account, api::Binance, market::Market};

use crate::brokers::core::Broker;
use crate::models::timeseries::Candle;

pub struct BinanceBroker {
    market: Market,
    account: Account,
}

use futures_util::{SinkExt, StreamExt};
use serde::Deserialize;
use tokio::sync::broadcast;
use tokio::time::{Duration, sleep};
use tokio_tungstenite::tungstenite::Message;

impl Broker for BinanceBroker {
    fn balance(&self) -> HashMap<String, f64> {
        match self.account.get_account() {
            Ok(answer) => {
                let res = answer
                    .balances
                    .iter()
                    .filter(|instrument| instrument.locked.parse::<f64>().unwrap() > 0.0)
                    .collect::<Vec<_>>();

                res.iter()
                    .map(|instrument| {
                        (
                            instrument.asset.clone(),
                            instrument.free.parse::<f64>().unwrap(),
                        )
                    })
                    .collect()
            }
            Err(e) => {
                println!("Error: {e}");
                HashMap::new()
            }
        }
    }

    fn market_current_price(&self, symbol: &str) -> f64 {
        self.market.get_price(symbol).unwrap().price
    }

    fn candle_stream(
        &self,
        symbol: &str,
    ) -> broadcast::Receiver<crate::models::timeseries::Candle> {
        let (tx, rx) = broadcast::channel::<Candle>(1024);
        let symbol = symbol.to_lowercase();

        tokio::spawn(async move {
            let interval = "1s"; // default interval; adjust if needed
            let mut backoff = Duration::from_secs(1);
            let max_backoff = Duration::from_secs(60);

            loop {
                let url = format!(
                    "wss://stream.binance.com:9443/ws/{}@kline_{}",
                    symbol, interval
                );

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
}

impl BinanceBroker {
    pub fn new() -> Self {
        let market = Binance::new(None, None);

        let api_key = Some(env::var("BINANCE_API_KEY").unwrap());
        let secret_key = Some(env::var("BINANCE_SECRET_KEY").unwrap());

        let account: Account = Binance::new(api_key, secret_key);

        // tokio::spawn(async move {
        //     if let Ok(answer) = user_stream.start() {
        //         println!("Data Stream Started ...");
        //         let listen_key = answer.listen_key;

        //         match user_stream.que tkeep_alive(&listen_key) {
        //             Ok(msg) => println!("Keepalive user data stream: {msg:?}"),
        //             Err(e) => println!("Error: {e:?}"),
        //         }

        //         match user_stream.close(&listen_key) {
        //             Ok(msg) => println!("Close user data stream: {msg:?}"),
        //             Err(e) => println!("Error: {e:?}"),
        //         }
        //     } else {
        //         println!("Not able to start an User Stream (Check your API_KEY)");
        //     }
        // });

        Self { market, account }
    }
}

impl Default for BinanceBroker {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Deserialize)]
struct WsEnvelope {
    #[serde(default)]
    stream: Option<String>,
    #[serde(default)]
    data: Option<KlineData>,
    #[serde(rename = "k", default)]
    k_inline: Option<KlineInner>,
}

#[derive(Deserialize)]
struct KlineData {
    #[serde(rename = "k")]
    k: KlineInner,
}

#[derive(Deserialize)]
struct KlineInner {
    #[serde(rename = "s")]
    symbol: String,
    #[serde(rename = "t")]
    open_time: u64,
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
    #[serde(rename = "x")]
    is_final: bool,
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
    })
}
