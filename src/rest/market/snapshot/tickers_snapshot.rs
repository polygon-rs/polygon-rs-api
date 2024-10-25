use crate::data_types::{ticker::Ticker, Parse};
use crate::rest::{
    parameters::{Parameter, ParameterRequirment, Parameters, TickerType, TickerTypes},
    error::ErrorCode,
};
use crate::tools::{request::Request, verification::Verification};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TickersSnapshot {
    pub status: Option<String>,
    pub tickers: Option<Vec<Ticker>>,
    pub count: Option<i64>,
}

impl TickersSnapshotRequest for TickersSnapshot {}

impl Parse for TickersSnapshot {
    fn parse(map: &mut serde_json::Map<String, serde_json::Value>) -> Self {
        let status = map
            .get("status")
            .and_then(|v| v.as_str())
            .map(|v| v.to_string());
        let count = map
            .get("count")
            .and_then(|v| v.as_i64());
        let tickers = map
            .get_mut("tickers")
            .and_then(|v| v.as_array()).map(|v| {
                let mut tickers = Vec::new();
                for ticker in v {
                    if let Some(t) = ticker.as_object_mut().map(|v| Ticker::parse(v)) {
                        tickers.push(t);
                    }
            }
            tickers
        });
        TickersSnapshot {
            status,
            count,
            tickers,
        }
    }
}

pub trait TickersSnapshotRequest {
    fn get_tickers_snapshot(
        api_key: String,
        tickers: Option<Vec<String>>,
        include_otc: Option<bool>,
        ticker_type: TickerType,
        request: &impl Request,
        verification: &impl Verification,
    ) -> Result<TickersSnapshot, ErrorCode> {
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
        let tickers_snapshot_parameters = Parameters {
            api_key: api_key,
            tickers: Some(tickers),
            include_otc: includeotc,
            ..Parameters::default()
        };
        if let Err(check) = verification.check_parameters(
            &ticker_types,
            PARAMETERS,
            &tickers_snapshot_parameters,
        ) {
            return Err(check);
        }
        let locale = match ticker_type {
            TickerType::Stocks => String::from("us"),
            TickerType::Forex | TickerType::Crypto => String::from("global"),
            _ => return Err(ErrorCode::TickerTypeeNotValidForAPICall),
        };
        let url = url(&tickers_snapshot_parameters, locale, ticker_type);
        match request.request(url) {
            Ok(mut map) => Ok(TickersSnapshot::parse(&mut map)),
            Err(e) => return Err(e),
        }
    }
}

const PARAMETERS: &'static [&'static ParameterRequirment] = &[
        &ParameterRequirment {
            required: false,
            parameter: Parameter::Tickers,
        },
        &ParameterRequirment {
            required: false,
            parameter: Parameter::IncludeOTC,
        },
    ];

fn url(parameters: &Parameters, locale: String, ticker_type: TickerType) -> String {
    let tickers = {
        let mut tickers_flattened = String::new();
        if let Some(tickers) =parameters.tickers{
            for ticker in tickers {
                tickers_flattened = tickers_flattened.replace('&', ",");
                tickers_flattened = format!("{}{}&", tickers_flattened, ticker);
            }
            tickers_flattened = format!("tickers={}", tickers_flattened);
        }
        tickers_flattened
    };
    String::from(format!(
        "https://api.polygon.io/v2/snapshot/locale/{}/markets/{}/tickers?{}{}apiKey={}",
        locale,
        ticker_type.to_string().to_lowercase(),
        tickers,
        if let Some(s) = parameters.include_otc {
            format!("include_otc={}&", s)
        } else {
            "".to_string()
        },
        parameters.api_key,
    ))
}
