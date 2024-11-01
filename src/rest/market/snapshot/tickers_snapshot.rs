use crate::data_types::{ticker::Ticker, Parse};
use crate::rest::{
    error::ErrorCode,
    parameters::{Parameter, ParameterRequirment, Parameters, TickerType, TickerTypes},
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
    fn parse(map: &serde_json::Map<String, serde_json::Value>) -> Self {
        let status = Self::string_parse(map, vec!["status"]);
        let count = Self::i64_parse(map, vec!["count"]);
        let tickers = Self::array_parse(map, vec!["tickers"]);

        TickersSnapshot {
            status,
            count,
            tickers,
        }
    }
}

pub trait TickersSnapshotRequest {
    fn get_tickers_snapshot(
        api_key: &String,
        tickers: Option<Vec<String>>,
        include_otc: Option<bool>,
        ticker_type: TickerType,
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
            api_key: api_key.to_string(),
            tickers: tickers,
            include_otc: includeotc,
            ..Parameters::default()
        };
        if let Err(check) =
            Verification::check_parameters(&ticker_types, PARAMETERS, &tickers_snapshot_parameters)
        {
            return Err(check);
        }
        let locale = match ticker_type {
            TickerType::Stocks => String::from("us"),
            TickerType::Forex | TickerType::Crypto => String::from("global"),
            _ => return Err(ErrorCode::TickerTypeeNotValidForAPICall),
        };
        let url = match url(&tickers_snapshot_parameters, locale, ticker_type) {
            Ok(url) => url,
            Err(e) => return Err(e),
        };
        match Request::request(url) {
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

fn url(
    parameters: &Parameters,
    locale: String,
    ticker_type: TickerType,
) -> Result<String, ErrorCode> {
    let tickers = {
        let mut tickers_flattened = String::new();
        if let Some(tickers) = &parameters.tickers {
            for ticker in tickers {
                tickers_flattened = tickers_flattened.replace('&', ",");
                tickers_flattened = format!("{}{}&", tickers_flattened, ticker);
            }
            tickers_flattened = format!("tickers={}", tickers_flattened);
        }
        tickers_flattened
    };
    let url = String::from(format!(
        "https://api.polygon.io/v2/snapshot/locale/{}/markets/{}/tickers?{}{}apiKey={}",
        locale,
        ticker_type.to_string().to_lowercase(),
        tickers,
        if let Some(s) = &parameters.include_otc {
            format!("include_otc={}&", s)
        } else {
            "".to_string()
        },
        &parameters.api_key,
    ));
    Ok(url)
}
