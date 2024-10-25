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
    fn parse(map: &mut serde_json::Map<String, serde_json::Value>) -> Self {
        let change = map.get("change").and_then(|v| v.as_f64());
        let change_percent = map.get("change_percent").and_then(|v| v.as_f64());
        if let Some(close) = map.get("c") {
            map.insert(String::from("close"), close);
        };
        let close = map.get("close").and_then(|v| v.as_f64());
        if let Some(high) = map.get("h") {
            map.insert(String::from("high"), high);
        };
        let high = map.get("high").and_then(|v| v.as_f64());
        let last_updated = map.get("last_updated").and_then(|v| v.as_i64());
        if let Some(low) = map.get("l") {
            map.insert(String::from("low"), low);
        };
        let low = map.get("low").and_then(|v| v.as_f64());
        if let Some(open) = map.get("o") {
            map.insert(String::from("open"), open);
        };
        let open = map.get("open").and_then(|v| v.as_f64());
        let previous_close = map.get("previous_close").and_then(|v| v.as_f64());
        if let Some(volume) = map.get("v") {
            map.insert(String::from("volume"), volume);
        };
        let volume = map.get("volume").and_then(|v| v.as_i64());
        if let Some(volume_weighted_average_price) = map.get("vw") {
            map.insert(
                String::from("volume_weighted_average_price"),
                volume_weighted_average_price,
            );
        };
        let volume_weighted_average_price = map
            .get("volume_weighted_average_price")
            .and_then(|v| v.as_f64());
        let otc = map.get("otc").and_then(|v| v.as_bool());
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
