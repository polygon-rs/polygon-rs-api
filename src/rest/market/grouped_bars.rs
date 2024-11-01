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
        &self,
        api_key: String,
        date: String,
        include_otc: Option<bool>,
        adjusted: Option<bool>,
    ) -> Result<GroupedBars, ErrorCode> {
        let grouped_bars_parameters = Parameters {
            api_key: api_key,
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
        let url = match url(&grouped_bars_parameters){
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
