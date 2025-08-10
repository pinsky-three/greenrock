use std::collections::HashMap;

use polars::frame::DataFrame;
use rust_decimal::Decimal;
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
pub struct StrategyState<T>
where
    T: Clone,
{
    _data_scope: DataFrame,
    _trades: HashMap<String, StrategyTrade>,
    state: T,
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
    type State: Clone;

    fn init(&self, state: &mut StrategyState<Self::State>) -> StrategyState<Self::State>;
    fn end(&self, state: &mut StrategyState<Self::State>) -> StrategyState<Self::State>;

    fn tick(
        &self,
        state: &mut StrategyState<Self::State>,
        tick: Candle,
    ) -> StrategyState<Self::State>;
}

#[derive(Clone)]
pub struct MinimalStrategy {
    pub state: StrategyState<HashMap<String, f64>>,
}

impl MinimalStrategy {
    pub fn new(data_scope: DataFrame) -> Self {
        Self {
            state: StrategyState {
                _data_scope: data_scope,
                _trades: HashMap::new(),
                state: HashMap::new(),
            },
        }
    }
}

impl Strategy for MinimalStrategy {
    type State = HashMap<String, f64>;

    fn tick(
        &self,
        state: &mut StrategyState<Self::State>,
        tick: Candle,
    ) -> StrategyState<Self::State> {
        let close = tick.close;

        state.state.insert("close".to_string(), close);

        (*state).clone()
    }

    fn init(&self, _state: &mut StrategyState<Self::State>) -> StrategyState<Self::State> {
        info!("init minimal strategy");
        _state.clone()
    }

    fn end(&self, _state: &mut StrategyState<Self::State>) -> StrategyState<Self::State> {
        info!("end minimal strategy");
        _state.clone()
    }
}
