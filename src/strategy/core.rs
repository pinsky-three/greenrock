use std::collections::HashMap;

use polars::frame::DataFrame;
use rust_decimal::Decimal;
// use ta::{DataItem, Next, indicators::MovingAverageConvergenceDivergence};
use tracing::info;

use crate::models::timeseries::Candle;
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

pub trait Strategy {
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
        state: &mut Self::State,
        data_scope: Vec<Candle>,
        tick: Candle,
    ) -> StrategyContext;

    fn default_state(&self) -> Self::State;
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

impl Strategy for MinimalStrategy {
    type State = HashMap<String, f64>;

    fn tick(
        &self,
        ctx: &mut StrategyContext,
        state: &mut Self::State,
        data_scope: Vec<Candle>,
        tick: Candle,
    ) -> StrategyContext {
        let close = tick.close;

        state.insert("close".to_string(), close);

        let data_scope_len = data_scope.len();

        info!("data_scope_len: {}", data_scope_len);

        let macd = state.get("macd").unwrap_or(&0.0);

        // if macd.is_none() {
        //     let mut macd = MovingAverageConvergenceDivergence::new(12, 26, 9).unwrap();
        //     macd.next(&di);
        //     state.state.insert("macd".to_string(), macd.next(&di));
        // }

        info!("macd: {:?}", macd);

        (*ctx).clone()
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
        state.insert("macd".to_string(), 0.33);

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

    fn default_state(&self) -> Self::State {
        Self::State::default()
    }
}
