use std::{collections::HashMap, time::Instant};

use chrono::{DateTime, Utc};
use polars::frame::DataFrame;
use rust_decimal::{Decimal, prelude::ToPrimitive};
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
    type State: Clone + Default;

    fn init(
        &self,
        ctx: &mut StrategyContext,
        state: &mut Self::State,
    ) -> (StrategyContext, Self::State);

    fn end(
        &self,
        ctx: &mut StrategyContext,
        state: &mut Self::State,
    ) -> (StrategyContext, Self::State);

    fn tick(
        &self,
        ctx: &mut StrategyContext,
        at: DateTime<Utc>,
        state: &mut Self::State,
        symbol: String,
        data_scope: Vec<Candle>,
        tick: Candle,
    ) -> StrategyAction;

    fn initial_state(&self) -> Self::State;

    fn portfolio(&self) -> HashMap<String, f64>;
}

#[derive(Clone)]
pub struct MinimalStrategy {
    pub context: StrategyContext,
}

impl MinimalStrategy {
    pub fn new(data_scope: DataFrame) -> Self {
        Self {
            context: StrategyContext {
                _data_scope: data_scope,
                _trades: HashMap::new(),
            },
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

impl Strategy for MinimalStrategy {
    type State = HashMap<String, f64>;

    fn tick(
        &self,
        _ctx: &mut StrategyContext,
        timestamp: DateTime<Utc>,
        state: &mut Self::State,
        symbol: String,
        data_scope: Vec<Candle>,
        tick: Candle,
    ) -> StrategyAction {
        let now = Instant::now();
        // let close = tick.close;

        // state.insert("close".to_string(), close);

        // let data_scope_len = data_scope.len();

        // info!("data_scope_len: {}", data_scope_len);

        // let macd = state.get("macd").unwrap_or(&0.0);
        let macd = data_scope.macd(12, 26, 9);
        state.insert("macd".to_string(), macd.macd);

        let ema = data_scope.ema(20);
        state.insert("ema".to_string(), ema);

        let st = data_scope.supertrend(10, 3.0);
        state.insert("st".to_string(), st.trend as f64);

        // if macd.is_none() {
        //     let mut macd = MovingAverageConvergenceDivergence::new(12, 26, 9).unwrap();
        //     macd.next(&di);
        //     state.state.insert("macd".to_string(), macd.next(&di));
        // }

        let duration = now.elapsed();

        info!(
            "[{}] macd: {:.3}, ema: {:.3}, st: {:.3}, trend: {:?} (computed in {:?})",
            tick.timestamp, macd.macd, ema, st.value, st.trend, duration,
        );

        // match st.trend {
        //     -1 => StrategyAction::Sell(symbol, tick.close),
        //     1 => StrategyAction::Buy(symbol, tick.close),
        //     _ => StrategyAction::Pass,
        // }

        let last_timestamp = state.get("last_timestamp").unwrap_or(&0_f64);

        let binding_trend = st.trend as f64;
        let last_trend = state.get("last_trend").unwrap_or(&binding_trend);

        if st.trend == 1 && timestamp.timestamp() != last_timestamp.to_i64().unwrap() {
            if last_trend == &(st.trend as f64) {
                return StrategyAction::Pass;
            }

            state.insert("last_timestamp".to_string(), timestamp.timestamp() as f64);
            state.insert("last_trend".to_string(), st.trend as f64);

            return StrategyAction::Emitted(Box::new(TradingAction {
                id: "sell".to_string(),
                timestamp,
                symbol,
                amount: 0.01,
            }));
        }

        StrategyAction::Pass
    }

    fn init(
        &self,
        ctx: &mut StrategyContext,
        state: &mut Self::State,
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

        (ctx.clone(), state.clone())
    }

    fn end(
        &self,
        ctx: &mut StrategyContext,
        state: &mut Self::State,
    ) -> (StrategyContext, Self::State) {
        info!("end minimal strategy");
        (ctx.clone(), state.clone())
    }

    fn initial_state(&self) -> Self::State {
        Self::State::default()
    }

    // hashmap of symbol and priority, priority is a number between 0 and 1
    fn portfolio(&self) -> HashMap<String, f64> {
        let mut portfolio = HashMap::new();

        portfolio.insert("BTCUSDT".to_string(), 0.5);
        portfolio.insert("ETHUSDT".to_string(), 0.3);
        portfolio.insert("SOLUSDT".to_string(), 0.2);

        portfolio
    }
}
