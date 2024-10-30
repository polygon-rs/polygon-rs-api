use crate::{
    data_types::{bar::Bar, Parse},
    rest::{
        error::ErrorCode,
        parameters::{Parameter, ParameterRequirment, Parameters, Sort, TickerTypes, Timespan},
    },
    tools::{
        request::{Next, Request},
        verification::Verification,
    },
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
    fn parse(map: &mut serde_json::Map<String, serde_json::Value>) -> Self {
        let adjusted = map.get("adjusted").and_then(|v| v.as_bool()).map(|v| v);
        let next_url = map
            .get("next_url")
            .and_then(|v| v.as_str())
            .map(|v| v.to_string());
        let request_id = map
            .get("request_id")
            .and_then(|v| v.as_str())
            .map(|v| v.to_string());
        let results = map.get_mut("results").and_then(|v| v.as_array()).map(|v| {
            let mut results = Vec::new();
            for result in v {
                if let Some(t) = result.clone().as_object_mut().map(|v| Bar::parse(v)) {
                    results.push(t);
                }
            }
            results
        });
        let status = map
            .get("status")
            .and_then(|v| v.as_str())
            .map(|v| v.to_string());
        let results_count = map.get("resultsCount").and_then(|v| v.as_i64()).map(|v| v);
        let ticker = map
            .get("ticker")
            .and_then(|v| v.as_str())
            .map(|v| v.to_string());
        let query_count = map.get("queryCount").and_then(|v| v.as_i64()).map(|v| v);
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

impl Next for Aggregates {}

pub trait AggregatesRequest {
    fn get_aggregates(
        api_key: String,
        ticker: String,
        multiplier: u16,
        timespan: Timespan,
        from: String,
        to: String,
        sort: Option<Sort>,
        limit: Option<u16>,
        adjusted: Option<bool>,
        request: &impl Request,
        verification: &impl Verification,
    ) -> Result<Aggregates, ErrorCode> {
        let aggregates_parameters = Parameters {
            api_key: api_key,
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
            verification.check_parameters(&TickerTypes::all(), PARAMETERS, &aggregates_parameters)
        {
            return Err(check);
        }
        let url = url(&aggregates_parameters);
        match request.request(url) {
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

fn url(parameters: &Parameters) -> String {
    String::from(format!(
        "https://api.polygon.io/v2/aggs/ticker/{}/range/{}/{}/{}/{}?{}{}{}apiKey={}",
        parameters.ticker.clone().unwrap(),
        parameters.multiplier.clone().unwrap(),
        parameters.timespan.clone().unwrap(),
        parameters.from.clone().unwrap(),
        parameters.to.clone().unwrap(),
        if let Some(adj) = parameters.adjusted {
            format!("adjusted={}&", adj)
        } else {
            "".to_string()
        },
        if let Some(s) = parameters.clone().sort {
            format!("sort={}&", s)
        } else {
            "".to_string()
        },
        if let Some(l) = parameters.clone().limit {
            format!("limit={}&", l)
        } else {
            "".to_string()
        },
        parameters.api_key,
    ))
}
