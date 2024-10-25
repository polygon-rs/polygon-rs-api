use crate::data_types::Parse;
use crate::rest::parameters::Timeframe;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Trade {
    pub conditions: Option<Vec<i64>>,
    pub exchange_id: Option<i64>,
    pub price: Option<f64>,
    pub sip_timestamp: Option<i64>,
    pub size: Option<i64>,
    pub id: Option<String>,
    pub timeframe: Option<Timeframe>,
}

impl Parse for Trade {
    fn parse(map: &mut serde_json::Map<String, serde_json::Value>) -> Self {
        
        if let Some(conditions) = map.get("c") {
            map.insert(String::from("conditions"), conditions.clone());
        };
        let conditions = map.get("conditions").and_then(|v| v.as_array()).map(|v| {
            let mut conditions = Vec::new();
            for condition in v {
                if let Some(c) = condition.as_i64() {
                    conditions.push(c);
                }
            }
            conditions
        });
        if let Some(exchange_id) = map.get("x") {
            map.insert(String::from("exchange_id"), exchange_id.clone());
        };
        let exchange_id = map.get("exchange_id").and_then(|v| v.as_i64());
        if let Some(price) = map.get("p") {
            map.insert(String::from("price"), price.clone());
        };
        let price = map.get("price").and_then(|v| v.as_f64());
        if let Some(sip_timestamp) = map.get("t") {
            map.insert(String::from("sip_timestamp"), sip_timestamp.clone());
        };
        let sip_timestamp = map.get("sip_timestamp").and_then(|v| v.as_i64());
        if let Some(size) = map.get("s") {
            map.insert(String::from("size"), size.clone());
        };
        let size = map.get("size").and_then(|v| v.as_i64());
        let id = map.get("i").and_then(|v| v.as_str()).map(|v|{
            String::from(v)
        });
        let timeframe = map
            .get("timeframe")
            .and_then(|v| v.as_str())
            .map(|v| match v {
                "DELAYED" => Timeframe::Delayed,
                "REAL-TIME" => Timeframe::RealTime,
                _ => Timeframe::Unknown,
            });
        Trade {
            conditions,
            exchange_id,
            price,
            sip_timestamp,
            size,
            id,
            timeframe,
        }
    }
}
