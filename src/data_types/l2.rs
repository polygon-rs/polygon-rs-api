use crate::data_types::{ask::Ask, bid::Bid, Parse};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct L2 {
    pub bids: Option<Vec<Bid>>,
    pub asks: Option<Vec<Ask>>,
    pub bid_count: Option<f64>,
    pub ask_count: Option<f64>,
    pub timestamp: Option<i64>,
    pub spread: Option<f64>,
    pub ticker: Option<String>,
}

impl Parse for L2 {
    fn parse(map: &serde_json::Map<String, serde_json::Value>) -> Self {
        let bids = Self::array_parse(map, vec!["bids"]);
        let asks = Self::array_parse(map, vec!["asks"]);
        let bid_count = Self::f64_parse(map, vec!["bid_count"]);
        let ask_count = Self::f64_parse(map, vec!["ask_count"]);
        let timestamp = Self::i64_parse(map, vec!["timestamp"]);
        let spread = Self::f64_parse(map, vec!["spread"]);
        let ticker = Self::string_parse(map, vec!["ticker"]);
        L2 {
            bids,
            asks,
            bid_count,
            ask_count,
            timestamp,
            spread,
            ticker,
        }
    }
}
