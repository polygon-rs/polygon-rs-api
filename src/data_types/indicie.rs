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
                "indices" => Some(TickerType::Indicies),
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
