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
    fn parse(map: &serde_json::Map<String, serde_json::Value>) -> Self {
        let to = Self::string_parse(map, vec!["to"]);
        let from = Self::string_parse(map, vec!["from"]);
        let request_id = Self::string_parse(map, vec!["request_id"]);
        let status = Self::string_parse(map, vec!["status"]);
        let quote = Self::object_parse(map, vec!["last"]);
        let symbol = Self::string_parse(map, vec!["symbol"]);
        let initial_amount = Self::f64_parse(map, vec!["initial_amount"]);
        let converted = Self::f64_parse(map, vec!["converted"]);

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
        &self,
        api_key: String,
        ticker: String,
        amount: Option<f64>,
        precision: Option<u8>,
    ) -> Result<CurrencyConversion, ErrorCode> {
        let currency_conversion_parameters = Parameters {
            api_key: api_key,
            ticker: Some(ticker),
            amount: amount,
            precision: precision,
            ..Parameters::default()
        };
        if let Err(check) = Verification::check_parameters(
            &TickerTypes::forex(),
            PARAMETERS,
            &currency_conversion_parameters,
        ) {
            return Err(check);
        }
        let url = match url(&currency_conversion_parameters){
            Ok(url) => url,
            Err(e) => return Err(e),
        };
        match Request::request(url) {
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

fn url(parameters: &Parameters) -> Result<String, ErrorCode> {
    let from = match &parameters.ticker {
        Some(ticker) => ticker[2..4].to_string(),
        None => return Err(ErrorCode::TickerNotSet),
    };
    let to = match &parameters.ticker {
        Some(ticker) => ticker[5..7].to_string(),
        None => return Err(ErrorCode::TickerNotSet),
    };
    let url = String::from(format!(
        "https://api.polygon.io/v1/conversion/{}/{}?{}{}apiKey={}",
        from,
        to,
        if let Some(s) = &parameters.amount {
            format!("amount={}&", s)
        } else {
            "".to_string()
        },
        if let Some(s) = &parameters.precision {
            format!("precision={}&", s)
        } else {
            "".to_string()
        },
        &parameters.api_key,
    ));
    Ok(url)
}
