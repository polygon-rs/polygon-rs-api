use crate::data_types::{
    details::Details, greeks::Greeks, quote::Quote, session::Session, trade::Trade,
    underlying_asset::UnderlyingAsset, Parse,
};
use crate::rest::parameters::TickerType;
use serde::{Deserialize, Serialize};

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
    fn parse(map: &serde_json::Map<String, serde_json::Value>) -> Self {
        let break_even_price = Self::f64_parse(map, vec!["break_even_price"]);
        let details = Self::object_parse(map, vec!["details"]);
        let fair_market_value = Self::f64_parse(map, vec!["fmv"]);
        let greeks = Self::object_parse(map, vec!["greeks"]);
        let implied_volatility = Self::f64_parse(map, vec!["implied_volatility"]);
        let last_quote = Self::object_parse(map, vec!["last_quote"]);
        let last_trade = Self::object_parse(map, vec!["last_trade"]);
        let market_status = Self::string_parse(map, vec!["market_status"]);
        let name = Self::string_parse(map, vec!["name"]);
        let open_interest = Self::i64_parse(map, vec!["open_interest"]);
        let session = Self::object_parse(map, vec!["session"]);
        let ticker = Self::string_parse(map, vec!["ticker"]);
        let ticker_type = match Self::string_parse(map, vec!["type"]) {
            Some(s) => match s.as_str() {
                "stocks" => Some(TickerType::Stocks),
                "options" => Some(TickerType::Options),
                "indices" => Some(TickerType::Indicies),
                "forex" => Some(TickerType::Forex),
                "crypto" => Some(TickerType::Crypto),
                _ => None,
            },
            None => None,
        };
        let underlying_asset = Self::object_parse(map, vec!["underlying_asset"]);
        let error = Self::string_parse(map, vec!["error"]);
        let message = Self::string_parse(map, vec!["message"]);
        let value = Self::f64_parse(map, vec!["value"]);

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
