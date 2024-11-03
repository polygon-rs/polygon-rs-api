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
pub struct LastTrade {
    pub request_id: Option<String>,
    pub results: Option<Trade>,
    pub status: Option<String>,
}

impl LastTradeRequest for LastTrade {}

impl Parse for LastTrade {
    fn parse(map: &serde_json::Map<String, serde_json::Value>) -> Self {
        let request_id = Self::string_parse(map, vec!["request_id"]);
        let status = Self::string_parse(map, vec!["status"]);
        let results = Self::object_parse(map, vec!["results"]);

        LastTrade {
            request_id,
            results,
            status,
        }
    }
}

pub trait LastTradeRequest {
    fn get_last_trade(api_key: &String, ticker: String) -> Result<LastTrade, ErrorCode> {
        let last_trade_parameters = Parameters {
            api_key: api_key.to_string(),
            ticker: Some(ticker),
            ..Parameters::default()
        };
        if let Err(check) = Verification::check_parameters(
            &TickerTypes::set(true, true, false, false, false),
            PARAMETERS,
            &last_trade_parameters,
        ) {
            return Err(check);
        }
        let url = match url(&last_trade_parameters) {
            Ok(url) => url,
            Err(e) => return Err(e),
        };
        match Request::request(url) {
            Ok(mut map) => Ok(LastTrade::parse(&mut map)),
            Err(e) => return Err(e),
        }
    }
}

const PARAMETERS: &'static [&'static ParameterRequirment] = &[&ParameterRequirment {
    required: true,
    parameter: Parameter::Ticker,
}];

fn url(parameters: &Parameters) -> Result<String, ErrorCode> {
    let url = String::from(format!(
        "https://api.polygon.io/v2/last/trade/{}apiKey={}",
        match &parameters.ticker {
            Some(ticker) => ticker,
            None => return Err(ErrorCode::TickerNotSet),
        },
        &parameters.api_key,
    ));
    Ok(url)
}
#[test]
fn test_last_trade_parse() {
    let data = serde_json::json!({
        "request_id": "req12345",
        "results": {
            "c": [
                29
            ],
            "x": 30,
            "p": 31.0,
            "t": 164545549,
            "s": 32,
            "i": "trade",
            "timeframe": "REAL-TIME",
            "T": "TEST1",
            "e": 33,
            "f": 164545550,
            "q": 34,
            "r": 35,
            "y": 164545551,
            "z": 36
        },
        "status": "OK"
    });
    let last_trade = LastTrade::parse(&data.as_object().unwrap());
    assert_eq!(last_trade.request_id.unwrap(), "req12345");
    assert_eq!(last_trade.status.unwrap(), "OK");
    assert_eq!(last_trade.results.unwrap().conditions.unwrap(), vec![29]);
}

#[test]
fn test_url() {
    let mut parameters = Parameters::default();
    parameters.api_key = String::from("apiKey");
    parameters.ticker = Some(String::from("AAPL"));
    let url = url(&parameters).unwrap();
    assert_eq!(url, "https://api.polygon.io/v2/last/trade/AAPLapiKey=apiKey");
}
