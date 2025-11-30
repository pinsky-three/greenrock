use std::{collections::HashMap, time::Instant};

use chrono::{DateTime, Utc};
use polars::frame::DataFrame;
use rust_decimal::{Decimal, prelude::ToPrimitive};
use serde::Serialize;
use ta::{Next, indicators::ExponentialMovingAverage};
// use ta::{DataItem, Next, indicators::MovingAverageConvergenceDivergence};
use tracing::info;

use crate::models::{analysis::TechnicalAnalysis, timeseries::Candle};
// use rust_decimal::prelude::*;

#[derive(Clone)]
pub enum StrategyTraitKind {
    Short,
    Long,
}

#[derive(Clone)]
pub struct StrategyTrade {
    _id: String,
    _kind: StrategyTraitKind,
    _value: Decimal,
    _start_value: Decimal,
    _end_value: Option<Decimal>,
    // start: DateTime<Utc>,
    // end: DateTime<Utc>,
}

#[derive(Clone)]
pub struct StrategyContext {
    pub _data_scope: DataFrame,
    pub _trades: HashMap<String, StrategyTrade>,
    // state: T,
}

// impl<T> Default for StrategyState<T>
// where
//     T: Clone,
// {
//     fn default() -> Self {
//         Self {
//             _data_scope: DataFrame::new(vec![]).unwrap(),
//             _trades: HashMap::new(),
//             state: T::default(),
//         }
//     }
// }

pub trait Strategy: Send + Sync {
    type State: Clone + Default + Serialize + StateCalculation<Self::State>;

    fn init(
        &self,
        ctx: &mut StrategyContext,
        // state: &mut Self::State,
    ) -> (StrategyContext, Self::State);

    fn end(
        &self,
        ctx: &mut StrategyContext,
        // state: &mut Self::State,
    ) -> (StrategyContext, Self::State);

    fn tick(
        &self,
        ctx: &mut StrategyContext,
        at: DateTime<Utc>,
        // state: &mut Self::State,
        symbol: String,
        data_scope: Vec<(Candle, Self::State)>,
        tick: (Candle, Self::State),
    ) -> StrategyAction;

    // fn initial_state(&self) -> Self::State;

    fn portfolio(&self) -> HashMap<String, f64>;

    fn state(&self) -> Self::State;
}

#[derive(Clone)]
pub struct MinimalStrategy {
    pub context: StrategyContext,
    pub state: IndicatorsForMinimalStrategy,
}

impl MinimalStrategy {
    pub fn new(data_scope: DataFrame) -> Self {
        Self {
            context: StrategyContext {
                _data_scope: data_scope,
                _trades: HashMap::new(),
            },
            state: IndicatorsForMinimalStrategy::default(),
        }
    }
}

// pub enum StrategyAction {
//     Sell(String, f64),
//     Buy(String, f64),
//     Nothing,
// }

#[derive(Clone, Debug)]
pub struct TradingAction {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub symbol: String,
    pub amount: f64,
    // pub action: StrategyAction,
}

#[derive(Clone, Debug)]
pub enum StrategyAction {
    Emitted(Box<TradingAction>),
    // Stop(TradingAction),
    Pass,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct IndicatorsForMinimalStrategy {
    // pub macd: f64,
    pub ema20: f64,
    // pub st: f64,

    // last_timestamp: i64,
    // last_trend: f64,
    #[serde(skip)]
    ema20_calculator: ExponentialMovingAverage,
}
pub trait StateCalculation<State>
where
    State: Clone + Default + Serialize,
{
    // fn calculate_one(candle: Candle) -> State;
    fn calculate_all(data_scope: Vec<Candle>) -> Vec<State>;
    fn next(input: (Candle, State)) -> State;
}

impl StateCalculation<IndicatorsForMinimalStrategy> for IndicatorsForMinimalStrategy {
    // fn calculate_one(candle: Candle) -> IndicatorsForMinimalStrategy {
    //     IndicatorsForMinimalStrategy {
    //         macd: 0.0,
    //         ema: 0.0,
    //         st: 0.0,
    //         last_timestamp: candle.timestamp,
    //         last_trend: 0.0,
    //     }
    // }

    fn next(
        (candle, mut _state): (Candle, IndicatorsForMinimalStrategy),
    ) -> IndicatorsForMinimalStrategy {
        // let mut ema20 = ExponentialMovingAverage::new(20).unwrap();
        let ema20 = _state.ema20_calculator.next(candle.close);
        IndicatorsForMinimalStrategy {
            ema20,
            ema20_calculator: _state.ema20_calculator,
        }
    }

    fn calculate_all(candles: Vec<Candle>) -> Vec<IndicatorsForMinimalStrategy> {
        // let macd = candles.macd(12, 26, 9);
        // let ema = candles.ema(20);
        // let st = candles.supertrend(10, 3.0);

        let mut ema20 = ExponentialMovingAverage::new(20).unwrap();

        let mut states = Vec::new();

        for candle in candles {
            states.push(IndicatorsForMinimalStrategy {
                ema20: ema20.next(candle.close),
                ema20_calculator: ema20.clone(),
            });
        }
        states
    }
}

impl Strategy for MinimalStrategy {
    type State = IndicatorsForMinimalStrategy; // HashMap<String, f64>;

    fn tick(
        &self,
        _ctx: &mut StrategyContext,
        timestamp: DateTime<Utc>,
        symbol: String,
        data_scope: Vec<(Candle, Self::State)>,
        tick: (Candle, Self::State),
    ) -> StrategyAction {
        let now = Instant::now();
        // let close = tick.close;

        // state.insert("close".to_string(), close);

        // let data_scope_len = data_scope.len();

        // info!("data_scope_len: {}", data_scope_len);

        // let macd = state.get("macd").unwrap_or(&0.0);
        // state.insert("macd".to_string(), macd.macd);

        // state.insert("ema".to_string(), ema);

        // state.insert("st".to_string(), st.trend as f64);

        // if macd.is_none() {
        //     let mut macd = MovingAverageConvergenceDivergence::new(12, 26, 9).unwrap();
        //     macd.next(&di);
        //     state.state.insert("macd".to_string(), macd.next(&di));
        // }

        // self.state.macd = macd.macd;
        // self.state.ema = ema;
        // self.state.st = st.value;

        let duration = now.elapsed();

        info!(
            "[{}] ema20: {} (computed in {:?})",
            tick.0.timestamp, self.state.ema20, duration,
        );

        // match st.trend {
        //     -1 => StrategyAction::Sell(symbol, tick.close),
        //     1 => StrategyAction::Buy(symbol, tick.close),
        //     _ => StrategyAction::Pass,
        // }

        // let last_timestamp = self.state.last_timestamp;

        // let last_trend = self.state.last_trend;

        // if st.trend == 1.0 && timestamp.timestamp() != last_timestamp.to_i64().unwrap() {
        //     if last_trend == st.trend {
        //         return StrategyAction::Pass;
        //     }

        //     // self.state.last_timestamp = timestamp.timestamp();
        //     // self.state.last_trend = st.trend;

        //     return StrategyAction::Emitted(Box::new(TradingAction {
        //         id: "sell".to_string(),
        //         timestamp,
        //         symbol,
        //         amount: 0.01,
        //     }));
        // }

        StrategyAction::Pass
    }

    fn init(
        &self,
        ctx: &mut StrategyContext,
        // state: &mut Self::State,
    ) -> (StrategyContext, Self::State) {
        info!("init minimal strategy");
        // let mut macd = MovingAverageConvergenceDivergence::new(12, 26, 9).unwrap();

        // let di = DataItem::builder()
        //     .high(candle.high)
        //     .low(candle.low)
        //     .close(candle.close)
        //     .open(candle.open)
        //     .volume(candle.volume)
        //     // .timestamp(candle.timestamp)
        //     .build()
        //     .unwrap();

        // macd.next(&di);
        // state.state.insert("macd".to_string(), macd.next(&di));

        // state.insert("macd".to_string(), 0.33);

        (ctx.clone(), self.state.clone())
    }

    fn end(
        &self,
        ctx: &mut StrategyContext,
        // state: &mut Self::State,
    ) -> (StrategyContext, Self::State) {
        info!("end minimal strategy");
        (ctx.clone(), self.state.clone())
    }

    // fn initial_state(&mut self) -> Self::State {
    //     self.state.clone()
    // }

    // hashmap of symbol and priority, priority is a number between 0 and 1
    fn portfolio(&self) -> HashMap<String, f64> {
        let mut portfolio = HashMap::new();

        portfolio.insert("BTCUSDT".to_string(), 0.5);
        portfolio.insert("ETHUSDT".to_string(), 0.3);
        portfolio.insert("SOLUSDT".to_string(), 0.2);

        portfolio
    }

    fn state(&self) -> Self::State {
        self.state.clone()
    }
}
