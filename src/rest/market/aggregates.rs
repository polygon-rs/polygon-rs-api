use crate::{
    data_types::{bar::Bar, Parse},
    rest::{
        error::ErrorCode,
        parameters::{Parameter, ParameterRequirment, Parameters, Sort, TickerTypes, Timespan},
    },
    tools::{request::Request, verification::Verification},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Aggregates {
    pub adjusted: Option<bool>,
    pub next_url: Option<String>,
    pub request_id: Option<String>,
    pub results: Option<Vec<Bar>>,
    pub status: Option<String>,
    pub results_count: Option<i64>,
    pub ticker: Option<String>,
    pub query_count: Option<i64>,
}

impl AggregatesRequest for Aggregates {}

impl Parse for Aggregates {
    fn parse(map: &serde_json::Map<String, serde_json::Value>) -> Self {
        let adjusted = Self::bool_parse(map, vec!["adjusted"]);
        let next_url = Self::string_parse(map, vec!["next_url"]);
        let request_id = Self::string_parse(map, vec!["request_id"]);
        let status = Self::string_parse(map, vec!["status"]);
        let results = Self::array_parse(map, vec!["results"]);
        let results_count = Self::i64_parse(map, vec!["resultsCount"]);
        let ticker = Self::string_parse(map, vec!["ticker"]);
        let query_count = Self::i64_parse(map, vec!["queryCount"]);
        Aggregates {
            adjusted,
            next_url,
            request_id,
            results,
            status,
            results_count,
            ticker,
            query_count,
        }
    }
}

pub trait AggregatesRequest {
    fn get_aggregates(
        api_key: &String,
        ticker: String,
        multiplier: u16,
        timespan: Timespan,
        from: String,
        to: String,
        sort: Option<Sort>,
        limit: Option<u16>,
        adjusted: Option<bool>,
    ) -> Result<Aggregates, ErrorCode> {
        let aggregates_parameters = Parameters {
            api_key: api_key.to_string(),
            ticker: Some(ticker),
            adjusted: adjusted,
            multiplier: Some(multiplier),
            timespan: Some(timespan),
            from: Some(from),
            to: Some(to),
            sort: sort,
            limit: limit,
            ..Parameters::default()
        };
        if let Err(check) =
            Verification::check_parameters(&TickerTypes::all(), PARAMETERS, &aggregates_parameters)
        {
            return Err(check);
        }
        let url = match url(&aggregates_parameters) {
            Ok(url) => url,
            Err(e) => return Err(e),
        };
        match Request::request(url) {
            Ok(mut map) => Ok(Aggregates::parse(&mut map)),
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
        required: true,
        parameter: Parameter::Multiplier,
    },
    &ParameterRequirment {
        required: true,
        parameter: Parameter::Timespan,
    },
    &ParameterRequirment {
        required: true,
        parameter: Parameter::From,
    },
    &ParameterRequirment {
        required: true,
        parameter: Parameter::To,
    },
    &ParameterRequirment {
        required: false,
        parameter: Parameter::Adjusted,
    },
    &ParameterRequirment {
        required: false,
        parameter: Parameter::Sort,
    },
    &ParameterRequirment {
        required: false,
        parameter: Parameter::Limit,
    },
];

fn url(parameters: &Parameters) -> Result<String, ErrorCode> {
    let url = String::from(format!(
        "https://api.polygon.io/v2/aggs/ticker/{}/range/{}/{}/{}/{}?{}{}{}apiKey={}",
        match &parameters.ticker {
            Some(ticker) => ticker,
            None => return Err(ErrorCode::TickerNotSet),
        },
        match &parameters.multiplier {
            Some(multiplier) => multiplier,
            None => return Err(ErrorCode::MultiplierNotSet),
        },
        match &parameters.timespan {
            Some(timespan) => timespan,
            None => return Err(ErrorCode::TimespanNotSet),
        },
        match &parameters.from {
            Some(from) => from,
            None => return Err(ErrorCode::FromNotSet),
        },
        match &parameters.to {
            Some(to) => to,
            None => return Err(ErrorCode::ToNotSet),
        },
        if let Some(adj) = &parameters.adjusted {
            format!("adjusted={}&", adj)
        } else {
            "".to_string()
        },
        if let Some(s) = &parameters.sort {
            format!("sort={}&", s)
        } else {
            "".to_string()
        },
        if let Some(l) = &parameters.limit {
            format!("limit={}&", l)
        } else {
            "".to_string()
        },
        &parameters.api_key,
    ));
    Ok(url)
}
