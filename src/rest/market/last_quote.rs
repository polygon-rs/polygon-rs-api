use crate::{ErrorCode, Parameter, ParameterRequirment, Parameters, Request, Sort, Timespan};
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct LastQuote {
    last_quote_parameters: Option<Parameters>,
    last_quote_url: Option<String>,
    pub request_id: String,
    pub results: Trade,
    pub status: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Trade {
    pub P: f64,
    pub S: i32,
    pub T: String,
    pub X: i32,
    pub p: f64,
    pub q: i32,
    pub s: i32,
    pub t: i64,
    pub x: i32,
    pub y: i64,
    pub z: i32
}

impl LastQuote {
    pub fn set_parameters(&mut self, api_key: String, ticker: String) {
        self.last_quote_parameters = Some(Parameters {
            api_key: api_key,
            ticker: Some(ticker),
            ..Parameters::default()
        })
    }
}

impl Request for LastQuote {
    const VERSION: &'static str = "v2";
    const CALL: &'static str = "last/nbbo";
    const PARAMETERS: &'static [&'static ParameterRequirment] = &[
        &ParameterRequirment {
            required: true,
            parameter: Parameter::Ticker,
        },
    ];

    fn parameters(&self) -> &Parameters {
        match &self.last_quote_parameters {
            Some(p) => p,
            None => panic!("There is no parameters set"),
        }
    }

    fn url(&mut self) -> String {
        self.set_url();
        match &self.last_quote_url {
            Some(u) => u.to_string(),
            None => panic!("There is no url set"),
        }
    }

    fn set_url(&mut self) {
        self.check_parameters();
        self.last_quote_url = Some(String::from(format!(
            "{}/{}/{}/{}apiKey={}",
            Self::BASE_URL,
            Self::VERSION,
            Self::CALL,
            self.parameters().clone().ticker.unwrap(),
            self.parameters().clone().api_key,
        )));
    }

    fn request(&mut self) -> Result<(), ErrorCode> {
        let r = match self.get_raw_data() {
            Ok(response) => response,
            Err(e) => return Err(e),
        };
        let a: LastQuote = match serde_json::from_str(r.as_str()) {
            Ok(it) => it,
            Err(err) => return Err(ErrorCode::FormatError),
        };
        *self = a;

        Ok(())
    }
}
