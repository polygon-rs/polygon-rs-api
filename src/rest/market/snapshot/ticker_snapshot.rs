use crate::data_types::{ticker::Ticker, Parse};
use crate::rest::{
    parameters::{Parameter, ParameterRequirment, Parameters, TickerType, TickerTypes},
    error::ErrorCode,
};
use crate::tools::{request::Request, verification::Verification};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TickerSnapshot {
    pub status: Option<String>,
    pub ticker: Option<Ticker>,
    pub request_id: Option<String>,
}

impl TickerSnapshotRequest for TickerSnapshot {}

impl Parse for TickerSnapshot {
    fn parse(map: &mut serde_json::Map<String, serde_json::Value>) -> Self {
        let status = map
            .get("status")
            .and_then(|v| v.as_str())
            .map(|v| v.to_string());
        let request_id = map
            .get("request_id")
            .and_then(|v| v.as_str())
            .map(|v| v.to_string());
        let ticker = map
            .get_mut("ticker")
            .and_then(|v| v.as_object_mut())
            .map(|v| Ticker::parse(v));
        TickerSnapshot {
            status: status,
            request_id: request_id,
            ticker,
        }
    }
}

pub trait TickerSnapshotRequest {
    fn get_ticker_snapshot(
        api_key: String,
        ticker: String,
        ticker_type: TickerType,
        request: &impl Request,
        verification: &impl Verification,
    ) -> Result<TickerSnapshot, ErrorCode> {
        let ticker_snapshot_parameters = Parameters {
            api_key: api_key,
            ticker: Some(ticker),
            ..Parameters::default()
        };
        if let Err(check) = verification.check_parameters(
            &TickerTypes::set(true, false, false, true, true),
            PARAMETERS,
            &ticker_snapshot_parameters,
        ) {
            return Err(check);
        }
        let locale = match ticker_type {
            TickerType::Stocks => String::from("us"),
            TickerType::Forex | TickerType::Crypto => String::from("global"),
            _ => return Err(ErrorCode::TickerTypeeNotValidForAPICall),
        };
        let url = url(&ticker_snapshot_parameters, locale, ticker_type);
        match request.request(url) {
            Ok(mut map) => Ok(TickerSnapshot::parse(&mut map)),
            Err(e) => return Err(e),
        }
    }
}

const PARAMETERS: &'static [&'static ParameterRequirment] = &[&ParameterRequirment {
    required: true,
    parameter: Parameter::Ticker,
}];

fn url(parameters: &Parameters, locale: String, ticker_type: TickerType) -> String {
    String::from(format!(
        "https://api.polygon.io/v2/snapshot/locale/{}/markets/{}/tickers/{}?apiKey={}",
        locale,
        ticker_type.to_string().to_lowercase(),
        parameters.ticker.clone().unwrap(),
        parameters.api_key,
    ))
}
