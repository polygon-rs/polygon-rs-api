use crate::data_types::Parse;
use crate::rest::parameters::Timeframe;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Default)]
pub struct UnderlyingAsset {
    pub change_to_break_even: Option<f64>,
    pub last_updated: Option<i64>,
    pub price: Option<f64>,
    pub ticker: Option<String>,
    pub timeframe: Option<Timeframe>,
    pub value: Option<f64>,
}

impl Parse for UnderlyingAsset {
    fn parse(map: &serde_json::Map<String, serde_json::Value>) -> Self {
        let change_to_break_even = map.get("change_to_break_even").and_then(|v| v.as_f64());
        let last_updated = map.get("last_updated").and_then(|v| v.as_i64());
        let price = map.get("price").and_then(|v| v.as_f64());
        let ticker = map
            .get("ticker")
            .and_then(|v| v.as_str())
            .map(|v| v.to_string());
        let timeframe = map
            .get("timeframe")
            .and_then(|v| v.as_str())
            .map(|v| match v {
                "DELAYED" => Timeframe::Delayed,
                "REAL-TIME" => Timeframe::RealTime,
                _ => Timeframe::Unknown,
            });
        let value = map.get("value").and_then(|v| v.as_f64());
        Self {
            change_to_break_even,
            last_updated,
            price,
            ticker,
            timeframe,
            value,
        }
    }
}
