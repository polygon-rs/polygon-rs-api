use std::f32::consts::E;

use crate::{rest::Rest, ErrorCode, Parameter, ParameterRequirment, Parameters, Request};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Daily {
    daily_parameters: Option<Parameters>,
    daily_url: Option<String>,
    pub adjusted: Option<bool>,
    pub after_hours: Option<f64>,
    pub close: Option<f64>,
    pub from: Option<String>,
    pub high: Option<f64>,
    pub low: Option<f64>,
    pub open: Option<f64>,
    pub pre_market: Option<f64>,
    pub status: Option<String>,
    pub symbol: Option<String>,
    pub volume: Option<f64>,
}

/*impl Daily {
    pub fn set_parameters(&mut self, api_key: String, ticker: String, date: String, adjusted: Option<bool>) {
        self.daily_parameters = Some(Parameters {
            api_key: api_key,
            ticker: Some(ticker),
            date: Some(date),
            adjusted: adjusted,
            ..Parameters::default()
        })
    }
}*/

impl Request for Daily {
    const PARAMETERS: &'static [&'static ParameterRequirment] = &[
        &ParameterRequirment {
            required: true,
            parameter: Parameter::Ticker,
        },
        &ParameterRequirment {
            required: true,
            parameter: Parameter::Date,
        },
        &ParameterRequirment {
            required: false,
            parameter: Parameter::Adjusted,
        },
    ];
    //const BASE_URL: &'static str = "https://api.polygon.io/v1/open-close/";
    //const VERSION: &'static str = "v1";


    fn parameters(&self) -> &Parameters {
        match &self.daily_parameters {
            Some(p) => p,
            None => panic!("There is no parameters set"),
        }
    }

    fn url(&self) -> String {
        match &self.daily_url {
            Some(u) => u,
            None => panic!("There is no url set"),
        }
    }

    fn set_parameters(&mut self, api_key: String, ticker: String, date: String, adjusted: Option<bool>) {
        self.daily_parameters = Some(Parameters {
            api_key: api_key,
            ticker: Some(ticker),
            date: Some(date),
            adjusted: adjusted,
            ..Parameters::default()
        })
    }

    fn set_url(&self) {
        self.check_parameters();
        self.daily_url = Some(String::from(""));
    }


    fn request(&mut self) -> Result<(), ErrorCode> {
        let r = match self.get_raw_data() {
            Ok(response) => response,
            Err(e) => return Err(e),
        };
        let d: Daily = match serde_json::from_str(r.as_str()) {
            Ok(it) => it,
            Err(err) => return Err(ErrorCode::FormatError),
        };
        *self = d;

        Ok(())
    }
}
