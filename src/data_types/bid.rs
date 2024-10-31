use crate::data_types::Parse;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Bid {
    pub price: Option<f64>,
    pub size: Option<HashMap<String, f64>>,
}

impl Parse for Bid {
    fn parse(map: &serde_json::Map<String, serde_json::Value>) -> Self {
        let price = Self::f64_parse(map, vec!["price"]);
        let size = Self::hashmap_parse(map, vec!["size"]);
        Bid { price, size }
    }
}
