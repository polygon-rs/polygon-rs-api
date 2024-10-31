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
    fn parse(map: &serde_json::Map<String, serde_json::Value>) -> Self {
        let histogram = Self::f64_parse(map, vec!["histogram"]);
        let signal = Self::f64_parse(map, vec!["signal"]);
        let timestamp = Self::i64_parse(map, vec!["timestamp"]);
        let value = Self::f64_parse(map, vec!["value"]);
        MACD {
            histogram,
            signal,
            timestamp,
            value,
        }
    }
}
