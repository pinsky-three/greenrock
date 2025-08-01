use std::collections::HashMap;

use binance::model::Kline;

use polars::frame::DataFrame;
use rust_decimal::Decimal;
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
pub struct StrategyState {
    _data_scope: DataFrame,
    _trades: HashMap<String, StrategyTrade>,
    state: HashMap<String, f64>,
}

impl Default for StrategyState {
    fn default() -> Self {
        Self {
            _data_scope: DataFrame::new(vec![]).unwrap(),
            _trades: HashMap::new(),
            state: HashMap::new(),
        }
    }
}

pub trait Strategy {
    // fn start(&self, state: &StrategyState) -> StrategyState;
    fn tick(&self, state: &mut StrategyState, tick: Option<Kline>) -> StrategyState;
}

pub struct MinimalStrategy {
    pub state: StrategyState,
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
    fn tick(&self, state: &mut StrategyState, tick: Option<Kline>) -> StrategyState {
        if let Some(tick) = tick {
            let close = tick.close.parse::<f64>().unwrap();

            state.state.insert("close".to_string(), close);
        }

        (*state).clone()
    }
}
