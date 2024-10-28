use crate::{
    data_types::{quote::Quote, Parse},
    rest::{
        error::ErrorCode,
        parameters::{Parameter, ParameterRequirment, Parameters, TickerTypes},
    },
    tools::{request::Request, verification::Verification},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CurrencyConversion {
    pub to: Option<String>,
    pub from: Option<String>,
    pub request_id: Option<String>,
    pub quote: Option<Quote>,
    pub status: Option<String>,
    pub symbol: Option<String>,
    pub initial_amount: Option<f64>,
    pub converted: Option<f64>,
}

impl Parse for CurrencyConversion {
    fn parse(map: &mut serde_json::Map<String, serde_json::Value>) -> Self {
        let to = map
            .get("to")
            .and_then(|v| v.as_str().map(|s| s.to_string()));
        let from = map
            .get("from")
            .and_then(|v| v.as_str().map(|s| s.to_string()));
        let request_id = map
            .get("request_id")
            .and_then(|v| v.as_str().map(|s| s.to_string()));
        let status = map
            .get("status")
            .and_then(|v| v.as_str().map(|s| s.to_string()));
        let symbol = map
            .get("symbol")
            .and_then(|v| v.as_str().map(|s| s.to_string()));
        let initial_amount = map.get("initial_amount").and_then(|v| v.as_f64());
        let converted = map.get("converted").and_then(|v| v.as_f64());
        let quote = map
            .get_mut("last")
            .and_then(|v| v.as_object_mut())
            .map(|v| Quote::parse(v));
        CurrencyConversion {
            to,
            from,
            request_id,
            quote,
            status,
            symbol,
            initial_amount,
            converted,
        }
    }
}

pub trait CurrencyConversionRequest {
    fn get_currency_conversion(
        api_key: String,
        ticker: String,
        amount: Option<f64>,
        precision: Option<u8>,
        request: &impl Request,
        verification: &impl Verification,
    ) -> Result<CurrencyConversion, ErrorCode> {
        let currency_conversion_parameters = Parameters {
            api_key: api_key,
            ticker: Some(ticker),
            amount: amount,
            precision: precision,
            ..Parameters::default()
        };
        if let Err(check) = verification.check_parameters(
            &TickerTypes::forex(),
            PARAMETERS,
            &currency_conversion_parameters,
        ) {
            return Err(check);
        }
        let url = url(&currency_conversion_parameters);
        match request.request(url) {
            Ok(mut map) => Ok(CurrencyConversion::parse(&mut map)),
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
        required: false,
        parameter: Parameter::Amount,
    },
    &ParameterRequirment {
        required: false,
        parameter: Parameter::Precision,
    },
];

fn url(parameters: &Parameters) -> String {
    let from = parameters.ticker.clone().unwrap()[2..4].to_string();
    let to = parameters.ticker.clone().unwrap()[5..7].to_string();
    String::from(format!(
        "https://api.polygon.io/v1/conversion/{}/{}?{}{}apiKey={}",
        from,
        to,
        if let Some(s) = parameters.amount {
            format!("amount={}&", s)
        } else {
            "".to_string()
        },
        if let Some(s) = parameters.precision {
            format!("precision={}&", s)
        } else {
            "".to_string()
        },
        parameters.api_key,
    ))
}
