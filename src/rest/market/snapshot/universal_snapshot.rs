use crate::data_types::{universal::Universal, Parse};
use crate::rest::{
    error::ErrorCode,
    parameters::{
        Order, Parameter, ParameterRequirment, Parameters, Sortv3, TickerType, TickerTypes,
    },
};
use crate::tools::{request::Request, verification::Verification};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UniversalSnapshot {
    pub status: Option<String>,
    pub universal: Option<Vec<Universal>>,
    pub request_id: Option<String>,
    pub next_url: Option<String>,
}

impl UniversalSnapshotRequest for UniversalSnapshot {}

impl Parse for UniversalSnapshot {
    fn parse(map: &serde_json::Map<String, serde_json::Value>) -> Self {
        let status = Self::string_parse(map, vec!["status"]);
        let request_id = Self::string_parse(map, vec!["request_id"]);
        let next_url = Self::string_parse(map, vec!["next_url"]);
        let universal = Self::array_parse(map, vec!["results"]);

        UniversalSnapshot {
            status,
            request_id,
            universal,
            next_url,
        }
    }
}

pub trait UniversalSnapshotRequest {
    fn get_universal_snapshot(
        &self,
        api_key: String,
        tickers: Option<Vec<String>>,
        ticker_from: Option<String>,
        ticker_to: Option<String>,
        ticker_type: Option<TickerType>,
        sort: Option<Sortv3>,
        limit: Option<u16>,
        order: Option<Order>,
    ) -> Result<UniversalSnapshot, ErrorCode> {
        let tickers = if ticker_from.is_some() || ticker_to.is_some() {
            None
        } else {
            tickers
        };
        let universal_snapshot_parameters = Parameters {
            api_key: api_key,
            tickers: tickers,
            ticker_from: ticker_from,
            ticker_to: ticker_to,
            ticker_type: ticker_type,
            sortv3: sort,
            limit: limit,
            order: order,
            ..Parameters::default()
        };
        let ticker_types = match ticker_type {
            Some(t) => match t {
                TickerType::Indicies => TickerTypes::indicies(),
                TickerType::Stocks => TickerTypes::stocks(),
                TickerType::Crypto => TickerTypes::crypto(),
                TickerType::Forex => TickerTypes::forex(),
                TickerType::Options => TickerTypes::options(),
            },
            None => TickerTypes::all(),
        };
        if let Err(check) = Verification::check_parameters(
            &ticker_types,
            PARAMETERS,
            &universal_snapshot_parameters,
        ) {
            return Err(check);
        }
        let url = match url(&universal_snapshot_parameters){
            Ok(url) => url,
            Err(e) => return Err(e)
        };
        match Request::request(url) {
            Ok(mut map) => Ok(UniversalSnapshot::parse(&mut map)),
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
        parameter: Parameter::TickerFrom,
    },
    &ParameterRequirment {
        required: false,
        parameter: Parameter::TickerTo,
    },
    &ParameterRequirment {
        required: false,
        parameter: Parameter::TickerType,
    },
    &ParameterRequirment {
        required: false,
        parameter: Parameter::Sortv3,
    },
    &ParameterRequirment {
        required: false,
        parameter: Parameter::Limit,
    },
    &ParameterRequirment {
        required: false,
        parameter: Parameter::Order,
    },
];

fn url(parameters: &Parameters) -> Result<String, ErrorCode> {
    let tickers = {
        let mut tickers_flattened = String::new();
        if let Some(tickers) = &parameters.tickers {
            for ticker in tickers {
                tickers_flattened = tickers_flattened.replace('&', ",");
                tickers_flattened = format!("{}{}&", tickers_flattened, ticker);
            }
            tickers_flattened = format!("tickers.any_of={}", tickers_flattened);
        }
        tickers_flattened
    };
    let url = String::from(format!(
        "https://api.polygon.io/v3/snapshot?{}{}{}{}{}{}{}apiKey={}",
        tickers,
        if let Some(tf) = &parameters.ticker_from {
            format!("ticker.gte={}&", tf)
        } else {
            "".to_string()
        },
        if let Some(tt) = &parameters.ticker_to {
            format!("ticker.lte={}&", tt)
        } else {
            "".to_string()
        },
        if let Some(tt) = parameters.ticker_type {
            let t = match tt {
                TickerType::Indicies => "indicies",
                TickerType::Stocks => "stocks",
                TickerType::Crypto => "crpyto",
                TickerType::Forex => "fx",
                TickerType::Options => "options",
            };
            format!("type={}&", t)
        } else {
            "".to_string()
        },
        if let Some(o) = &parameters.order {
            format!("order={}&", o)
        } else {
            "".to_string()
        },
        if let Some(l) = &parameters.limit {
            format!("limit={}&", l)
        } else {
            "".to_string()
        },
        if let Some(s) = &parameters.sortv3 {
            format!("sort={}&", s)
        } else {
            "".to_string()
        },
        &parameters.api_key,
    ));
    Ok(url)
}
