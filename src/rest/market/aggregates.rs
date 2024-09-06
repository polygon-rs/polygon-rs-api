use crate::{Parameters, Sort, Timespan, Request, ParameterRequirment, ErrorCode, Parameter};
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Aggregates {
    aggregates_parameters: Option<Parameters>,
    aggregates_url: Option<String>,
    pub adjusted: bool,
    pub next_url: String,
    pub request_id: String,
    pub results: Vec<Bar>,
    pub status: String,
    pub resultsCount: i32,
    pub ticker: String,
    pub query_count: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Bar {
    pub c: f64,
    pub h: f64,
    pub l: f64,
    pub n: i32,
    pub o: f64,
    pub t: i64,
    pub v: f64,
    pub vw: f64,
}

impl Aggregates {
    pub fn set_parameters(
        &mut self,
        api_key: String,
        ticker: String,
        multiplier: u16,
        timespan: Timespan,
        from: String,
        to: String,
        sort: Option<Sort>,
        limit: Option<u16>,
        adjusted: Option<bool>,
    ) {
        self.aggregates_parameters = Some(Parameters {
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
        })
    }
}

impl Request for Aggregates {
    const VERSION: &'static str = "v2";
    const CALL: &'static str = "aggs";
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

    fn parameters(&self) -> &Parameters {
        match &self.aggregates_parameters {
            Some(p) => p,
            None => panic!("There is no parameters set"),
        }
    }

    fn url(&mut self) -> String {
        self.set_url();
        match &self.aggregates_url {
            Some(u) => u.to_string(),
            None => panic!("There is no url set"),
        }
    }

    fn set_url(&mut self) {
        self.check_parameters();
        self.aggregates_url = Some(String::from(format!(
            "{}/{}/{}/ticker/{}/range/{}/{}/{}/{}?{}{}{}apiKey={}",
            Self::BASE_URL,
            Self::VERSION,
            Self::CALL,
            self.parameters().clone().ticker.unwrap(),
            self.parameters().clone().multiplier.unwrap(),
            self.parameters().clone().timespan.unwrap(),
            self.parameters().clone().from.unwrap(),
            self.parameters().clone().to.unwrap(),
            if let Some(adj) = self.parameters().clone().adjusted {
                format!("adjusted={}&", adj)
            } else {
                "".to_string()
            },
            if let Some(s) = self.parameters().clone().sort {
                format!("sort={}&", s)
            } else {
                "".to_string()
            },
            if let Some(l) = self.parameters().clone().limit {
                format!("limit={}&", l)
            } else {
                "".to_string()
            },
            self.parameters().clone().api_key,
        )));
    }

    fn request(&mut self) -> Result<(), ErrorCode> {
        let r = match self.get_raw_data() {
            Ok(response) => response,
            Err(e) => return Err(e),
        };
        let a: Aggregates = match serde_json::from_str(r.as_str()) {
            Ok(it) => it,
            Err(err) => return Err(ErrorCode::FormatError),
        };
        *self = a;

        Ok(())
    }
}