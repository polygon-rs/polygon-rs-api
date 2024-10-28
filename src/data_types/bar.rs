use crate::data_types::Parse;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Bar {
    pub excahnge: Option<String>,
    pub close: Option<f64>,
    pub high: Option<f64>,
    pub low: Option<f64>,
    pub transactions: Option<i64>,
    pub open: Option<f64>,
    pub timestamp: Option<i64>,
    pub volume: Option<f64>,
    pub volume_weighted: Option<f64>,
    pub otc: Option<bool>,
}

impl Parse for Bar {
    fn parse(map: &mut serde_json::Map<String, serde_json::Value>) -> Self {
        let excahnge = map.get("T").and_then(|v| v.as_str().map(|s| s.to_string()));
        let close = map.get("c").and_then(|v| v.as_f64());
        let high = map.get("h").and_then(|v| v.as_f64());
        let low = map.get("l").and_then(|v| v.as_f64());
        let transactions = map.get("n").and_then(|v| v.as_i64());
        let open = map.get("o").and_then(|v| v.as_f64());
        let timestamp = map.get("t").and_then(|v| v.as_i64());
        let volume = map.get("v").and_then(|v| v.as_f64());
        let volume_weighted = map.get("vw").and_then(|v| v.as_f64());
        let otc = map.get("otc").and_then(|v| v.as_bool());
        Bar {
            excahnge,
            close,
            high,
            low,
            transactions,
            open,
            timestamp,
            volume,
            volume_weighted,
            otc,
        }
    }
}
