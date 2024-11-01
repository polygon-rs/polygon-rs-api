use crate::data_types::{l2::L2, Parse};
use crate::rest::{
    error::ErrorCode,
    parameters::{Parameter, ParameterRequirment, Parameters, TickerTypes},
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
    fn parse(map: &serde_json::Map<String, serde_json::Value>) -> Self {
        let status = Self::string_parse(map, vec!["status"]);
        let l2 = Self::array_parse(map, vec!["data"]);

        L2Snapshot { status, l2 }
    }
}

pub trait L2SnapshotRequest {
    fn get_l2(&self, api_key: String, ticker: String) -> Result<L2Snapshot, ErrorCode> {
        let l2_snapshot_parameters = Parameters {
            api_key: api_key,
            ticker: Some(ticker),
            ..Parameters::default()
        };
        if let Err(check) = Verification::check_parameters(
            &TickerTypes::crypto(),
            PARAMETERS,
            &l2_snapshot_parameters,
        ) {
            return Err(check);
        }
        let url = match url(&l2_snapshot_parameters){
            Ok(url) => url,
            Err(e) => return Err(e)
        };
        match Request::request(url) {
            Ok(mut map) => Ok(L2Snapshot::parse(&mut map)),
            Err(e) => return Err(e),
        }
    }
}

const PARAMETERS: &'static [&'static ParameterRequirment] = &[&ParameterRequirment {
    required: true,
    parameter: Parameter::Ticker,
}];

fn url(parameters: &Parameters) -> Result<String, ErrorCode> {
    let url = String::from(format!(
        "https://api.polygon.io/v2/snapshot/locale/global/markets/crypto/tickers/{}/book?apiKey={}",
        match &parameters.ticker {
            Some(ticker) => ticker,
            None => return Err(ErrorCode::TickerNotSet),
        },
        &parameters.api_key,
    ));
    Ok(url)
}
