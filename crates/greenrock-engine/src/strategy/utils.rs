use binance::model::Kline;
use polars::{frame::DataFrame, prelude::AnyValue};

pub fn row_to_kline(df: &DataFrame, i: usize) -> Kline {
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

    Kline {
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
    }
}
