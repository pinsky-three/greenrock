use std::{any::Any, collections::HashMap};

use binance::model::Kline;
use greenrock::processor::load_btc_data;
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
}

impl Default for StrategyState {
    fn default() -> Self {
        Self {
            data_scope: DataFrame::new(vec![]).unwrap(),
            traits: HashMap::new(),
        }
    }
}

trait Starting {
    // fn start(&self, state: &StrategyState) -> StrategyState;
    fn tick(&self, state: &StrategyState, tick: Option<Kline>) -> StrategyState;
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
            },
        }
    }
}

impl Starting for MinimalStrategy {
    fn tick(&self, state: &StrategyState, tick: Option<Kline>) -> StrategyState {
        if let Some(tick) = tick {
            let close = tick.close.parse::<f64>().unwrap();

            // println!("{}", close);
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

    for i in 0..df.shape().0 {
        let row = df.get_row(i).unwrap();

        let timestamp = match row.0[0] {
            AnyValue::Datetime(value, _, _) => value,
            _ => panic!("Invalid timestamp"),
        };

        let open = match row.0[1] {
            AnyValue::String(value) => value.parse::<f64>().unwrap(),
            _ => panic!("Invalid open"),
        };

        let high = match row.0[2] {
            AnyValue::Float64(value) => value,
            _ => panic!("Invalid high"),
        };

        let low = match row.0[3] {
            AnyValue::Float64(value) => value,
            _ => panic!("Invalid low"),
        };

        let close = match row.0[4] {
            AnyValue::Float64(value) => value,
            _ => panic!("Invalid close"),
        };

        let volume = match row.0[5] {
            AnyValue::String(value) => value.parse::<f64>().unwrap(),
            _ => panic!("Invalid volume"),
        };

        let tick = Kline {
            symbol: "BTCUSDT".to_string(),
            interval: "1m".to_string(),
            ignore_me: "".to_string(),
            open_time: timestamp,
            open: open.to_string(),
            high: high.to_string(),
            low: low.to_string(),
            close: close.to_string(),
            volume: volume.to_string(),
            close_time: timestamp,
            first_trade_id: 0,
            last_trade_id: 0,
            number_of_trades: 0,
            is_final_bar: false,
            quote_asset_volume: "".to_string(),
            taker_buy_base_asset_volume: "".to_string(),
            taker_buy_quote_asset_volume: "".to_string(),
        };

        strategy.state = strategy.tick(&strategy.state, Some(tick));
    }

    // println!("{}", df.shape().0);

    println!("{}", strategy.state.data_scope);
}
