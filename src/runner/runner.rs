use ta::{DataItem, Next, indicators::MovingAverageConvergenceDivergence};
use tokio::signal;

use tokio_util::sync::CancellationToken;
use tracing::info;

use crate::{
    brokers::{binance::BinanceBroker, core::Broker},
    strategy::core::{Strategy, StrategyState},
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
        mut init_state: StrategyState<State>,
        // mut tick_rx: broadcast::Receiver<Candle>,
        cancel: CancellationToken,
    ) -> StrategyState<State> {
        let mut state = self.strategy.init(&mut init_state);

        let binance_broker = BinanceBroker::new();

        let mut candle_rx = binance_broker.candle_stream("BTCUSDT", "1s");

        let mut macd = MovingAverageConvergenceDivergence::new(12, 26, 9).unwrap();

        loop {
            tokio::select! {
                _ = cancel.cancelled() => {
                    break;
                }
                tick = candle_rx.recv() => {
                    match tick {
                        Ok(candle) => {
                            let di = DataItem::builder()
                                .high(candle.high)
                                .low(candle.low)
                                .close(candle.close)
                                .open(candle.open)
                                .volume(candle.volume)
                                // .timestamp(candle.timestamp)
                                .build()
                                .unwrap();

                            let macd_res = macd.next(&di);

                            info!(
                                "candle close={:.2} high={:.2} low={:.2} volume={:.3} macd={:.3}",
                                candle.close,
                                candle.high,
                                candle.low,
                                // candle.open,
                                candle.volume,
                                // candle.timestamp,
                                macd_res.macd,
                            );

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

        self.strategy.end(&mut state)
    }

    pub async fn run_until_ctrl_c(&self, state: StrategyState<State>) -> StrategyState<State> {
        let cancel = CancellationToken::new();
        let cancel_clone = cancel.clone();
        tokio::spawn(async move {
            let _ = signal::ctrl_c().await;
            cancel_clone.cancel();
        });

        self.run_with_token(state, cancel).await
    }
}
