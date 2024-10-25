use crate::data_types::Parse;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Session {
    pub change: Option<f64>,
    pub change_percent: Option<f64>,
    pub close: Option<f64>,
    pub high: Option<f64>,
    pub low: Option<f64>,
    pub open: Option<f64>,
    pub previous_close: Option<f64>,
}

impl Parse for Session {
    fn parse(map: &mut serde_json::Map<String, serde_json::Value>) -> Self {
        let change = map.get("change").and_then(|v| v.as_f64());
        let change_percent = map.get("change_percent").and_then(|v| v.as_f64());
        let close = map.get("close").and_then(|v| v.as_f64());
        let high = map.get("high").and_then(|v| v.as_f64());
        let low = map.get("low").and_then(|v| v.as_f64());
        let open = map.get("open").and_then(|v| v.as_f64());
        let previous_close = map.get("previous_close").and_then(|v| v.as_f64());
        Self {
            change,
            change_percent,
            close,
            high,
            low,
            open,
            previous_close,
        }
    }
}
