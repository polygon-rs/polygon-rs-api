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
        &mut self,
        api_key: String,
        ticker: String,
        adjusted: Option<bool>,
        request: &impl Request,
        verification: &impl Verification,
    ) -> Result<Previous, ErrorCode> {
        let previous_parameters = Parameters {
            api_key: api_key,
            ticker: Some(ticker),
            adjusted: adjusted,
            ..Parameters::default()
        };
        if let Err(check) =
            verification.check_parameters(&TickerTypes::all(), PARAMETERS, &previous_parameters)
        {
            return Err(check);
        }
        let url = url(&previous_parameters);
        match request.request(url) {
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

fn url(parameters: &Parameters) -> String {
    String::from(format!(
        "https://api.polygon.io/v2/aggs/ticker/{}/prev?{}apiKey={}",
        parameters.ticker.clone().unwrap(),
        if let Some(adj) = parameters.adjusted {
            format!("adjusted={}&", adj)
        } else {
            "".to_string()
        },
        parameters.api_key,
    ))
}
