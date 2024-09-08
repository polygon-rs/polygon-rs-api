use crate::{ ErrorCode, Parameter, ParameterRequirment, Parameters, Request};
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Daily {
    #[serde(skip)]
    daily_parameters: Parameters,
    #[serde(skip)]
    daily_url: String,
    pub afterHours: f64,
    pub close: f64,
    pub from: String,
    pub high: f64,
    pub low: f64,
    pub open: f64,
    pub preMarket: f64,
    pub status: String,
    pub symbol: String,
    pub volume: f64,
}

impl Daily {
    pub fn set_parameters(
        &mut self,
        api_key: String,
        ticker: String,
        date: String,
        adjusted: Option<bool>,
    ) {
        self.daily_parameters = Parameters {
            api_key: api_key,
            ticker: Some(ticker),
            date: Some(date),
            adjusted: adjusted,
            ..Parameters::default()
        }
    }
}

impl Request for Daily {
    const VERSION: &'static str = "v1";
    const CALL: &'static str = "open-close";
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

    fn parameters(&self) -> &Parameters {
        &self.daily_parameters
        /*match &self.daily_parameters {
            Some(p) => p,
            None => panic!("There is no parameters set"), //Need to remove panic for error
        }*/
    }

    fn url(&mut self) -> &String {
        &self.daily_url
        /*self.set_url();
        match &self.daily_url {
            Some(u) => u.to_string(),
            None => panic!("There is no url set"), //Need to remove panic for error
        }*/
    }

    fn set_url(&mut self) -> Result<(), ErrorCode> {
        if let Err(check) = self.check_parameters() { return Err(check)}
        self.daily_url = String::from(format!(
            "{}/{}/{}/{}/{}?{}apiKey={}",
            Self::BASE_URL,
            Self::VERSION,
            Self::CALL,
            self.parameters().clone().ticker.unwrap(),
            self.parameters().clone().date.unwrap(),
            if let Some(adj) = self.parameters().clone().adjusted {
                format!("adjusted={}&", adj)
            } else {
                "".to_string()
            },
            self.parameters().clone().api_key,
        ));
        Ok(())
    }

    fn request(&mut self) -> Result<(), ErrorCode> {
        if let Err(check) = self.set_url() { return Err(check)}
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
