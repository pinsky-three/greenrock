use std::collections::HashMap;

use binance::model::{Order, OrderBook, TradeHistory};
use chrono::{DateTime, Duration, Utc};
use polars::frame::DataFrame;
use serde::Serialize;
// use ta::{DataItem, Next, indicators::MovingAverageConvergenceDivergence};
use tokio::signal;

use tokio_util::sync::CancellationToken;
use tracing::info;

use crate::{
    brokers::{binance::BinanceBroker, core::Broker},
    models::timeseries::{Candle, DataScopeRing},
    strategy::core::{StateCalculation, Strategy, StrategyAction, StrategyContext},
};

pub struct Runner<State, B, S>
where
    B: Broker + Send + Sync,
    S: Strategy<State = State> + Send + Sync,
    // State: Clone,
{
    broker: B,
    strategy: S,
}

pub struct RunConfig {
    pub symbol: String,
    pub interval: String,
    // pub data_scope_len: usize,
    // pub start_time: Option<DateTime<Utc>>,
    // pub end_time: Option<DateTime<Utc>>,
}

impl<State, B, S> Runner<State, B, S>
where
    State: Clone + Default + Serialize + StateCalculation<State>,
    B: Broker + Send + Sync,
    S: Strategy<State = State> + Send + Sync,
{
    pub fn new(broker: B, strategy: S) -> Self {
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

    pub fn portfolio(&self) -> HashMap<String, f64> {
        self.strategy.portfolio()
    }

    pub fn order_book(&self, symbol: &str, depth: u64) -> OrderBook {
        self.broker.order_book(symbol, depth)
    }

    pub fn order_book_stream(&self, symbol: &str) -> tokio::sync::broadcast::Receiver<OrderBook> {
        self.broker.order_book_stream(symbol)
    }

    pub fn market_current_price(&self, symbol: &str) -> f64 {
        self.broker.market_current_price(symbol)
    }

    pub fn indicators(&self) -> State {
        self.strategy.state()
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

    pub async fn candles_stream(
        &self,
        symbol: &str,
        interval: &str,
    ) -> tokio::sync::broadcast::Receiver<Candle> {
        self.broker.candle_stream(symbol, interval)
    }

    pub async fn run_with_cancel_signal(
        &self,
        config: &RunConfig,
        cancel: CancellationToken,
    ) -> StrategyContext {
        let mut init_ctx = StrategyContext {
            _data_scope: DataFrame::new(vec![]).unwrap(),
            _trades: HashMap::new(),
        };

        let (mut ctx, mut _state) = self.strategy.init(&mut init_ctx);

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

        let mut data_scope_ring = DataScopeRing::<State>::new(2000);

        let states = State::calculate_all(data_scope.clone());

        for (candle, state) in data_scope.iter().zip(states) {
            data_scope_ring.upsert((candle.clone(), state));
        }

        loop {
            tokio::select! {
                _ = cancel.cancelled() => {
                    break;
                }
                tick = candle_rx.recv() => {
                    match tick {
                        Ok(candle) => {
                            // let macd = data_scope.macd(12, 26, 9);
                            // let ema = data_scope.ema(20);
                            // let st = data_scope.supertrend(10, 3.0);



                            // let state = IndicatorsForMinimalStrategy {
                            //     macd: macd.macd,
                            //     ema: ema,
                            //     st: st.value,
                            //     last_timestamp: candle.timestamp,
                            //     last_trend: st.trend,
                            // };

                            // Self::State::default();

                            // let state = State::calculate_one(candle.clone());
                            let state = State::next((candle.clone(), _state.clone()));
                            // State::calculate_all()

                            // data_scope.push(candle.clone());
                            data_scope_ring.upsert((candle.clone(), state));

                            let response = self
                                .strategy
                                .tick(
                                    &mut ctx,
                                    DateTime::from_timestamp(candle.timestamp, 0).unwrap(),
                                    config.symbol.to_string(),
                                    data_scope_ring.snapshot(),
                                    (candle, _state.clone()),
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

        let (ctx, _state) = self.strategy.end(&mut ctx);

        ctx
    }

    pub async fn run_until_ctrl_c(&self, config: &RunConfig) -> StrategyContext {
        let cancel = CancellationToken::new();
        let cancel_clone = cancel.clone();
        tokio::spawn(async move {
            let _ = signal::ctrl_c().await;
            cancel_clone.cancel();
        });

        self.run_with_cancel_signal(config, cancel).await
    }
}
