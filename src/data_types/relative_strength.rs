use crate::data_types::Parse;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RelativeStrength {
    pub timestamp: Option<i64>,
    pub value: Option<f64>,
}

impl Parse for RelativeStrength {
    fn parse(map: &mut serde_json::Map<String, serde_json::Value>) -> Self {
        let timestamp = map.get("timestamp").and_then(|v| v.as_i64());
        let value = map.get("value").and_then(|v| v.as_f64());
        RelativeStrength { timestamp, value }
    }
}
