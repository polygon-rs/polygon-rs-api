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
pub struct GroupedBars {
    pub adjusted: Option<bool>,
    pub bars: Option<Vec<Bar>>,
    pub status: Option<String>,
    pub results_count: Option<i64>,
    pub query_count: Option<i64>,
}

impl GroupedBarsRequest for GroupedBars {}

impl Parse for GroupedBars {
    fn parse(map: &serde_json::Map<String, serde_json::Value>) -> Self {
        let adjusted = Self::bool_parse(map, vec!["adjusted"]);
        let bars = Self::array_parse(map, vec!["bars"]);
        let status = Self::string_parse(map, vec!["status"]);
        let results_count = Self::i64_parse(map, vec!["resultsCount"]);
        let query_count = Self::i64_parse(map, vec!["queryCount"]);

        GroupedBars {
            adjusted,
            bars,
            status,
            results_count,
            query_count,
        }
    }
}

pub trait GroupedBarsRequest {
    fn get_grouped_bars(
        api_key: &String,
        date: String,
        include_otc: Option<bool>,
        adjusted: Option<bool>,
    ) -> Result<GroupedBars, ErrorCode> {
        let grouped_bars_parameters = Parameters {
            api_key: api_key.to_string(),
            date: Some(date),
            adjusted: adjusted,
            include_otc: include_otc,
            ..Parameters::default()
        };
        if let Err(check) = Verification::check_parameters(
            &TickerTypes::set(true, false, false, true, true),
            PARAMETERS,
            &grouped_bars_parameters,
        ) {
            return Err(check);
        }
        let url = match url(&grouped_bars_parameters) {
            Ok(url) => url,
            Err(e) => return Err(e),
        };
        match Request::request(url) {
            Ok(mut map) => Ok(GroupedBars::parse(&mut map)),
            Err(e) => return Err(e),
        }
    }
}

const PARAMETERS: &'static [&'static ParameterRequirment] = &[
    &ParameterRequirment {
        required: true,
        parameter: Parameter::Date,
    },
    &ParameterRequirment {
        required: false,
        parameter: Parameter::Adjusted,
    },
    &ParameterRequirment {
        required: false,
        parameter: Parameter::IncludeOTC,
    },
];

fn url(parameters: &Parameters) -> Result<String, ErrorCode> {
    let url = String::from(format!(
        "https://api.polygon.io/v2/aggs/grouped/locale/us/market/stocks/{}?{}{}apiKey={}",
        match &parameters.date {
            Some(date) => date,
            None => return Err(ErrorCode::DateNotSet),
        },
        if let Some(adj) = &parameters.adjusted {
            format!("adjusted={}&", adj)
        } else {
            "".to_string()
        },
        if let Some(s) = &parameters.include_otc {
            format!("include_otc={}&", s)
        } else {
            "".to_string()
        },
        &parameters.api_key,
    ));
    Ok(url)
}
#[test]
fn test_grouped_bars_parse() {
    let data = serde_json::json!({
        "adjusted": true,
        "bars": [
            {
                "T": "test_exchange",
                "c": 1.23,
                "h": 2.34,
                "l": 0.12,
                "n": 123,
                "o": 0.12,
                "t": 164545545,
                "v": 456.78,
                "vw": 901.23,
                "otc": false
            }
        ],
        "status": "OK",
        "resultsCount": 1,
        "queryCount": 1
    });
    let grouped_bars = GroupedBars::parse(&data.as_object().unwrap());
    assert_eq!(grouped_bars.adjusted.unwrap(), true);
    assert_eq!(grouped_bars.bars.unwrap()[0].close.unwrap(), 1.23);
    assert_eq!(grouped_bars.status.unwrap(), "OK");
    assert_eq!(grouped_bars.results_count.unwrap(), 1);
    assert_eq!(grouped_bars.query_count.unwrap(), 1);
}

#[test]
fn test_url() {
    let mut parameters = Parameters::default();
    parameters.api_key = String::from("apiKey");
    parameters.date = Some(String::from("2023-04-01"));
    parameters.adjusted = Some(true);
    parameters.include_otc = Some(true);
    let url = url(&parameters).unwrap();
    assert_eq!(url, "https://api.polygon.io/v2/aggs/grouped/locale/us/market/stocks/2023-04-01?adjusted=true&include_otc=true&apiKey=apiKey");
}
