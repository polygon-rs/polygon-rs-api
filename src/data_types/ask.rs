use crate::data_types::Parse;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Ask {
    pub price: Option<f64>,
    pub size: Option<HashMap<String, f64>>,
}

impl Parse for Ask {
    fn parse(map: &serde_json::Map<String, serde_json::Value>) -> Self {
        let price = Self::f64_parse(map, vec!["price"]);
        let size = Self::hashmap_parse(map, vec!["size"]);
        Ask { price, size }
    }
}

#[test]
fn test_ask_parse() {
    let data = serde_json::json!({
        "price": 1.23,
        "size": {
            "a": 1.0,
            "b": 2.0,
        }
    });
    let ask = Ask::parse(&data.as_object().unwrap());
    assert_eq!(ask.price.unwrap(), 1.23);
    assert_eq!(ask.size.clone().unwrap().get("a").unwrap(), &1.0);
    assert_eq!(ask.size.clone().unwrap().get("b").unwrap(), &2.0);
}
