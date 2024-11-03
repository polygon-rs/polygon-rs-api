use crate::{
    data_types::{bar::Bar, Parse},
    rest::{
        error::ErrorCode,
        parameters::{Parameter, ParameterRequirment, Parameters, TickerTypes},
    },
    tools::{request::Request, verification::Verification},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Previous {
    pub adjusted: Option<bool>,
    pub query_count: Option<i64>,
    pub request_id: Option<String>,
    pub results: Option<Vec<Bar>>,
    pub results_count: Option<i64>,
    pub status: Option<String>,
    pub ticker: Option<String>,
}

impl PreviousRequest for Previous {}

impl Parse for Previous {
    fn parse(map: &serde_json::Map<String, serde_json::Value>) -> Self {
        let adjusted = Self::bool_parse(map, vec!["adjusted"]);
        let request_id = Self::string_parse(map, vec!["request_id"]);
        let status = Self::string_parse(map, vec!["status"]);
        let results = Self::array_parse(map, vec!["results"]);
        let results_count = Self::i64_parse(map, vec!["resultsCount"]);
        let query_count = Self::i64_parse(map, vec!["queryCount"]);
        let ticker = Self::string_parse(map, vec!["ticker"]);

        Previous {
            adjusted,
            request_id,
            query_count,
            results,
            results_count,
            status,
            ticker,
        }
    }
}

pub trait PreviousRequest {
    fn get_previous(
        api_key: &String,
        ticker: String,
        adjusted: Option<bool>,
    ) -> Result<Previous, ErrorCode> {
        let previous_parameters = Parameters {
            api_key: api_key.to_string(),
            ticker: Some(ticker),
            adjusted: adjusted,
            ..Parameters::default()
        };
        if let Err(check) =
            Verification::check_parameters(&TickerTypes::all(), PARAMETERS, &previous_parameters)
        {
            return Err(check);
        }
        let url = match url(&previous_parameters) {
            Ok(url) => url,
            Err(e) => return Err(e),
        };
        match Request::request(url) {
            Ok(mut map) => Ok(Previous::parse(&mut map)),
            Err(e) => return Err(e),
        }
    }
}

const PARAMETERS: &'static [&'static ParameterRequirment] = &[
    &ParameterRequirment {
        required: true,
        parameter: Parameter::Ticker,
    },
    &ParameterRequirment {
        required: false,
        parameter: Parameter::Adjusted,
    },
];

fn url(parameters: &Parameters) -> Result<String, ErrorCode> {
    let url = String::from(format!(
        "https://api.polygon.io/v2/aggs/ticker/{}/prev?{}apiKey={}",
        match &parameters.ticker {
            Some(ticker) => ticker,
            None => return Err(ErrorCode::TickerNotSet),
        },
        if let Some(adj) = &parameters.adjusted {
            format!("adjusted={}&", adj)
        } else {
            "".to_string()
        },
        &parameters.api_key,
    ));
    Ok(url)
}
#[test]
fn test_previous_parse() {
    let data = serde_json::json!({
        "ticker": "AAPL",
        "status": "OK",
        "adjusted": true,
        "queryCount": 1,
        "resultsCount": 1,
        "results": [
            {
                "v": 123456,
                "vw": 1.23,
                "o": 2.34,
                "c": 3.45,
                "h": 4.56,
                "l": 0.12,
                "t": 164545545,
                "n": 123
            }
        ],
        "request_id": "req12345"
    });
    let previous = Previous::parse(&data.as_object().unwrap());
    assert_eq!(previous.ticker.unwrap(), "AAPL");
    assert_eq!(previous.status.unwrap(), "OK");
    assert_eq!(previous.adjusted.unwrap(), true);
    assert_eq!(previous.query_count.unwrap(), 1);
    assert_eq!(previous.results_count.unwrap(), 1);
    assert_eq!(previous.results.unwrap()[0].volume.unwrap(), 123456.0);
    assert_eq!(previous.request_id.unwrap(), "req12345");
}

#[test]
fn test_url() {
    let mut parameters = Parameters::default();
    parameters.api_key = String::from("apiKey");
    parameters.ticker = Some(String::from("AAPL"));
    parameters.adjusted = Some(true);
    let url = url(&parameters).unwrap();
    assert_eq!(url, "https://api.polygon.io/v2/aggs/ticker/AAPL/prev?adjusted=true&apiKey=apiKey");
}
