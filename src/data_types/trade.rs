use crate::rest::parameters::Timeframe;
use crate::data_types::Parse;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Default)]
pub struct Trade {
    pub conditions: Option<Vec<i64>>,
    pub exchange_id: Option<i64>,
    pub price: Option<f64>,
    pub sip_timestamp: Option<i64>,
    pub size: Option<i64>,
    pub timeframe: Option<Timeframe>,
}

impl Parse for Trade {
    fn parse(map: &serde_json::Map<String, serde_json::Value>) -> Self {
        let conditions = map.get("conditions").and_then(|v| v.as_array()).map(|v| {
            let mut conditions = Vec::new();
            for condition in v {
                if let Some(c) = condition.as_i64() {
                    conditions.push(c);
                }
            }
            conditions
        });
        let exchange_id = map.get("exchange_id").and_then(|v| v.as_i64());
        let price = map.get("price").and_then(|v| v.as_f64());
        let sip_timestamp = map.get("sip_timestamp").and_then(|v| v.as_i64());
        let size = map.get("size").and_then(|v| v.as_i64());
        let timeframe = map.get("timeframe").and_then(|v| v.as_str()).map(|v| match v {
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
            timeframe,
        }
    }
}