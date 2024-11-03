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

#[test]
fn test_day_parse() {
    let data = serde_json::json!({
        "change": 1.0,
        "change_percent": 2.0,
        "c": 3.0,
        "h": 4.0,
        "last_updated": 164545545,
        "l": 5.0,
        "o": 6.0,
        "previous_close": 7.0,
        "v": 8,
        "vw": 9.0,
        "otc": false
    });
    let day = Day::parse(&data.as_object().unwrap());
    assert_eq!(day.change.unwrap(), 1.0);
    assert_eq!(day.change_percent.unwrap(), 2.0);
    assert_eq!(day.close.unwrap(), 3.0);
    assert_eq!(day.high.unwrap(), 4.0);
    assert_eq!(day.last_updated.unwrap(), 164545545);
    assert_eq!(day.low.unwrap(), 5.0);
    assert_eq!(day.open.unwrap(), 6.0);
    assert_eq!(day.previous_close.unwrap(), 7.0);
    assert_eq!(day.volume.unwrap(), 8);
    assert_eq!(day.volume_weighted_average_price.unwrap(), 9.0);
    assert_eq!(day.otc.unwrap(), false);
}
