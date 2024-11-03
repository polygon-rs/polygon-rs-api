use crate::data_types::Parse;
use crate::rest::parameters::Timeframe;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
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
        let change_to_break_even = Self::f64_parse(map, vec!["change_to_break_even"]);
        let last_updated = Self::i64_parse(map, vec!["last_updated"]);
        let price = Self::f64_parse(map, vec!["price"]);
        let ticker = Self::string_parse(map, vec!["ticker"]);
        let timeframe = match Self::string_parse(map, vec!["timeframe"]) {
            Some(timeframe) => match timeframe.as_str() {
                "DELAYED" => Some(Timeframe::Delayed),
                "REAL-TIME" => Some(Timeframe::RealTime),
                _ => None,
            },
            None => None,
        };
        let value = Self::f64_parse(map, vec!["value"]);

        UnderlyingAsset {
            change_to_break_even,
            last_updated,
            price,
            ticker,
            timeframe,
            value,
        }
    }
}

#[test]
fn test_underlying_asset_parse() {
    let data = serde_json::json!({
        "change_to_break_even": 1.23,
        "last_updated": 164545545,
        "price": 2.34,
        "ticker": "TEST",
        "timeframe": "REAL-TIME",
        "value": 3.45
    });
    let underlying_asset = UnderlyingAsset::parse(&data.as_object().unwrap());
    assert_eq!(underlying_asset.change_to_break_even.unwrap(), 1.23);
    assert_eq!(underlying_asset.last_updated.unwrap(), 164545545);
    assert_eq!(underlying_asset.price.unwrap(), 2.34);
    assert_eq!(underlying_asset.ticker.unwrap(), "TEST");
    assert_eq!(underlying_asset.timeframe.unwrap(), Timeframe::RealTime);
    assert_eq!(underlying_asset.value.unwrap(), 3.45);
}
