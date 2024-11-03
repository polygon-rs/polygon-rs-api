use crate::data_types::{day::Day, min::Min, quote::Quote, trade::Trade, Parse};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Ticker {
    pub day: Option<Day>,
    pub last_trade: Option<Trade>,
    pub last_quote: Option<Quote>,
    pub min: Option<Min>,
    pub previous_day: Option<Day>,
    pub ticker: Option<String>,
    pub todays_change: Option<f64>,
    pub todays_change_percent: Option<f64>,
    pub timestamp: Option<i64>,
    pub fair_market_value: Option<f64>,
}

impl Parse for Ticker {
    fn parse(map: &serde_json::Map<String, serde_json::Value>) -> Self {
        let day = Self::object_parse(map, vec!["day"]);
        let last_trade = Self::object_parse(map, vec!["lastTrade"]);
        let last_quote = Self::object_parse(map, vec!["lastQuote"]);
        let min = Self::object_parse(map, vec!["min"]);
        let previous_day = Self::object_parse(map, vec!["prevDay"]);
        let ticker = Self::string_parse(map, vec!["ticker"]);
        let todays_change = Self::f64_parse(map, vec!["todaysChange"]);
        let todays_change_percent = Self::f64_parse(map, vec!["todaysChangePerc"]);
        let timestamp = Self::i64_parse(map, vec!["updated"]);
        let fair_market_value = Self::f64_parse(map, vec!["fmv"]);

        Ticker {
            day,
            last_trade,
            last_quote,
            min,
            previous_day,
            ticker,
            todays_change,
            todays_change_percent,
            timestamp,
            fair_market_value,
        }
    }
}

#[test]
fn test_ticker_parse() {
    let data = serde_json::json!({
        "day": {
            "change": 1.0,
            "change_percent": 2.0,
            "c": 3.0,
            "h": 4.0,
            "last_updated": 164545545,
            "l": 5.0,
            "o": 6.0,
            "previous_close": 7.0,
            "v": 8,
            "vw": 9.0,
            "otc": false
        },
        "lastTrade": {
            "conditions": [
                29
            ],
            "exchange_id": 30,
            "price": 31.0,
            "sip_timestamp": 164545549,
            "size": 32,
            "trade_id": "trade",
            "timeframe": "REAL-TIME",
            "exchange": "TEST1",
            "trade_correction": 33,
            "trf_timestamp": 164545550,
            "sequence_number": 34,
            "trf_id": 35,
            "participant_timestamp": 164545551,
            "tape": 36
        },
        "lastQuote": {
            "p": 1.23,
            "s": 456,
            "P": 7.89,
            "S": 123,
            "bid_exchange": 10,
            "ask_exchange": 11,
            "t": 164545545,
            "mid_point": 4.56,
            "timeframe": "DELAYED",
            "x": 12,
            "T": "TEST",
            "c": [
                13
            ],
            "f": 164545546,
            "i": [
                14
            ],
            "q": 15,
            "y": 164545547,
            "z": 16
        },
        "min": {
            "av": 123456,
            "o": 1.23,
            "h": 2.34,
            "l": 0.12,
            "c": 3.45,
            "v": 456.78,
            "vw": 901.23,
            "n": 123,
            "t": 164545545
        },
        "prevDay": {
            "change": 10.0,
            "change_percent": 20.0,
            "c": 30.0,
            "h": 40.0,
            "last_updated": 164545500,
            "l": 50.0,
            "o": 60.0,
            "previous_close": 70.0,
            "v": 80,
            "vw": 90.0,
            "otc": false
        },
        "ticker": "TEST",
        "todaysChange": 100.0,
        "todaysChangePerc": 200.0,
        "updated": 164545600,
        "fmv": 1000.0
    });
    let ticker = Ticker::parse(&data.as_object().unwrap());
    assert_eq!(ticker.day.unwrap().change.unwrap(), 1.0);
    assert_eq!(ticker.last_trade.unwrap().conditions.unwrap(), vec![29]);
    assert_eq!(ticker.last_quote.unwrap().bid.unwrap(), 1.23);
    assert_eq!(ticker.min.unwrap().accumulated_volume.unwrap(), 123456);
    assert_eq!(ticker.previous_day.unwrap().change.unwrap(), 10.0);
    assert_eq!(ticker.ticker.unwrap(), "TEST");
    assert_eq!(ticker.todays_change.unwrap(), 100.0);
    assert_eq!(ticker.todays_change_percent.unwrap(), 200.0);
    assert_eq!(ticker.timestamp.unwrap(), 164545600);
    assert_eq!(ticker.fair_market_value.unwrap(), 1000.0);
}
