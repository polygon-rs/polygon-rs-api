use crate::{
    data_types::Parse,
    rest::{
        error::ErrorCode,
        parameters::{Parameter, ParameterRequirment, Parameters, TickerTypes},
    },
    tools::{request::Request, verification::Verification},
};
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

impl DailyRequest for Daily {}

impl Parse for Daily {
    fn parse(map: &serde_json::Map<String, serde_json::Value>) -> Self {
        let after_hours = Self::f64_parse(map, vec!["afterHours"]);
        let close = Self::f64_parse(map, vec!["close"]);
        let from = Self::string_parse(map, vec!["from"]);
        let high = Self::f64_parse(map, vec!["high"]);
        let low = Self::f64_parse(map, vec!["low"]);
        let open = Self::f64_parse(map, vec!["open"]);
        let pre_market = Self::f64_parse(map, vec!["preMarket"]);
        let status = Self::string_parse(map, vec!["status"]);
        let symbol = Self::string_parse(map, vec!["symbol"]);
        let volume = Self::f64_parse(map, vec!["volume"]);
        Daily {
            after_hours,
            close,
            from,
            high,
            low,
            open,
            pre_market,
            status,
            symbol,
            volume,
        }
    }
}

pub trait DailyRequest {
    fn get_daily(
        &mut self,
        api_key: String,
        ticker: String,
        date: String,
        adjusted: Option<bool>,
        request: &impl Request,
        verification: &impl Verification,
    ) -> Result<Daily, ErrorCode> {
        let daily_parameters = Parameters {
            api_key: api_key,
            ticker: Some(ticker),
            date: Some(date),
            adjusted: adjusted,
            ..Parameters::default()
        };
        if let Err(check) = verification.check_parameters(
            &TickerTypes::set(true, true, false, false, true),
            PARAMETERS,
            &daily_parameters,
        ) {
            return Err(check);
        }
        let url = url(&daily_parameters);
        match request.request(url) {
            Ok(mut map) => Ok(Daily::parse(&mut map)),
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
        parameter: Parameter::Date,
    },
    &ParameterRequirment {
        required: false,
        parameter: Parameter::Adjusted,
    },
];
fn url(parameters: &Parameters) -> String {
    String::from(format!(
        "https://api.polygon.io/v1/open-close/{}/{}?{}apiKey={}",
        parameters.ticker.clone().unwrap(),
        parameters.date.clone().unwrap(),
        if let Some(adj) = parameters.adjusted {
            format!("adjusted={}&", adj)
        } else {
            "".to_string()
        },
        parameters.api_key,
    ))
}
