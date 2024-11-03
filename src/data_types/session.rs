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

#[test]
fn test_session_parse() {
    let data = serde_json::json!({
        "change": 1.23,
        "change_percent": 2.34,
        "close": 3.45,
        "high": 4.56,
        "low": 5.67,
        "open": 6.78,
        "previous_close": 7.89
    });
    let session = Session::parse(&data.as_object().unwrap());
    assert_eq!(session.change.unwrap(), 1.23);
    assert_eq!(session.change_percent.unwrap(), 2.34);
    assert_eq!(session.close.unwrap(), 3.45);
    assert_eq!(session.high.unwrap(), 4.56);
    assert_eq!(session.low.unwrap(), 5.67);
    assert_eq!(session.open.unwrap(), 6.78);
    assert_eq!(session.previous_close.unwrap(), 7.89);
}
