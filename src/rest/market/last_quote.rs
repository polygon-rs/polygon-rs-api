use crate::{ErrorCode, Parameter, ParameterRequirment, Parameters, Request};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct LastQuote {
    last_quote_parameters: Parameters,
    last_quote_url: String,
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
        self.last_quote_parameters = Parameters {
            api_key: api_key,
            ticker: Some(ticker),
            ..Parameters::default()
        }
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
        &self.last_quote_parameters
        /*match &self.last_quote_parameters {
            Some(p) => p,
            None => panic!("There is no parameters set"),
        }*/
    }

    fn url(&mut self) -> &String {
        &self.last_quote_url
        /*match &self.last_quote_url {
            Some(u) => u.to_string(),
            None => panic!("There is no url set"),
        }*/
    }

    fn set_url(&mut self) -> Result<(), ErrorCode> {
        if let Err(check) = self.check_parameters() { return Err(check)}
        self.last_quote_url = String::from(format!(
            "{}/{}/{}/{}apiKey={}",
            Self::BASE_URL,
            Self::VERSION,
            Self::CALL,
            self.parameters().clone().ticker.unwrap(),
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
        let a: LastQuote = match serde_json::from_str(r.as_str()) {
            Ok(it) => it,
            Err(err) => return Err(ErrorCode::FormatError),
        };
        *self = a;

        Ok(())
    }
}
