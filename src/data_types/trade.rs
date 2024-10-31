use crate::data_types::Parse;
use crate::rest::parameters::Timeframe;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Trade {
    pub conditions: Option<Vec<i64>>,
    pub exchange_id: Option<i64>,
    pub price: Option<f64>,
    pub sip_timestamp: Option<i64>,
    pub size: Option<i64>,
    pub trade_id: Option<String>,
    pub timeframe: Option<Timeframe>,
    pub exchange: Option<String>,
    pub trade_correction: Option<i64>,
    pub trf_timestamp: Option<i64>,
    pub sequence_number: Option<i64>,
    pub trf_id: Option<i64>,
    pub participant_timestamp: Option<i64>,
    pub tape: Option<i64>,
}

impl Parse for Trade {
    fn parse(map: &serde_json::Map<String, serde_json::Value>) -> Self {
        let conditions = Self::array_i64_parse(map, vec!["c", "conditions"]);
        let exchange_id = Self::i64_parse(map, vec!["exchange", "x", "exchange_id"]);
        let price = Self::f64_parse(map, vec!["p", "price"]);
        let sip_timestamp = Self::i64_parse(map, vec!["t", "timestamp", "sip_timestamp"]);
        let size = Self::i64_parse(map, vec!["s", "size"]);
        let trade_id = Self::string_parse(map, vec!["i", "id"]);
        let timeframe = match Self::string_parse(map, vec!["timeframe"]) {
            Some(timeframe) => match timeframe.as_str() {
                "DELAYED" => Some(Timeframe::Delayed),
                "REAL-TIME" => Some(Timeframe::RealTime),
                _ => None,
            },
            None => None,
        };
        let exchange = Self::string_parse(map, vec!["T"]);
        let trade_correction = Self::i64_parse(map, vec!["e", "correction"]);
        let trf_timestamp = Self::i64_parse(map, vec!["f", "trf_timestamp"]);
        let trf_id = Self::i64_parse(map, vec!["r", "trf_id"]);
        let sequence_number = Self::i64_parse(map, vec!["q", "sequence_number"]);
        let participant_timestamp = Self::i64_parse(map, vec!["y", "participant_timestamp"]);
        let tape = Self::i64_parse(map, vec!["z", "tape"]);

        Trade {
            conditions,
            exchange_id,
            price,
            sip_timestamp,
            size,
            trade_id,
            timeframe,
            exchange,
            trade_correction,
            trf_timestamp,
            sequence_number,
            trf_id,
            participant_timestamp,
            tape,
        }
    }
}
