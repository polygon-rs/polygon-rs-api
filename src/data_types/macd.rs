use crate::data_types::Parse;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MACD {
    pub histogram: Option<f64>,
    pub signal: Option<f64>,
    pub timestamp: Option<i64>,
    pub value: Option<f64>,
}

impl Parse for MACD {
    fn parse(map: &mut serde_json::Map<String, serde_json::Value>) -> Self {
        let histogram = map.get("histogram").and_then(|v| v.as_f64());
        let signal = map.get("signal").and_then(|v| v.as_f64());
        let timestamp = map.get("timestamp").and_then(|v| v.as_i64());
        let value = map.get("value").and_then(|v| v.as_f64());
        MACD {
            histogram,
            signal,
            timestamp,
            value,
        }
    }
}
