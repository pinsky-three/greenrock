use std::collections::HashMap;

use crate::brokers::core::Broker;

pub struct BinanceBroker {}

impl Broker for BinanceBroker {
    fn balance(&self) -> HashMap<String, f64> {
        HashMap::new()
    }
}
