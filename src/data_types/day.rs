use crate::data_types::Parse;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Day {
    pub change: Option<f64>,
    pub change_percent: Option<f64>,
    pub close: Option<f64>,
    pub high: Option<f64>,
    pub last_updated: Option<i64>,
    pub low: Option<f64>,
    pub open: Option<f64>,
    pub previous_close: Option<f64>,
    pub volume: Option<i64>,
    pub volume_weighted_average_price: Option<f64>,
    pub otc: Option<bool>,
}

impl Parse for Day {
    fn parse(map: &serde_json::Map<String, serde_json::Value>) -> Self {
        let change = Self::f64_parse(map, vec!["change"]);
        let change_percent = Self::f64_parse(map, vec!["change_percent"]);
        let close = Self::f64_parse(map, vec!["c", "close"]);
        let high = Self::f64_parse(map, vec!["h", "high"]);
        let last_updated = Self::i64_parse(map, vec!["last_updated"]);
        let low = Self::f64_parse(map, vec!["l", "low"]);
        let open = Self::f64_parse(map, vec!["o", "open"]);
        let previous_close = Self::f64_parse(map, vec!["previous_close"]);
        let volume = Self::i64_parse(map, vec!["v", "volume"]);
        let volume_weighted_average_price =
            Self::f64_parse(map, vec!["vw", "volume_weighted_average_price"]);
        let otc = Self::bool_parse(map, vec!["otc"]);
        Self {
            change,
            change_percent,
            close,
            high,
            last_updated,
            low,
            open,
            previous_close,
            volume,
            volume_weighted_average_price,
            otc,
        }
    }
}
