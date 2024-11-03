use crate::data_types::Parse;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MovingAverage {
    pub timestamp: Option<i64>,
    pub value: Option<f64>,
}

impl Parse for MovingAverage {
    fn parse(map: &serde_json::Map<String, serde_json::Value>) -> Self {
        let timestamp = Self::i64_parse(map, vec!["timestamp"]);
        let value = Self::f64_parse(map, vec!["value"]);
        MovingAverage { timestamp, value }
    }
}

#[test]
fn test_moving_average_parse() {
    let data = serde_json::json!({
        "timestamp": 164545545,
        "value": 1.23
    });
    let moving_average = MovingAverage::parse(&data.as_object().unwrap());
    assert_eq!(moving_average.timestamp.unwrap(), 164545545);
    assert_eq!(moving_average.value.unwrap(), 1.23);
}
