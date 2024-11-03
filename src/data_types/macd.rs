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

#[test]
fn test_macd_parse() {
    let data = serde_json::json!({
        "histogram": 1.23,
        "signal": 2.34,
        "timestamp": 164545545,
        "value": 3.45
    });
    let macd = MACD::parse(&data.as_object().unwrap());
    assert_eq!(macd.histogram.unwrap(), 1.23);
    assert_eq!(macd.signal.unwrap(), 2.34);
    assert_eq!(macd.timestamp.unwrap(), 164545545);
    assert_eq!(macd.value.unwrap(), 3.45);
}
