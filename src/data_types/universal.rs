use crate::data_types::{
    details::Details, greeks::Greeks, quote::Quote, session::Session, trade::Trade, Parse,
};
use crate::rest::parameters::TickerType;
use serde::{Deserialize, Serialize};

use super::underlying_asset::UnderlyingAsset;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Universal {
    pub break_even_price: Option<f64>,
    pub details: Option<Details>,
    pub fair_market_value: Option<f64>,
    pub greeks: Option<Greeks>,
    pub implied_volatility: Option<f64>,
    pub last_quote: Option<Quote>,
    pub last_trade: Option<Trade>,
    pub market_status: Option<String>,
    pub name: Option<String>,
    pub open_interest: Option<i64>,
    pub session: Option<Session>,
    pub ticker: Option<String>,
    pub ticker_type: Option<TickerType>,
    pub underlying_asset: Option<UnderlyingAsset>,
    pub error: Option<String>,
    pub message: Option<String>,
    pub value: Option<f64>,
}

impl Parse for Universal {
    fn parse(map: &mut serde_json::Map<String, serde_json::Value>) -> Self {
        let break_even_price = map.get("break_even_price").and_then(|v| v.as_f64());
        let details = map
            .get_mut("details")
            .and_then(|v| v.as_object_mut().map(|v| Details::parse(v)));
        let fair_market_value = map.get("fmv").and_then(|v| v.as_f64());
        let greeks = map
            .get_mut("greeks")
            .and_then(|v| v.as_object_mut().map(|v| Greeks::parse(v)));
        let implied_volatility = map.get("implied_volatility").and_then(|v| v.as_f64());
        let last_quote = map
            .get_mut("last_quote")
            .and_then(|v| v.as_object_mut().map(|v| Quote::parse(v)));
        let last_trade = map
            .get_mut("last_trade")
            .and_then(|v| v.as_object_mut().map(|v| Trade::parse(v)));
        let market_status = map
            .get("market_status")
            .and_then(|v| v.as_str())
            .map(|v| v.to_string());
        let name = map
            .get("name")
            .and_then(|v| v.as_str())
            .map(|v| v.to_string());
        let open_interest = map.get("open_interest").and_then(|v| v.as_i64());
        let session = map
            .get_mut("session")
            .and_then(|v| v.as_object_mut().map(|v| Session::parse(v)));
        let ticker = map
            .get("ticker")
            .and_then(|v| v.as_str())
            .map(|v| v.to_string());
        let ticker_type = map
            .get_mut("type")
            .and_then(|v| v.as_str())
            .map(|v| match v {
                "stocks" => TickerType::Stocks,
                "options" => TickerType::Options,
                "indices" => TickerType::Indicies,
                "forex" => TickerType::Forex,
                "crypto" => TickerType::Crypto,
                _ => TickerType::default(),
            });
        let underlying_asset = map
            .get_mut("underlying_asset")
            .and_then(|v| v.as_object_mut().map(|v| UnderlyingAsset::parse(v)));
        let error = map
            .get("error")
            .and_then(|v| v.as_str())
            .map(|v| v.to_string());
        let message = map
            .get("message")
            .and_then(|v| v.as_str())
            .map(|v| v.to_string());
        let value = map.get("value").and_then(|v| v.as_f64());
        Universal {
            break_even_price,
            details,
            fair_market_value,
            greeks,
            implied_volatility,
            last_quote,
            last_trade,
            market_status,
            name,
            open_interest,
            session,
            ticker,
            ticker_type,
            underlying_asset,
            error,
            message,
            value,
        }
    }
}
