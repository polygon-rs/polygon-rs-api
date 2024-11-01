use crate::data_types::{ticker::Ticker, Parse};
use crate::rest::{
    error::ErrorCode,
    parameters::{Parameter, ParameterRequirment, Parameters, TickerType, TickerTypes},
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
    fn parse(map: &serde_json::Map<String, serde_json::Value>) -> Self {
        let status = Self::string_parse(map, vec!["status"]);
        let request_id = Self::string_parse(map, vec!["request_id"]);
        let ticker = Self::object_parse(map, vec!["ticker"]);

        TickerSnapshot {
            status,
            request_id,
            ticker,
        }
    }
}

pub trait TickerSnapshotRequest {
    fn get_ticker_snapshot(
        
        api_key: &String,
        ticker: String,
        ticker_type: TickerType,
    ) -> Result<TickerSnapshot, ErrorCode> {
        let ticker_snapshot_parameters = Parameters {
            api_key: api_key.to_string(),
            ticker: Some(ticker),
            ..Parameters::default()
        };
        if let Err(check) = Verification::check_parameters(
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
        let url = match url(&ticker_snapshot_parameters, locale, ticker_type){
            Ok(url) => url,
            Err(e) => return Err(e)
        };
        match Request::request(url) {
            Ok(mut map) => Ok(TickerSnapshot::parse(&mut map)),
            Err(e) => return Err(e),
        }
    }
}

const PARAMETERS: &'static [&'static ParameterRequirment] = &[&ParameterRequirment {
    required: true,
    parameter: Parameter::Ticker,
}];

fn url(parameters: &Parameters, locale: String, ticker_type: TickerType) -> Result<String, ErrorCode> {
    let url = String::from(format!(
        "https://api.polygon.io/v2/snapshot/locale/{}/markets/{}/tickers/{}?apiKey={}",
        locale,
        ticker_type.to_string().to_lowercase(),
        match &parameters.ticker{
            Some(ticker) => ticker,
            None => return Err(ErrorCode::TickerNotSet)
        },
        &parameters.api_key,
    ));
    Ok(url)
}
