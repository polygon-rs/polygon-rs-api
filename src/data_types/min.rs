use crate::data_types::Parse;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Min {
    pub accumulated_volume: Option<i64>,
    pub open: Option<f64>,
    pub high: Option<f64>,
    pub low: Option<f64>,
    pub close: Option<f64>,
    pub volume: Option<f64>,
    pub volume_weighted_average_price: Option<f64>,
    pub transactions: Option<i64>,
    pub timestamp: Option<i64>,
}

impl Parse for Min {
    fn parse(map: &mut serde_json::Map<String, serde_json::Value>) -> Self {
        let accumulated_volume = map.get("av").and_then(|v| v.as_i64());
        let open = map.get("o").and_then(|v| v.as_f64());
        let high = map.get("h").and_then(|v| v.as_f64());
        let low = map.get("l").and_then(|v| v.as_f64());
        let close = map.get("c").and_then(|v| v.as_f64());
        let volume = map.get("v").and_then(|v| v.as_f64());
        let volume_weighted_average_price = map.get("vw").and_then(|v| v.as_f64());
        let transactions = map.get("n").and_then(|v| v.as_i64());
        let timestamp = map.get("t").and_then(|v| v.as_i64());
        Min {
            accumulated_volume,
            open,
            high,
            low,
            close,
            volume,
            volume_weighted_average_price,
            transactions,
            timestamp,
        }
    }
}
