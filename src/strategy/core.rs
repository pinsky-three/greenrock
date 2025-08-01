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
    id: String,
    kind: StrategyTraitKind,
    value: Decimal,
    start_value: Decimal,
    end_value: Option<Decimal>,
    // start: DateTime<Utc>,
    // end: DateTime<Utc>,
}

#[derive(Clone)]
pub struct StrategyState {
    data_scope: DataFrame,
    trades: HashMap<String, StrategyTrade>,
    state: HashMap<String, f64>,
}

impl Default for StrategyState {
    fn default() -> Self {
        Self {
            data_scope: DataFrame::new(vec![]).unwrap(),
            trades: HashMap::new(),
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
                data_scope,
                trades: HashMap::new(),
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
