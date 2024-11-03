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


#[test]
fn test_l2_parse() {
    let data = serde_json::json!({
        "bids": [
            {
                "price": 1.23,
                "size": {
                    "a": 1.0,
                    "b": 2.0,
                }
            }
        ],
        "asks": [
            {
                "price": 4.56,
                "size": {
                    "a": 7.0,
                    "b": 8.0,
                }
            }
        ],
        "bid_count": 1,
        "ask_count": 1,
        "timestamp": 164545545,
        "spread": 3.33,
        "ticker": "TEST"
    });
    let l2 = L2::parse(&data.as_object().unwrap());
    assert_eq!(l2.bids.clone().unwrap()[0].price.unwrap(), 1.23);
    assert_eq!(l2.asks.clone().unwrap()[0].price.unwrap(), 4.56);
    assert_eq!(l2.bid_count.unwrap(), 1.0);
    assert_eq!(l2.ask_count.unwrap(), 1.0);
    assert_eq!(l2.timestamp.unwrap(), 164545545);
    assert_eq!(l2.spread.unwrap(), 3.33);
    assert_eq!(l2.ticker.unwrap(), "TEST");
}
