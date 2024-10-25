use crate::data_types::Parse;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Ask {
    pub price: Option<f64>,
    pub size: Option<HashMap<String, f64>>,
}

impl Parse for Ask {
    fn parse(map: &mut serde_json::Map<String, serde_json::Value>) -> Self {
        let price = map.get("price").and_then(|v| v.as_f64());
        let size = map.get("size").and_then(|v| v.as_f64());
        Ask { price, size }
    }
}
