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

#[test]
fn test_contract_parse() {
    let data = serde_json::json!({
        "break_even_price": 1.23,
        "day": {
            "change": 1.0,
            "change_percent": 2.0,
            "close": 3.0,
            "high": 4.0,
            "last_updated": 164545545,
            "low": 5.0,
            "open": 6.0,
            "previous_close": 7.0,
            "volume": 8,
            "volume_weighted_average_price": 9.0,
            "otc": false
        },
        "details": {
            "contract_type": "Call",
            "contract_style": "American",
            "expiration_date": "2023-03-03",
            "shares_per_contract": 100,
            "strike_price": 10.0,
            "ticker": "TEST"
        },
        "fair_market_value": 11.0,
        "greeks": {
            "delta": 12.0,
            "gamma": 13.0,
            "theta": 14.0,
            "vega": 15.0
        },
        "implied_volatility": 16.0,
        "quote": {
            "bid": 17.0,
            "bid_size": 18,
            "ask": 19.0,
            "ask_size": 20,
            "bid_exchange_id": 21,
            "ask_exchange_id": 22,
            "last_updated": 164545546,
            "mid_point": 23.0,
            "timeframe": "DELAYED",
            "exchange_id": 24,
            "exchange": "TEST",
            "conditions": [
                25
            ],
            "trf_timestamp": 164545547,
            "indicators": [
                26
            ],
            "sequence_number": 27,
            "participant_timestamp": 164545548,
            "tape": 28
        },
        "trade": {
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
        "open_interest": 37,
        "underlying_asset": {
            "change_to_break_even": 38.0,
            "last_updated": 164545552,
            "price": 39.0,
            "ticker": "TEST2",
            "timeframe": "REAL-TIME",
            "value": 40.0
        }
    });
    let contract = Contract::parse(&data.as_object().unwrap());
    assert_eq!(contract.break_even_price.unwrap(), 1.23);
    assert_eq!(contract.day.unwrap().change.unwrap(), 1.0);
    assert_eq!(contract.details.unwrap().contract_type.unwrap(), crate::rest::parameters::ContractType::Call);
    assert_eq!(contract.fair_market_value.unwrap(), 11.0);
    assert_eq!(contract.greeks.unwrap().delta.unwrap(), 12.0);
    assert_eq!(contract.implied_volatility.unwrap(), 16.0);
    assert_eq!(contract.quote.unwrap().bid.unwrap(), 17.0);
    assert_eq!(contract.trade.unwrap().conditions.unwrap(), vec![29]);
    assert_eq!(contract.open_interest.unwrap(), 37);
    assert_eq!(contract.underlying_asset.unwrap().ticker.unwrap(), "TEST2");
}
