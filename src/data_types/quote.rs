use crate::rest::parameters::Timeframe;
use crate::data_types::Parse;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Quote {
    pub bid: Option<f64>,
    pub bid_size: Option<i64>,
    pub ask: Option<f64>,
    pub ask_size: Option<i64>,
    pub bid_exchange_id: Option<i64>,
    pub ask_exchange_id: Option<i64>,
    pub last_updated: Option<i64>,
    pub mid_point: Option<f64>,
    pub timeframe: Option<Timeframe>,
    pub exchange_id: Option<i64>,
}

impl Parse for Quote {
    fn parse(map: &mut serde_json::Map<String, serde_json::Value>) -> Self {
        if let Some(bid) = map.get("p") {
            map.insert(String::from("bid"), bid);
        };
        if let Some(bid) = map.get("b") {
            map.insert(String::from("bid"), bid);
        };
        let bid = map.get("bid").and_then(|v| v.as_f64());
        if let Some(bid_size) = map.get("s") {
            map.insert(String::from("bid_size"), bid_size);
        };
        let bid_size = map.get("bid_size").and_then(|v| v.as_i64());
        if let Some(ask) = map.get("P") {
            map.insert(String::from("ask"), ask);
        };
        if let Some(ask) = map.get("a") {
            map.insert(String::from("ask"), ask);
        };
        let ask = map.get("ask").and_then(|v| v.as_f64());
        if let Some(ask_size) = map.get("S") {
            map.insert(String::from("ask_size"), ask_size);
        };
        let ask_size = map.get("ask_size").and_then(|v| v.as_i64());
        let bid_exchange_id = map.get("bid_exchange_id").and_then(|v| v.as_i64());
        let ask_exchange_id = map.get("ask_exchange_id").and_then(|v| v.as_i64());
        if let Some(last_updated) = map.get("t") {
            map.insert(String::from("last_updated"), last_updated);
        };
        let last_updated = map.get("last_updated").and_then(|v| v.as_i64());
        let mid_point = map.get("mid_point").and_then(|v| v.as_f64());
        let timeframe = map.get("timeframe").and_then(|v| v.as_str()).map(|v| match v {
            "DELAYED" => Timeframe::Delayed,
            "REAL-TIME" => Timeframe::RealTime,
            _ => Timeframe::Unknown,
        });
        let exchange_id = map.get("x").and_then(|v| v.as_i64());
        Quote {
            bid,
            bid_size,
            ask,
            ask_size,
            bid_exchange_id,
            ask_exchange_id,
            last_updated,
            mid_point,
            timeframe,
            exchange_id,
        }
    }
}