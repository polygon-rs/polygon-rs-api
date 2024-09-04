use crate::{rest::Rest, ErrorCode, Parameter, ParameterRequirment, Parameters, Request};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Daily {
    daily_parameters: Option<Parameters>,
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

impl Daily {
    pub fn set_parameters(mut self, api_key: String, ticker: String, date: String, adjusted: bool) {
        self.daily_parameters = Some(Parameters {
            api_key: api_key,
            ticker: Some(ticker),
            date: Some(date),
            adjusted: Some(adjusted),
            ..Parameters::default()
        })
    }
}

/*impl Default for Daily {
    fn default() -> Daily {
        Daily {
            daily_parameters: None,
            adjusted: None,
            after_hours: None,
            close: None,
            from: None,
            high: None,
            low: None,
            open: None,
            pre_market: None,
            status: None,
            symbol: None,
            volume: None
        }
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
    const BASE_URL: &'static str = "https://api.polygon.io/v1/open-close/";

    fn parameters(&self) -> &Parameters {
        match &self.daily_parameters {
            Some(p) => p,
            None => panic!("There is no parameters set"),
        }
    }

    fn request(&self) -> Result<Rest, ErrorCode> {
        match self.get_raw_data() {
            Ok(response) => match serde_json::from_str(response.as_str()) {
                Ok(daily) => Ok(daily),
                Err(e) => return Err(ErrorCode::FormatError),
            },
            Err(e) => return Err(ErrorCode::RequestError),
        }
    }
}
