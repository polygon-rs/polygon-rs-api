use crate::{
    data_types::{trade::Trade, Parse},
    rest::{
        error::ErrorCode,
        parameters::{Parameter, ParameterRequirment, Parameters, TickerTypes},
    },
    tools::{request::Request, verification::Verification},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LastTrade {
    pub request_id: Option<String>,
    pub results: Option<Trade>,
    pub status: Option<String>,
}

impl LastTradeRequest for LastTrade {}

impl Parse for LastTrade {
    fn parse(map: &mut serde_json::Map<String, serde_json::Value>) -> Self {
        let request_id = map
            .get("request_id")
            .and_then(|v| v.as_str())
            .map(|v| v.to_string());
        let status = map
            .get("status")
            .and_then(|v| v.as_str())
            .map(|v| v.to_string());
        let results = map
            .get_mut("results")
            .and_then(|v| v.as_object_mut())
            .map(|v| Trade::parse(v));
        LastTrade{
            request_id,
            results,
            status,
        }
    }
}

pub trait LastTradeRequest {
    fn get_last_trade(
        api_key: String,
        ticker: String,
        request: &impl Request,
        verification: &impl Verification,
    ) -> Result<LastTrade, ErrorCode> {
        let last_trade_parameters = Parameters {
            api_key: api_key,
            ticker: Some(ticker),
            ..Parameters::default()
        };
        if let Err(check) = verification.check_parameters(
            &TickerTypes::set(true, true, false, false, false),
            PARAMETERS,
            &last_trade_parameters,
        ) {
            return Err(check);
        }
        let url = url(&last_trade_parameters);
        match request.request(url) {
            Ok(mut map) => Ok(LastTrade::parse(&mut map)),
            Err(e) => return Err(e),
        }
    }
}

const PARAMETERS: &'static [&'static ParameterRequirment] = &[&ParameterRequirment {
    required: true,
    parameter: Parameter::Ticker,
}];

fn url(parameters: &Parameters) -> String {
    String::from(format!(
        "https://api.polygon.io/v2/last/trade/{}apiKey={}",
        parameters.ticker.clone().unwrap(), parameters.api_key,
    ))
}
