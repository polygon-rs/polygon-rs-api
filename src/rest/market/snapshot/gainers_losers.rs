use crate::data_types::{ticker::Ticker, Parse};
use crate::rest::{
    parameters::{Direction,Parameter, ParameterRequirment, Parameters, TickerType, TickerTypes},
    error::ErrorCode,
};
use crate::tools::{request::Request, verification::Verification};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GainersLosers {
    pub status: Option<String>,
    pub tickers: Option<Vec<Ticker>>,
}

impl GainersLosersRequest for GainersLosers {}

impl Parse for GainersLosers {
    fn parse(map: &mut serde_json::Map<String, serde_json::Value>) -> Self {
        let status = map
            .get("status")
            .and_then(|v| v.as_str())
            .map(|v| v.to_string());
        let tickers = map
            .get_mut("tickers")
            .and_then(|v| v.as_array()).map(|v| {
                let mut tickers = Vec::new();
                for ticker in v {
                    if let Some(t) = ticker.clone().as_object_mut().map(|v| Ticker::parse(v)) {
                        tickers.push(t);
                    }
            }
            tickers
        });
        GainersLosers {
            status,
            tickers,
        }
    }
}

pub trait GainersLosersRequest {
    fn get_gainers_losers(
        api_key: String,
        direction: Direction,
        include_otc: Option<bool>,
        ticker_type: TickerType,
        request: &impl Request,
        verification: &impl Verification,
    ) -> Result<GainersLosers, ErrorCode> {
        let ticker_types = match ticker_type {
            TickerType::Stocks => TickerTypes::stocks(),
            TickerType::Forex => TickerTypes::forex(),
            TickerType::Crypto => TickerTypes::crypto(),
            _ => return Err(ErrorCode::TickerTypeeNotValidForAPICall),
        };
        let includeotc = match ticker_type {
            TickerType::Forex | TickerType::Crypto => None,
            _ => include_otc,
        };
        let gainers_losers_parameters = Parameters {
            api_key: api_key,
            direction: Some(direction),
            include_otc: includeotc,
            ..Parameters::default()
        };
        if let Err(check) = verification.check_parameters(
            &ticker_types,
            PARAMETERS,
            &gainers_losers_parameters,
        ) {
            return Err(check);
        }
        let locale = match ticker_type {
            TickerType::Stocks => String::from("us"),
            TickerType::Forex | TickerType::Crypto => String::from("global"),
            _ => return Err(ErrorCode::TickerTypeeNotValidForAPICall),
        };
        let url = url(&gainers_losers_parameters, locale, ticker_type);
        match request.request(url) {
            Ok(mut map) => Ok(GainersLosers::parse(&mut map)),
            Err(e) => return Err(e),
        }
    }
}

const PARAMETERS: &'static [&'static ParameterRequirment] = &[
        &ParameterRequirment {
            required: true,
            parameter: Parameter::Direction,
        },
        &ParameterRequirment {
            required: false,
            parameter: Parameter::IncludeOTC,
        },
    ];

fn url(parameters: &Parameters, locale: String, ticker_type: TickerType) -> String {
    String::from(format!(
        "https://api.polygon.io/v2/snapshot/locale/{}/markets/{}/{}?{}apiKey={}",
        locale,
        ticker_type.to_string().to_lowercase(),
        if let Some(s) = parameters.clone().direction {
            format!("direction={}&", s.to_string().to_lowercase())
        } else {
            "".to_string()
        },
        if let Some(s) = parameters.include_otc {
            format!("include_otc={}&", s)
        } else {
            "".to_string()
        },
        parameters.api_key,
    ))
}
