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
