use crate::data_types::{
    day::Day, details::Details, greeks::Greeks, quote::Quote, trade::Trade,
    underlying_asset::UnderlyingAsset, Parse,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Contract {
    pub break_even_price: Option<f64>,
    pub day: Option<Day>,
    pub details: Option<Details>,
    pub fair_market_value: Option<f64>,
    pub greeks: Option<Greeks>,
    pub implied_volatility: Option<f64>,
    pub quote: Option<Quote>,
    pub trade: Option<Trade>,
    pub open_interest: Option<i64>,
    pub underlying_asset: Option<UnderlyingAsset>,
}

impl Parse for Contract {
    fn parse(map: &serde_json::Map<String, serde_json::Value>) -> Self {
        let break_even_price = Self::f64_parse(map, vec!["break_even_price"]);
        let day = Self::object_parse(map, vec!["day"]);
        let details = Self::object_parse(map, vec!["details"]);
        let fair_market_value = Self::f64_parse(map, vec!["fair_market_value"]);
        let greeks = Self::object_parse(map, vec!["greeks"]);
        let implied_volatility = Self::f64_parse(map, vec!["implied_volatility"]);
        let quote = Self::object_parse(map, vec!["quote"]);
        let trade = Self::object_parse(map, vec!["trade"]);
        let open_interest = Self::i64_parse(map, vec!["open_interest"]);
        let underlying_asset = Self::object_parse(map, vec!["underlying_asset"]);
        Contract {
            break_even_price,
            day,
            details,
            fair_market_value,
            greeks,
            implied_volatility,
            quote,
            trade,
            open_interest,
            underlying_asset,
        }
    }
}
