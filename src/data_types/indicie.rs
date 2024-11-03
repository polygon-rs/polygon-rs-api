use crate::data_types::{session::Session, Parse};
use crate::rest::parameters::TickerType;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Indicie {
    pub timestamp: Option<i64>,
    pub market_status: Option<String>,
    pub name: Option<String>,
    pub session: Option<Session>,
    pub ticker: Option<String>,
    pub timeframe: Option<String>,
    pub ticker_type: Option<TickerType>,
    pub value: Option<f64>,
    pub error: Option<String>,
    pub message: Option<String>,
}

impl Parse for Indicie {
    fn parse(map: &serde_json::Map<String, serde_json::Value>) -> Self {
        let timestamp = Self::i64_parse(map, vec!["timestamp"]);
        let market_status = Self::string_parse(map, vec!["market_status"]);
        let name = Self::string_parse(map, vec!["name"]);
        let session = Self::object_parse(map, vec!["session"]);
        let ticker = Self::string_parse(map, vec!["ticker"]);
        let timeframe = Self::string_parse(map, vec!["timeframe"]);
        let ticker_type = match Self::string_parse(map, vec!["ticker_type"]) {
            Some(ticker_type) => match ticker_type.as_str() {
                "stocks" => Some(TickerType::Stocks),
                "options" => Some(TickerType::Options),
                "indicies" => Some(TickerType::Indicies),
                "forex" => Some(TickerType::Forex),
                "crypto" => Some(TickerType::Crypto),
                _ => None,
            },
            None => None,
        };
        let value = Self::f64_parse(map, vec!["value"]);
        let error = Self::string_parse(map, vec!["error"]);
        let message = Self::string_parse(map, vec!["message"]);
        Indicie {
            timestamp,
            market_status,
            name,
            session,
            ticker,
            timeframe,
            ticker_type,
            value,
            error,
            message,
        }
    }
}

#[test]
fn test_indicie_parse() {
    let data = serde_json::json!({
        "timestamp": 1679756220000 as i64,
        "market_status": "PRE",
        "name": "Dow Jones Industrial Average",
        "session": {
            "change": 1.23,
            "change_percent": 2.34,
            "close": 3.45,
            "high": 4.56,
            "low": 5.67,
            "open": 6.78,
            "previous_close": 7.89
        },
        "ticker": "DJIA",
        "timeframe": "2023-03-25",
        "ticker_type": "indicies",
        "value": 12345.67,
        "error": null,
        "message": null
    });
    let indicie = Indicie::parse(&data.as_object().unwrap());
    assert_eq!(indicie.timestamp.unwrap(), 1679756220000);
    assert_eq!(indicie.market_status.unwrap(), "PRE");
    assert_eq!(indicie.name.unwrap(), "Dow Jones Industrial Average");
    assert_eq!(indicie.session.unwrap().change.unwrap(), 1.23);
    assert_eq!(indicie.ticker.unwrap(), "DJIA");
    assert_eq!(indicie.timeframe.unwrap(), "2023-03-25");
    assert_eq!(indicie.ticker_type.unwrap(), TickerType::Indicies);
    assert_eq!(indicie.value.unwrap(), 12345.67);
    assert_eq!(indicie.error, None);
    assert_eq!(indicie.message, None);
}
