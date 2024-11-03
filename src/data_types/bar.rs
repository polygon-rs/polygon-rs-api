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
    fn parse(map: &serde_json::Map<String, serde_json::Value>) -> Self {
        let excahnge = Self::string_parse(map, vec!["T"]);
        let close = Self::f64_parse(map, vec!["c"]);
        let high = Self::f64_parse(map, vec!["h"]);
        let low = Self::f64_parse(map, vec!["l"]);
        let open = Self::f64_parse(map, vec!["o"]);
        let transactions = Self::i64_parse(map, vec!["n"]);
        let timestamp = Self::i64_parse(map, vec!["t"]);
        let volume = Self::f64_parse(map, vec!["v"]);
        let volume_weighted = Self::f64_parse(map, vec!["vw"]);
        let otc = Self::bool_parse(map, vec!["otc"]);
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

#[test]
fn test_bar_parse() {
    let data = serde_json::json!({
        "T": "test_exchange",
        "c": 1.23,
        "h": 2.34,
        "l": 0.12,
        "n": 123,
        "o": 0.12,
        "t": 164545545,
        "v": 456.78,
        "vw": 901.23,
        "otc": false
    });
    let bar = Bar::parse(&data.as_object().unwrap());
    assert_eq!(bar.excahnge.unwrap(), "test_exchange");
    assert_eq!(bar.close.unwrap(), 1.23);
    assert_eq!(bar.high.unwrap(), 2.34);
    assert_eq!(bar.low.unwrap(), 0.12);
    assert_eq!(bar.transactions.unwrap(), 123);
    assert_eq!(bar.open.unwrap(), 0.12);
    assert_eq!(bar.timestamp.unwrap(), 164545545);
    assert_eq!(bar.volume.unwrap(), 456.78);
    assert_eq!(bar.volume_weighted.unwrap(), 901.23);
    assert_eq!(bar.otc.unwrap(), false);
}