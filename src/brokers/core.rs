use std::collections::HashMap;

// pub struct Broker {
//     name: String,
// }

// impl Broker {

// }

pub trait Broker {
    fn balance(&self) -> HashMap<String, f64>;
}
