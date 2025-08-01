use std::time::Instant;

use greenrock::{
    processor::load_btc_data,
    strategy::{
        core::{MinimalStrategy, Strategy},
        utils::row_to_kline,
    },
};
use polars::prelude::{IntoLazy, col};

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
