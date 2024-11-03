use crate::{
    data_types::{quote::Quote, Parse},
    rest::{
        error::ErrorCode,
        parameters::{Parameter, ParameterRequirment, Parameters, TickerTypes},
    },
    tools::{request::Request, verification::Verification},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LastQuote {
    pub request_id: Option<String>,
    pub results: Option<Quote>,
    pub status: Option<String>,
}

impl LastQuoteRequest for LastQuote {}

impl Parse for LastQuote {
    fn parse(map: &serde_json::Map<String, serde_json::Value>) -> Self {
        let request_id = Self::string_parse(map, vec!["request_id"]);
        let status = Self::string_parse(map, vec!["status"]);
        let results = Self::object_parse(map, vec!["results"]);

        LastQuote {
            request_id,
            results,
            status,
        }
    }
}

pub trait LastQuoteRequest {
    fn get_last_quote(api_key: &String, ticker: String) -> Result<LastQuote, ErrorCode> {
        let last_quote_parameters = Parameters {
            api_key: api_key.to_string(),
            ticker: Some(ticker),
            ..Parameters::default()
        };
        if let Err(check) = Verification::check_parameters(
            &TickerTypes::stocks(),
            PARAMETERS,
            &last_quote_parameters,
        ) {
            return Err(check);
        }
        let url = match url(&last_quote_parameters) {
            Ok(url) => url,
            Err(e) => return Err(e),
        };
        match Request::request(url) {
            Ok(mut map) => Ok(LastQuote::parse(&mut map)),
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
        "https://api.polygon.io/v2/last/nbbo/{}?apiKey={}",
        match &parameters.ticker {
            Some(ticker) => ticker,
            None => return Err(ErrorCode::TickerNotSet),
        },
        &parameters.api_key,
    ));
    Ok(url)
}
#[test]
fn test_last_quote_parse() {
    let data = serde_json::json!({
        "request_id": "req12345",
        "results": {
            "p": 1.23,
            "s": 456,
            "P": 7.89,
            "S": 123,
            "bid_exchange": 10,
            "ask_exchange": 11,
            "t": 164545545,
            "mid_point": 4.56,
            "timeframe": "DELAYED",
            "x": 12,
            "T": "TEST",
            "c": [
                13
            ],
            "f": 164545546,
            "i": [
                14
            ],
            "q": 15,
            "y": 164545547,
            "z": 16
        },
        "status": "OK"
    });
    let last_quote = LastQuote::parse(&data.as_object().unwrap());
    assert_eq!(last_quote.request_id.unwrap(), "req12345");
    assert_eq!(last_quote.status.unwrap(), "OK");
    assert_eq!(last_quote.results.unwrap().bid.unwrap(), 1.23);
}

#[test]
fn test_url() {
    let mut parameters = Parameters::default();
    parameters.api_key = String::from("apiKey");
    parameters.ticker = Some(String::from("AAPL"));
    let url = url(&parameters).unwrap();
    assert_eq!(url, "https://api.polygon.io/v2/last/nbbo/AAPL?apiKey=apiKey");
}
