use crate::data_types::Parse;
use crate::rest::parameters::Timeframe;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Quote {
    pub bid: Option<f64>,
    pub bid_size: Option<i64>,
    pub ask: Option<f64>,
    pub ask_size: Option<i64>,
    pub bid_exchange_id: Option<i64>,
    pub ask_exchange_id: Option<i64>,
    pub last_updated: Option<i64>,
    pub mid_point: Option<f64>,
    pub timeframe: Option<Timeframe>,
    pub exchange_id: Option<i64>,
    pub exchange: Option<String>,
    pub conditions: Option<Vec<i64>>,
    pub trf_timestamp: Option<i64>,
    pub indicators: Option<Vec<i64>>,
    pub sequence_number: Option<i64>,
    pub participant_timestamp: Option<i64>,
    pub tape: Option<i64>,
}

impl Parse for Quote {
    fn parse(map: &serde_json::Map<String, serde_json::Value>) -> Self {
        let bid = Self::f64_parse(map, vec!["p", "b", "bid_price", "bid"]);
        let bid_size = Self::i64_parse(map, vec!["s", "bid_size"]);
        let ask = Self::f64_parse(map, vec!["P", "a", "ask_price", "ask"]);
        let ask_size = Self::i64_parse(map, vec!["S", "ask_size"]);
        let bid_exchange_id = Self::i64_parse(map, vec!["bid_exchange", "bid_exchange_id"]);
        let ask_exchange_id = Self::i64_parse(map, vec!["ask_exchange", "ask_exchange_id"]);
        let last_updated =
            Self::i64_parse(map, vec!["t", "sip_timestamp", "timestamp", "last_updated"]);
        let mid_point = Self::f64_parse(map, vec!["mid_point"]);
        let timeframe = match Self::string_parse(map, vec!["timeframe"]) {
            Some(timeframe) => match timeframe.as_str() {
                "DELAYED" => Some(Timeframe::Delayed),
                "REAL-TIME" => Some(Timeframe::RealTime),
                _ => None,
            },
            None => None,
        };
        let exchange_id = Self::i64_parse(map, vec!["exchange", "x"]);
        let exchange = Self::string_parse(map, vec!["T"]);
        let conditions = Self::array_i64_parse(map, vec!["c", "conditions"]);
        let trf_timestamp = Self::i64_parse(map, vec!["f", "trf_timestamp"]);
        let indicators = Self::array_i64_parse(map, vec!["i", "indicators"]);
        let sequence_number = Self::i64_parse(map, vec!["q", "sequence_number"]);
        let participant_timestamp = Self::i64_parse(map, vec!["y", "participant_timestamp"]);
        let tape = Self::i64_parse(map, vec!["z", "tape"]);

        Quote {
            bid,
            bid_size,
            ask,
            ask_size,
            bid_exchange_id,
            ask_exchange_id,
            last_updated,
            mid_point,
            timeframe,
            exchange_id,
            exchange,
            conditions,
            trf_timestamp,
            indicators,
            sequence_number,
            participant_timestamp,
            tape,
        }
    }
}
