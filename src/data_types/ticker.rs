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
