use crate::data_types::Parse;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RelativeStrength {
    pub timestamp: Option<i64>,
    pub value: Option<f64>,
}

impl Parse for RelativeStrength {
    fn parse(map: &serde_json::Map<String, serde_json::Value>) -> Self {
        let timestamp = Self::i64_parse(map, vec!["timestamp"]);
        let value = Self::f64_parse(map, vec!["value"]);
        RelativeStrength { timestamp, value }
    }
}

#[test]
fn test_relative_strength_parse() {
    let data = serde_json::json!({
        "timestamp": 164545545,
        "value": 1.23
    });
    let relative_strength = RelativeStrength::parse(&data.as_object().unwrap());
    assert_eq!(relative_strength.timestamp.unwrap(), 164545545);
    assert_eq!(relative_strength.value.unwrap(), 1.23);
}
