use polars::prelude::*;

pub fn load_btc_data(file_path: &str) -> DataFrame {
    let file = std::fs::File::open(file_path).unwrap();

    ParquetReader::new(file).finish().unwrap()
}
