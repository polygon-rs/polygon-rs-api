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

#[test]
fn test_universal_parse() {
    let data = serde_json::json!({
        "break_even_price": 1.23,
        "details": {
            "contract_type": "Call",
            "contract_style": "American",
            "expiration_date": "2023-03-03",
            "shares_per_contract": 100,
            "strike_price": 10.0,
            "ticker": "TEST"
        },
        "fmv": 11.0,
        "greeks": {
            "delta": 12.0,
            "gamma": 13.0,
            "theta": 14.0,
            "vega": 15.0
        },
        "implied_volatility": 16.0,
        "last_quote": {
            "p": 17.0,
            "s": 18,
            "P": 19.0,
            "S": 20,
            "bid_exchange": 21,
            "ask_exchange": 22,
            "t": 164545546,
            "mid_point": 23.0,
            "timeframe": "DELAYED",
            "x": 24,
            "T": "TEST",
            "c": [
                25
            ],
            "f": 164545547,
            "i": [
                26
            ],
            "q": 27,
            "y": 164545548,
            "z": 28
        },
        "last_trade": {
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
        "market_status": "string",
        "name": "string",
        "open_interest": 37,
        "session": {
            "change": 38.0,
            "change_percent": 39.0,
            "close": 40.0,
            "high": 41.0,
            "low": 42.0,
            "open": 43.0,
            "previous_close": 44.0
        },
        "ticker": "string",
        "type": "options",
        "underlying_asset": {
            "change_to_break_even": 45.0,
            "last_updated": 164545552,
            "price": 46.0,
            "ticker": "TEST2",
            "timeframe": "REAL-TIME",
            "value": 47.0
        },
        "error": "string",
        "message": "string",
        "value": 48.0
    });
    let universal = Universal::parse(&data.as_object().unwrap());
    assert_eq!(universal.break_even_price.unwrap(), 1.23);
    assert_eq!(universal.details.unwrap().contract_type.unwrap(), crate::rest::parameters::ContractType::Call);
    assert_eq!(universal.fair_market_value.unwrap(), 11.0);
    assert_eq!(universal.greeks.unwrap().delta.unwrap(), 12.0);
    assert_eq!(universal.implied_volatility.unwrap(), 16.0);
    assert_eq!(universal.last_quote.unwrap().bid.unwrap(), 17.0);
    assert_eq!(universal.last_trade.unwrap().conditions.unwrap(), vec![29]);
    assert_eq!(universal.market_status.unwrap(), "string");
    assert_eq!(universal.name.unwrap(), "string");
    assert_eq!(universal.open_interest.unwrap(), 37);
    assert_eq!(universal.session.unwrap().change.unwrap(), 38.0);
    assert_eq!(universal.ticker.unwrap(), "string");
    assert_eq!(universal.ticker_type.unwrap(), TickerType::Options);
    assert_eq!(universal.underlying_asset.unwrap().change_to_break_even.unwrap(), 45.0);
    assert_eq!(universal.error.unwrap(), "string");
    assert_eq!(universal.message.unwrap(), "string");
    assert_eq!(universal.value.unwrap(), 48.0);
}
