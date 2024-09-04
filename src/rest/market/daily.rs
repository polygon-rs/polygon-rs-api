use crate::{polygon::error::ErrorCode, rest::Rest, Get, Parameter, Parameters, Polygon};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Daily {
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

impl Get for Daily {
    const PARAMETERS: Vec<Parameters> = [
        Parameters {
            required: true,
            parameter: Parameter::Ticker,
        },
        Parameters {
            required: true,
            parameter: Parameter::Date,
        },
        Parameters {
            required: false,
            parameter: Parameter::Adjusted,
        },
    ]
    .to_vec();
    const BASE_URL: &'static str = "https://api.polygon.io/v1/open-close/";

    fn get(&self) -> Result<Rest, ErrorCode> {
        match self.request() {
            Ok(response) => match serde_json::from_str(response.as_str()) {
                Ok(daily) => Ok(daily),
                Err(e) => return Err(ErrorCode::FormatError),
            },
            Err(e) => return Err(ErrorCode::RequestError),
        }
    }
}

/*impl Daily {
    const PARAMETERS: [Parameters; 3] = [Parameters{ required: true, parameter: Parameter::Ticker},Parameters{ required: true, parameter: Parameter::Date},Parameters{ required: false, parameter: Parameter::Adjusted}];
    const BASE_URL: &'static str = "https://api.polygon.io/v1/open-close/";

    #[tokio::main]
    pub async fn get(&self, p: Polygon) -> Result<Daily, ErrorCode> {
        /*match p.verify_ticker() {
            Ok(t) => (),
            Err(e) => return Err(ErrorCode::TickerError),
        };
        match p.verify_date() {
            Ok(d) => (),
            Err(e) => return Err(ErrorCode::DateError),
        };
        let mut url_options = String::from("");
        match p.adjusted {
            Some(a) => match a {
                true => url_options = format!("{}?adjusted=true&", url_options),
                false => url_options = format!("{}?adjusted=false&", url_options),
            },
            None => (),
        }
        let url = format!(
            "https://api.polygon.io/v1/open-close/{}/{}{}?apiKey={}",
            p.clone().ticker.unwrap(),
            url_options,
            p.clone().date.unwrap(),
            p.api_key
        );*/
        match self.request() {
            Ok(response) => match serde_json::from_str(response.as_str()) {
                Ok(daily) => Ok(daily),
                Err(e) => return Err(ErrorCode::FormatError),
            },
            Err(e) => return Err(ErrorCode::RequestError),
        }
    }
}*/
