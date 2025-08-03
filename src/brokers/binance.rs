use std::{collections::HashMap, env};

use binance::{account::Account, api::Binance, market::Market};

use crate::brokers::core::Broker;

pub struct BinanceBroker {
    market: Market,
    account: Account,
}

impl Broker for BinanceBroker {
    fn balance(&self) -> HashMap<String, f64> {
        match self.account.get_account() {
            Ok(answer) => {
                let res = answer
                    .balances
                    .iter()
                    .filter(|instrument| instrument.locked.parse::<f64>().unwrap() > 0.0)
                    .collect::<Vec<_>>();

                res.iter()
                    .map(|instrument| {
                        (
                            instrument.asset.clone(),
                            instrument.free.parse::<f64>().unwrap(),
                        )
                    })
                    .collect()
            }
            Err(e) => {
                println!("Error: {e}");
                HashMap::new()
            }
        }
    }

    fn market_current_price(&self, symbol: &str) -> f64 {
        self.market.get_price(symbol).unwrap().price
    }
}

impl BinanceBroker {
    pub fn new() -> Self {
        let market = Binance::new(None, None);

        let api_key = Some(env::var("BINANCE_API_KEY").unwrap());
        let secret_key = Some(env::var("BINANCE_SECRET_KEY").unwrap());

        let account: Account = Binance::new(api_key, secret_key);

        Self { market, account }
    }
}

impl Default for BinanceBroker {
    fn default() -> Self {
        Self::new()
    }
}
