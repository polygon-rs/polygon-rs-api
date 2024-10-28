use crate::data_types::{indicie::Indicie, Parse};
use crate::rest::{
    parameters::{Order, Parameter, ParameterRequirment, Parameters, Sortv3, TickerTypes},
    error::ErrorCode,
};
use crate::tools::{request::Request, verification::Verification};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct IndiciesSnapshot {
    pub status: Option<String>,
    pub indicies: Option<Vec<Indicie>>,
    pub request_id: Option<String>,
    pub next_url: Option<String>,
}

impl IndiciesSnapshot {
    fn next(&mut self, api_key: String, request: &impl Request) -> Result<(), ErrorCode> {
        if self.next_url.is_none() {
            return Err(ErrorCode::NoNextURL);
        }
        let next_url = if let Some(next_url) = &self.next_url {
            format!("{}&apiKey={}",next_url, api_key)
        } else { return Err(ErrorCode::NoNextURL); };
        match request.request(next_url) {
            Ok(mut map) => {*self = IndiciesSnapshot::parse(&mut map); Ok(())},
            Err(e) => return Err(e),
        }
    }
}

impl IndiciesSnapshotRequest for IndiciesSnapshot {}

impl Parse for IndiciesSnapshot {
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
        let indicies = map.get_mut("results").and_then(|v| v.as_array()).map(|v| {
            let mut indicies = Vec::new();
            for indicie in v {
                if let Some(i) = indicie.clone().as_object_mut().map(|v| Indicie::parse(v)) {
                    indicies.push(i);
                }
            }
            indicies
        });
        IndiciesSnapshot {
            status: status,
            request_id: request_id,
            indicies,
            next_url,
        }
    }
}

pub trait IndiciesSnapshotRequest {
    fn get_indicie_snapshot(
        api_key: String,
        tickers: Option<Vec<String>>,
        ticker_from: Option<String>,
        ticker_to: Option<String>,
        sort: Option<Sortv3>,
        limit: Option<u16>,
        order: Option<Order>,
        request: &impl Request,
        verification: &impl Verification,
    ) -> Result<IndiciesSnapshot, ErrorCode> {
        let tickers = if ticker_from.is_some() || ticker_to.is_some() {
            None
        } else {
            tickers
        };
        let indicies_snapshot_parameters = Parameters {
            api_key: api_key,
            tickers: tickers,
            ticker_from: ticker_from,
            ticker_to: ticker_to,
            sortv3: sort,
            limit: limit,
            order: order,
            ..Parameters::default()
        };
        if let Err(check) = verification.check_parameters(
            &TickerTypes::indicies(),
            PARAMETERS,
            &indicies_snapshot_parameters,
        ) {
            return Err(check);
        }
        let url = url(&indicies_snapshot_parameters);
        match request.request(url) {
            Ok(mut map) => Ok(IndiciesSnapshot::parse(&mut map)),
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
        "https://api.polygon.io/v3/snapshot/indicies?{}{}{}{}{}{}apiKey={}",
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
