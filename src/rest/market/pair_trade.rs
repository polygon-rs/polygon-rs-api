use crate::{
    data_types::{trade::Trade, Parse},
    rest::{
        error::ErrorCode,
        parameters::{Parameter, ParameterRequirment, Parameters, TickerTypes},
    },
    tools::{request::Request, verification::Verification},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PairTrade {
    pub request_id: Option<String>,
    pub pair_trade: Option<Trade>,
    pub status: Option<String>,
    pub symbol: Option<String>,
}

impl PairTradeRequest for PairTrade {}

impl Parse for PairTrade {
    fn parse(map: &serde_json::Map<String, serde_json::Value>) -> Self {
        let request_id = Self::string_parse(map, vec!["request_id"]);
        let pair_trade = Self::object_parse(map, vec!["last"]);
        let status = Self::string_parse(map, vec!["status"]);
        let symbol = Self::string_parse(map, vec!["symbol"]);

        PairTrade {
            request_id,
            pair_trade,
            status,
            symbol,
        }
    }
}

pub trait PairTradeRequest {
    //Once reference to tickers is complete extract to and from via ticker reference
    fn get_pair_trade(api_key: &String, from: String, to: String) -> Result<PairTrade, ErrorCode> {
        let pair_trade_parameters = Parameters {
            api_key: api_key.to_string(),
            ticker: Some(format!("X:{}{}",from,to)),
            ..Parameters::default()
        };
        if let Err(check) = Verification::check_parameters(
            &&TickerTypes::crypto(),
            PARAMETERS,
            &pair_trade_parameters,
        ) {
            return Err(check);
        }
        let url = match url(&pair_trade_parameters, from, to) {
            Ok(url) => url,
            Err(e) => return Err(e),
        };
        match Request::request(url) {
            Ok(mut map) => Ok(PairTrade::parse(&mut map)),
            Err(e) => return Err(e),
        }
    }
}

const PARAMETERS: &'static [&'static ParameterRequirment] = &[&ParameterRequirment {
    required: true,
    parameter: Parameter::Ticker,
}];

fn url(parameters: &Parameters, from: String, to: String) -> Result<String, ErrorCode> {
    let url = String::from(format!(
        "https://api.polygon.io/v1/last/crypto/{}/{}?apiKey={}",
        from, to, &parameters.api_key,
    ));
    Ok(url)
}
#[test]
fn test_pair_trade_parse() {
    let data = serde_json::json!({
        "request_id": "req12345",
        "last": {
            "conditions": [
                1
            ],
            "exchange": 10,
            "price": 1.23,
            "size": 100,
            "timestamp": 164545545,
            "symbol": "X:BTCUSD"
        },
        "status": "OK"
    });
    let pair_trade = PairTrade::parse(&data.as_object().unwrap());
    assert_eq!(pair_trade.request_id.unwrap(), "req12345");
    assert_eq!(pair_trade.status.unwrap(), "OK");
    assert_eq!(pair_trade.pair_trade.unwrap().conditions.unwrap(), vec![1]);
}

#[test]
fn test_url() {
    let mut parameters = Parameters::default();
    parameters.api_key = String::from("apiKey");
    parameters.ticker = Some(String::from("X:BTCUSD"));
    let url = url(&parameters, String::from("BTC"), String::from("USD")).unwrap();
    assert_eq!(url, "https://api.polygon.io/v1/last/crypto/BTC/USD?apiKey=apiKey");
}
