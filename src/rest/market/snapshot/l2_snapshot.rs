use crate::data_types::{l2::L2, Parse};
use crate::rest::{
    parameters::{Parameter, ParameterRequirment, Parameters, TickerTypes},
    error::ErrorCode,
};
use crate::tools::{request::Request, verification::Verification};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct L2Snapshot {
    pub status: Option<String>,
    pub l2: Option<Vec<L2>>,
}

impl L2SnapshotRequest for L2Snapshot {}

impl Parse for L2Snapshot {
    fn parse(map: &mut serde_json::Map<String, serde_json::Value>) -> Self {
        let status = map
            .get("status")
            .and_then(|v| v.as_str())
            .map(|v| v.to_string());
        let l2 = map.get_mut("data").and_then(|v| v.as_array()).map(|v| {
            let mut l2s = Vec::new();
            for l in v {
                if let Some(i) = l.as_object_mut().map(|v| L2::parse(v)) {
                    l2s.push(i);
                }
            }
            l2s
        });
        L2Snapshot {
            status,
            l2,
        }
    }
}

pub trait L2SnapshotRequest {
    fn get_indicie_snapshot(
        api_key: String,
        ticker: String,
        request: &impl Request,
        verification: &impl Verification,
    ) -> Result<L2Snapshot, ErrorCode> {
        let l2_snapshot_parameters = Parameters {
            api_key: api_key,
            ticker: Some(ticker),
            ..Parameters::default()
        };
        if let Err(check) = verification.check_parameters(
            &TickerTypes::crypto(),
            PARAMETERS,
            &l2_snapshot_parameters,
        ) {
            return Err(check);
        }
        let url = url(&l2_snapshot_parameters);
        match request.request(url) {
            Ok(mut map) => Ok(L2Snapshot::parse(&mut map)),
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
        "https://api.polygon.io/v2/snapshot/locale/global/markets/crypto/tickers/{}/book?apiKey={}",
        if let Some(s) = parameters.ticker {
           s
        } else {
            "".to_string()
        },
        parameters.api_key,
    ))
}
