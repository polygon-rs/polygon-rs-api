use crate::data_types::Parse;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Default)]
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
}

impl Parse for Day {
    fn parse(map: &serde_json::Map<String, serde_json::Value>) -> Self {
        let change = map.get("change").and_then(|v| v.as_f64());
        let change_percent = map.get("change_percent").and_then(|v| v.as_f64());
        let close = map.get("close").and_then(|v| v.as_f64());
        let high = map.get("high").and_then(|v| v.as_f64());
        let last_updated = map.get("last_updated").and_then(|v| v.as_i64());
        let low = map.get("low").and_then(|v| v.as_f64());
        let open = map.get("open").and_then(|v| v.as_f64());
        let previous_close = map.get("previous_close").and_then(|v| v.as_f64());
        let volume = map.get("volume").and_then(|v| v.as_i64());
        let volume_weighted_average_price =
            map.get("volume_weighted_average_price").and_then(|v| v.as_f64());
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
        }
    }

}