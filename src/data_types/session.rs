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
    fn parse(map: &serde_json::Map<String, serde_json::Value>) -> Self {
        let change = Self::f64_parse(map, vec!["change"]);
        let change_percent = Self::f64_parse(map, vec!["change_percent"]);
        let close = Self::f64_parse(map, vec!["close"]);
        let high = Self::f64_parse(map, vec!["high"]);
        let low = Self::f64_parse(map, vec!["low"]);
        let open = Self::f64_parse(map, vec!["open"]);
        let previous_close = Self::f64_parse(map, vec!["previous_close"]);
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
