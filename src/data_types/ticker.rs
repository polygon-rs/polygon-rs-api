use crate::data_types::{day::Day, quote::Quote, trade::Trade, min::Min, Parse};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Ticker{
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
    fn parse(map: &mut serde_json::Map<String, serde_json::Value>) -> Self {
        let day = map.get_mut("day").and_then(|v| v.as_object_mut()).map(|v| Day::parse(v));
        let last_trade = map.get_mut("lastTrade").and_then(|v| v.as_object_mut()).map(|v| Trade::parse(v));
        let last_quote = map.get_mut("lastQuote").and_then(|v| v.as_object_mut()).map(|v| Quote::parse(v));
        let min = map.get_mut("min").and_then(|v| v.as_object_mut()).map(|v| Min::parse(v));
        let previous_day = map.get_mut("prevDay").and_then(|v| v.as_object_mut()).map(|v| Day::parse(v));
        let ticker = map.get("ticker").and_then(|v| v.as_str()).map(|v| v.to_string());
        let todays_change = map.get("todaysChange").and_then(|v| v.as_f64());
        let todays_change_percent = map.get("todaysChangePerc").and_then(|v| v.as_f64());
        let timestamp = map.get("updated").and_then(|v| v.as_i64());
        let fair_market_value = map.get("fmv").and_then(|v| v.as_f64());
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
