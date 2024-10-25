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
    fn parse(map: &mut serde_json::Map<String, serde_json::Value>) -> Self {
        let break_even_price = map.get("break_even_price").and_then(|v| v.as_f64());
        let day = map
            .get_mut("day")
            .and_then(|v| v.as_object_mut())
            .map(|v| Day::parse(v));
        let details = map
            .get_mut("details")
            .and_then(|v| v.as_object_mut())
            .map(|v| Details::parse(v));
        let fair_market_value = map.get("fair_market_value").and_then(|v| v.as_f64());
        let greeks = map
            .get_mut("greeks")
            .and_then(|v| v.as_object_mut())
            .map(|v| Greeks::parse(v));
        let implied_volatility = map.get("implied_volatility").and_then(|v| v.as_f64());
        let quote = map
            .get_mut("quote")
            .and_then(|v| v.as_object_mut())
            .map(|v| Quote::parse(v));
        let trade = map
            .get_mut("trade")
            .and_then(|v| v.as_object_mut())
            .map(|v| Trade::parse(v));
        let open_interest = map.get("open_interest").and_then(|v| v.as_i64());
        let underlying_asset = map
            .get_mut("underlying_asset")
            .and_then(|v| v.as_object_mut())
            .map(|v| UnderlyingAsset::parse(v));
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
