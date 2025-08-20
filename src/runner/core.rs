use std::collections::HashMap;

use binance::model::{Order, TradeHistory};
use chrono::{DateTime, Duration, Utc};
use polars::frame::DataFrame;
// use ta::{DataItem, Next, indicators::MovingAverageConvergenceDivergence};
use tokio::signal;

use tokio_util::sync::CancellationToken;
use tracing::info;

use crate::{
    brokers::{binance::BinanceBroker, core::Broker},
    models::timeseries::{Candle, CandleRing},
    strategy::core::{Strategy, StrategyAction, StrategyContext},
};

pub struct Runner<State, B>
where
    B: Broker + Send + Sync,
    State: Clone,
{
    broker: B,
    strategy: Box<dyn Strategy<State = State>>,
}

pub struct RunConfig {
    pub symbol: String,
    pub interval: String,
    // pub data_scope_len: usize,
    // pub start_time: Option<DateTime<Utc>>,
    // pub end_time: Option<DateTime<Utc>>,
}

impl<State, B> Runner<State, B>
where
    State: Clone + Default,
    B: Broker + Send + Sync,
{
    pub fn new(broker: B, strategy: Box<dyn Strategy<State = State>>) -> Self {
        Self { broker, strategy }
    }

    pub fn open_orders(&self, symbol: &str) -> Vec<Order> {
        self.broker.open_orders(symbol)
    }

    pub fn trade_history(&self, symbol: &str) -> Vec<TradeHistory> {
        self.broker.trade_history(symbol)
    }

    pub fn balance(&self) -> HashMap<String, f64> {
        self.broker.balance()
    }

    pub fn market_current_price(&self, symbol: &str) -> f64 {
        self.broker.market_current_price(symbol)
    }

    pub async fn candles(
        &self,
        symbol: &str,
        interval: &str,
        limit: u16,
        start_time: Option<DateTime<Utc>>,
        end_time: Option<DateTime<Utc>>,
    ) -> Vec<Candle> {
        self.broker
            .candles(symbol, interval, limit, start_time, end_time)
            .await
    }

    pub async fn run_with_cancel_signal(
        &self,
        mut init_state: State,
        config: &RunConfig,
        cancel: CancellationToken,
    ) -> StrategyContext {
        let mut init_ctx = StrategyContext {
            _data_scope: DataFrame::new(vec![]).unwrap(),
            _trades: HashMap::new(),
        };

        let (mut ctx, mut state) = self.strategy.init(&mut init_ctx, &mut init_state);

        let binance_broker = BinanceBroker::new();

        let mut candle_rx = binance_broker.candle_stream(&config.symbol, &config.interval);

        // let mut data_scope = Vec::new();

        let data_scope = binance_broker
            .candles(
                &config.symbol,
                &config.interval,
                1000,
                Some(Utc::now() - Duration::days(1)),
                Some(Utc::now()),
            )
            .await;

        let mut data_scope_ring = CandleRing::new(2000);

        for candle in data_scope {
            data_scope_ring.upsert(candle);
        }

        loop {
            tokio::select! {
                _ = cancel.cancelled() => {
                    break;
                }
                tick = candle_rx.recv() => {
                    match tick {
                        Ok(candle) => {
                            // let di = DataItem::builder()
                            //     .high(candle.high)
                            //     .low(candle.low)
                            //     .close(candle.close)
                            //     .open(candle.open)
                            //     .volume(candle.volume)
                            //     // .timestamp(candle.timestamp)
                            //     .build()
                            //     .unwrap();

                            // let macd_res = macd.next(&di);

                            // info!(
                            //     "candle close={:.2} high={:.2} low={:.2} volume={:.3} macd={:.3}",
                            //     candle.close,
                            //     candle.high,
                            //     candle.low,
                            //     // candle.open,
                            //     candle.volume,
                            //     // candle.timestamp,
                            //     macd_res.macd,
                            // );

                            // let mut ctx = StrategyContext {
                            //     _data_scope: DataFrame::new(vec![]).unwrap(),
                            //     _trades: HashMap::new(),
                            // };

                            // data_scope.push(candle.clone());
                            data_scope_ring.upsert(candle.clone());

                            let response = self
                                .strategy
                                .tick(
                                    &mut ctx,
                                    DateTime::from_timestamp(candle.timestamp, 0).unwrap(),
                                    &mut state,
                                    config.symbol.to_string(),
                                    data_scope_ring.snapshot(),
                                    candle,
                                );

                            match response {
                                StrategyAction::Emitted(action) => {
                                    info!("Emitted action: {:?}", action);
                                }
                                StrategyAction::Pass => {
                                    info!("Pass");
                                }
                            }

                            // println!("atr: {atr:?}");
                        }
                        Err(tokio::sync::broadcast::error::RecvError::Lagged(n)) => {
                            info!("candle stream lagged by {} messages", n);
                        }
                        Err(tokio::sync::broadcast::error::RecvError::Closed) => {
                            info!("candle stream closed");
                            break;
                        }
                    }
                }
            }
        }

        let (ctx, _state) = self.strategy.end(&mut ctx, &mut state);

        ctx
    }

    pub async fn run_until_ctrl_c(&self, config: &RunConfig, state: State) -> StrategyContext {
        let cancel = CancellationToken::new();
        let cancel_clone = cancel.clone();
        tokio::spawn(async move {
            let _ = signal::ctrl_c().await;
            cancel_clone.cancel();
        });

        self.run_with_cancel_signal(state, config, cancel).await
    }
}
