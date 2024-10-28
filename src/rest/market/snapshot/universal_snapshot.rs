use crate::data_types::{universal::Universal, Parse};
use crate::rest::{
    parameters::{Order, Parameter, ParameterRequirment, Parameters, Sortv3, TickerType, TickerTypes},
    error::ErrorCode,
};
use crate::tools::{request::{Next, Request}, verification::Verification};
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
    fn parse(map: &mut serde_json::Map<String, serde_json::Value>) -> Self {
        let status = map
            .get("status")
            .and_then(|v| v.as_str())
            .map(|v| v.to_string());
        let request_id = map
            .get("request_id")
            .and_then(|v| v.as_str())
            .map(|v| v.to_string());
        let next_url = map
            .get("next_url")
            .and_then(|v| v.as_str())
            .map(|v| v.to_string());
        let universal = map.get_mut("results").and_then(|v| v.as_array()).map(|v| {
            let mut universal_snapshots = Vec::new();
            for universal_snapshot in v {
                if let Some(us) = universal_snapshot.clone().as_object_mut().map(|v| Universal::parse(v)) {
                    universal_snapshots.push(us);
                }
            }
            universal_snapshots
        });
        UniversalSnapshot {
            status: status,
            request_id: request_id,
            universal,
            next_url,
        }
    }
}

impl Next for UniversalSnapshot {}

pub trait UniversalSnapshotRequest {
    fn get_universal_snapshot(
        api_key: String,
        tickers: Option<Vec<String>>,
        ticker_from: Option<String>,
        ticker_to: Option<String>,
        ticker_type: Option<TickerType>,
        sort: Option<Sortv3>,
        limit: Option<u16>,
        order: Option<Order>,
        request: &impl Request,
        verification: &impl Verification,
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
        if let Err(check) = verification.check_parameters(
            &ticker_types,
            PARAMETERS,
            &universal_snapshot_parameters,
        ) {
            return Err(check);
        }
        let url = url(&universal_snapshot_parameters);
        match request.request(url) {
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
    &ParameterRequirment{
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

fn url(parameters: &Parameters) -> String {
    let tickers = {
        let mut tickers_flattened = String::new();
        if let Some(tickers) = parameters.clone().tickers {
            for ticker in tickers {
                tickers_flattened = tickers_flattened.replace('&', ",");
                tickers_flattened = format!("{}{}&", tickers_flattened, ticker);
            }
            tickers_flattened = format!("tickers.any_of={}", tickers_flattened);
        }
        tickers_flattened
    };
    String::from(format!(
        "https://api.polygon.io/v3/snapshot?{}{}{}{}{}{}{}apiKey={}",
        tickers,
        if let Some(tf) = parameters.clone().ticker_from {
            format!("ticker.gte={}&", tf)
        } else {
            "".to_string()
        },
        if let Some(tt) = parameters.clone().ticker_to {
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
        if let Some(o) = parameters.clone().order {
            format!("order={}&", o)
        } else {
            "".to_string()
        },
        if let Some(l) = parameters.clone().limit {
            format!("limit={}&", l)
        } else {
            "".to_string()
        },
        if let Some(s) = parameters.clone().sortv3 {
            format!("sort={}&", s)
        } else {
            "".to_string()
        },
        parameters.api_key,
    ))
}

