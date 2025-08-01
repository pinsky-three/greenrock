use std::{any::Any, collections::HashMap, time::Instant};

use binance::model::Kline;
use chrono::{Duration, Utc};
use greenrock::{processor::load_btc_data, utils::row_to_kline};
use polars::{
    frame::DataFrame,
    prelude::{AnyValue, DataType, IntoLazy, col},
};
use rust_decimal::Decimal;
// use rust_decimal::prelude::*;

#[derive(Clone)]
enum StrategyTraitKind {
    Short,
    Long,
}

#[derive(Clone)]
struct StrategyTrait {
    id: String,
    kind: StrategyTraitKind,
    value: Decimal,
    start_value: Decimal,
    end_value: Option<Decimal>,
    // start: DateTime<Utc>,
    // end: DateTime<Utc>,
}

#[derive(Clone)]
struct StrategyState {
    data_scope: DataFrame,
    traits: HashMap<String, StrategyTrait>,
    state: HashMap<String, f64>,
}

impl Default for StrategyState {
    fn default() -> Self {
        Self {
            data_scope: DataFrame::new(vec![]).unwrap(),
            traits: HashMap::new(),
            state: HashMap::new(),
        }
    }
}

trait Starting {
    // fn start(&self, state: &StrategyState) -> StrategyState;
    fn tick(&self, state: &mut StrategyState, tick: Option<Kline>) -> StrategyState;
}

struct MinimalStrategy {
    state: StrategyState,
}

impl MinimalStrategy {
    fn new(data_scope: DataFrame) -> Self {
        Self {
            state: StrategyState {
                data_scope,
                traits: HashMap::new(),
                state: HashMap::new(),
            },
        }
    }
}

impl Starting for MinimalStrategy {
    fn tick(&self, state: &mut StrategyState, tick: Option<Kline>) -> StrategyState {
        if let Some(tick) = tick {
            let close = tick.close.parse::<f64>().unwrap();

            state.state.insert("close".to_string(), close);
        }

        (*state).clone()
    }
}

fn main() {
    let df = load_btc_data("processed_btc_data/2021-01.parquet");

    let mut strategy = MinimalStrategy::new(df.clone());

    let df = df
        .clone()
        .lazy()
        .select([
            col("timestamp"),
            col("open"),
            col("high"),
            col("low"),
            col("close"),
            col("volume"),
        ])
        .collect()
        .unwrap();

    let start = Instant::now();

    for i in 0..df.shape().0 {
        let tick = row_to_kline(&df, i);
        strategy.state = strategy.tick(&mut strategy.state.clone(), Some(tick));
    }

    println!(
        "evaluated {} klines in {:.3}s",
        df.shape().0,
        start.elapsed().as_secs_f64()
    );

    // println!("{}", strategy.state.data_scope);
}
