use std::collections::HashMap;

use polars::frame::DataFrame;
// use ta::{DataItem, Next, indicators::MovingAverageConvergenceDivergence};
use tokio::signal;

use tokio_util::sync::CancellationToken;
use tracing::info;

use crate::{
    brokers::{binance::BinanceBroker, core::Broker},
    strategy::core::{Strategy, StrategyContext},
};

pub struct Runner<State>
where
    State: Clone,
{
    strategy: Box<dyn Strategy<State = State>>,
}

impl<State> Runner<State>
where
    State: Clone,
{
    pub fn new(strategy: Box<dyn Strategy<State = State>>) -> Self {
        Self { strategy }
    }

    pub async fn run_with_token(
        &self,
        mut init_state: State,
        // mut tick_rx: broadcast::Receiver<Candle>,
        cancel: CancellationToken,
    ) -> StrategyContext {
        let mut init_ctx = StrategyContext {
            _data_scope: DataFrame::new(vec![]).unwrap(),
            _trades: HashMap::new(),
        };

        let (mut ctx, mut state) = self.strategy.init(&mut init_ctx, &mut init_state);

        let binance_broker = BinanceBroker::new();

        let mut candle_rx = binance_broker.candle_stream("BTCUSDT", "1s");

        // let mut state = HashMap::<String, f64>::new();

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



                            ctx = self.strategy.tick(&mut ctx, &mut state, candle);

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

    pub async fn run_until_ctrl_c(&self, state: State) -> StrategyContext {
        let cancel = CancellationToken::new();
        let cancel_clone = cancel.clone();
        tokio::spawn(async move {
            let _ = signal::ctrl_c().await;
            cancel_clone.cancel();
        });

        self.run_with_token(state, cancel).await
    }
}
