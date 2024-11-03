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
        api_key: &String,
        ticker: String,
        date: String,
        adjusted: Option<bool>,
    ) -> Result<Daily, ErrorCode> {
        let daily_parameters = Parameters {
            api_key: api_key.to_string(),
            ticker: Some(ticker),
            date: Some(date),
            adjusted: adjusted,
            ..Parameters::default()
        };
        if let Err(check) = Verification::check_parameters(
            &TickerTypes::set(true, true, false, false, true),
            PARAMETERS,
            &daily_parameters,
        ) {
            return Err(check);
        }
        let url = match url(&daily_parameters) {
            Ok(url) => url,
            Err(e) => return Err(e),
        };
        match Request::request(url) {
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
fn url(parameters: &Parameters) -> Result<String, ErrorCode> {
    let url = String::from(format!(
        "https://api.polygon.io/v1/open-close/{}/{}?{}apiKey={}",
        match &parameters.ticker {
            Some(ticker) => ticker,
            None => return Err(ErrorCode::TickerNotSet),
        },
        match &parameters.date {
            Some(date) => date,
            None => return Err(ErrorCode::DateNotSet),
        },
        if let Some(adj) = &parameters.adjusted {
            format!("adjusted={}&", adj)
        } else {
            "".to_string()
        },
        &parameters.api_key,
    ));
    Ok(url)
}
#[test]
fn test_daily_parse() {
    let data = serde_json::json!({
        "afterHours": 1.23,
        "close": 2.34,
        "from": "2023-04-01",
        "high": 3.45,
        "low": 0.12,
        "open": 0.12,
        "preMarket": 4.56,
        "status": "OK",
        "symbol": "AAPL",
        "volume": 123456
    });
    let daily = Daily::parse(&data.as_object().unwrap());
    assert_eq!(daily.after_hours.unwrap(), 1.23);
    assert_eq!(daily.close.unwrap(), 2.34);
    assert_eq!(daily.from.unwrap(), "2023-04-01");
    assert_eq!(daily.high.unwrap(), 3.45);
    assert_eq!(daily.low.unwrap(), 0.12);
    assert_eq!(daily.open.unwrap(), 0.12);
    assert_eq!(daily.pre_market.unwrap(), 4.56);
    assert_eq!(daily.status.unwrap(), "OK");
    assert_eq!(daily.symbol.unwrap(), "AAPL");
    assert_eq!(daily.volume.unwrap(), 123456.0);
}

#[test]
fn test_url() {
    let mut parameters = Parameters::default();
    parameters.api_key = String::from("apiKey");
    parameters.ticker = Some(String::from("AAPL"));
    parameters.date = Some(String::from("2023-04-01"));
    parameters.adjusted = Some(true);
    let url = url(&parameters).unwrap();
    assert_eq!(url, "https://api.polygon.io/v1/open-close/AAPL/2023-04-01?adjusted=true&apiKey=apiKey");
}
