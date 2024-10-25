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
    fn parse(map: &mut serde_json::Map<String, serde_json::Value>) -> Self {
        let timestamp = map.get("last_updated").and_then(|v| v.as_i64());
        let market_status = map
            .get("market_status")
            .and_then(|v| v.as_str())
            .map(|v| v.to_string());
        let name = map
            .get("name")
            .and_then(|v| v.as_str())
            .map(|v| v.to_string());
        let session: Option<Session> = map
            .get_mut("session")
            .and_then(|v| v.as_object_mut())
            .map(|v| Session::parse(v));
        let ticker = map
            .get("ticker")
            .and_then(|v| v.as_str())
            .map(|v| v.to_string());
        let timeframe = map
            .get("timeframe")
            .and_then(|v| v.as_str())
            .map(|v| v.to_string());
        let ticker_type = map
            .get("ticker_type")
            .and_then(|v| v.as_str())
            .map(|v| match v {
                "stocks" => TickerType::Stocks,
                "options" => TickerType::Options,
                "indices" => TickerType::Indicies,
                "forex" => TickerType::Forex,
                "crypto" => TickerType::Crypto,
                _ => TickerType::default(),
            });
        let value = map.get("value").and_then(|v| v.as_f64());
        let error = map
            .get("error")
            .and_then(|v| v.as_str())
            .map(|v| v.to_string());
        let message = map
            .get("message")
            .and_then(|v| v.as_str())
            .map(|v| v.to_string());
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
