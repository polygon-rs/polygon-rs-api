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
    fn parse(map: &mut serde_json::Map<String, serde_json::Value>) -> Self {
        let bids = map.get("bids").and_then(|v| v.as_array()).map(|v| {
            let mut bids = Vec::new();
            for bid in v {
                bids.push(Bid::parse(bid.clone().as_object_mut().unwrap()));
            }
            bids
        });
        let asks = map.get("asks").and_then(|v| v.as_array()).map(|v| {
            let mut asks = Vec::new();
            for ask in v {
                asks.push(Ask::parse(ask.clone().as_object_mut().unwrap()));
            }
            asks
        });
        let bid_count = map.get("bid_count").and_then(|v| v.as_f64());
        let ask_count = map.get("ask_count").and_then(|v| v.as_f64());
        let timestamp = map.get("timestamp").and_then(|v| v.as_i64());
        let spread = map.get("spread").and_then(|v| v.as_f64());
        let ticker = map
            .get("ticker")
            .and_then(|v| v.as_str())
            .map(|v| v.to_string());
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
