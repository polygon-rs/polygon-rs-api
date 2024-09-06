use crate::{ErrorCode, Parameter, ParameterRequirment, Parameters, Request, Sort, Timespan};
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Previous {
    previous_parameters: Option<Parameters>,
    previous_url: Option<String>,
    pub adjusted: bool,
    pub query_count: i32,
    pub request_id: String,
    pub results: Vec<Bar>,
    pub resultsCount: i32,
    pub status: String,
    pub ticker: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Bar {
    pub T: String,
    pub c: f64,
    pub h: f64,
    pub l: f64,
    pub o: f64,
    pub t: i64,
    pub v: f64,
    pub vw: f64,
}

impl Previous {
    pub fn set_parameters(&mut self, api_key: String, ticker: String, adjusted: Option<bool>) {
        self.previous_parameters = Some(Parameters {
            api_key: api_key,
            ticker: Some(ticker),
            adjusted: adjusted,
            ..Parameters::default()
        })
    }
}

impl Request for Previous {
    const VERSION: &'static str = "v2";
    const CALL: &'static str = "aggs";
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

    fn parameters(&self) -> &Parameters {
        match &self.previous_parameters {
            Some(p) => p,
            None => panic!("There is no parameters set"),
        }
    }

    fn url(&mut self) -> String {
        self.set_url();
        match &self.previous_url {
            Some(u) => u.to_string(),
            None => panic!("There is no url set"),
        }
    }

    fn set_url(&mut self) {
        self.check_parameters();
        self.previous_url = Some(String::from(format!(
            "{}/{}/{}/ticker/{}/prev?{}apiKey={}",
            Self::BASE_URL,
            Self::VERSION,
            Self::CALL,
            self.parameters().clone().ticker.unwrap(),
            if let Some(adj) = self.parameters().clone().adjusted {
                format!("adjusted={}&", adj)
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
        let a: Previous = match serde_json::from_str(r.as_str()) {
            Ok(it) => it,
            Err(err) => return Err(ErrorCode::FormatError),
        };
        *self = a;

        Ok(())
    }
}
