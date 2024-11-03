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
    fn get_l2(api_key: &String, ticker: String) -> Result<L2Snapshot, ErrorCode> {
        let l2_snapshot_parameters = Parameters {
            api_key: api_key.to_string(),
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
        let url = match url(&l2_snapshot_parameters) {
            Ok(url) => url,
            Err(e) => return Err(e),
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

#[test]
fn test_l2_snapshot_parse() {
    let data = serde_json::json!({
        "status": "OK",
        "data": [
            {
                "bids": [
                    {
                        "price": 1.23,
                        "size": {
                            "a": 1.0,
                            "b": 2.0,
                        }
                    }
                ],
                "asks": [
                    {
                        "price": 4.56,
                        "size": {
                            "a": 7.0,
                            "b": 8.0,
                        }
                    }
                ],
                "bid_count": 1,
                "ask_count": 1,
                "timestamp": 164545545,
                "spread": 3.33,
                "ticker": "TEST"
            }
        ]
    });
    let l2_snapshot = L2Snapshot::parse(&data.as_object().unwrap());
    assert_eq!(l2_snapshot.status.unwrap(), "OK");
    assert_eq!(l2_snapshot.l2.unwrap()[0].bids.clone().unwrap()[0].price.unwrap(), 1.23);
}

#[test]
fn test_url() {
    let mut parameters = Parameters::default();
    parameters.api_key = String::from("apiKey");
    parameters.ticker = Some(String::from("X:BTCUSD"));
    let url = url(&parameters).unwrap();
    assert_eq!(url, "https://api.polygon.io/v2/snapshot/locale/global/markets/crypto/tickers/X:BTCUSD/book?apiKey=apiKey");
}
