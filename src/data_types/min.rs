use crate::data_types::Parse;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Min {
    pub accumulated_volume: Option<i64>,
    pub open: Option<f64>,
    pub high: Option<f64>,
    pub low: Option<f64>,
    pub close: Option<f64>,
    pub volume: Option<f64>,
    pub volume_weighted_average_price: Option<f64>,
    pub transactions: Option<i64>,
    pub timestamp: Option<i64>,
}

impl Parse for Min {
    fn parse(map: &serde_json::Map<String, serde_json::Value>) -> Self {
        let accumulated_volume = Self::i64_parse(map, vec!["av"]);
        let open = Self::f64_parse(map, vec!["o"]);
        let high = Self::f64_parse(map, vec!["h"]);
        let low = Self::f64_parse(map, vec!["l"]);
        let close = Self::f64_parse(map, vec!["c"]);
        let volume = Self::f64_parse(map, vec!["v"]);
        let volume_weighted_average_price = Self::f64_parse(map, vec!["vw"]);
        let transactions = Self::i64_parse(map, vec!["n"]);
        let timestamp = Self::i64_parse(map, vec!["t"]);
        Min {
            accumulated_volume,
            open,
            high,
            low,
            close,
            volume,
            volume_weighted_average_price,
            transactions,
            timestamp,
        }
    }
}
