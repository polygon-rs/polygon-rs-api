use crate::data_types::{ticker::Ticker, Parse};
use crate::rest::{
    error::ErrorCode,
    parameters::{Parameter, ParameterRequirment, Parameters, TickerType, TickerTypes},
};
use crate::tools::{request::Request, verification::Verification};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TickerSnapshot {
    pub status: Option<String>,
    pub ticker: Option<Ticker>,
    pub request_id: Option<String>,
}

impl TickerSnapshotRequest for TickerSnapshot {}

impl Parse for TickerSnapshot {
    fn parse(map: &serde_json::Map<String, serde_json::Value>) -> Self {
        let status = Self::string_parse(map, vec!["status"]);
        let request_id = Self::string_parse(map, vec!["request_id"]);
        let ticker = Self::object_parse(map, vec!["ticker"]);

        TickerSnapshot {
            status,
            request_id,
            ticker,
        }
    }
}

pub trait TickerSnapshotRequest {
    fn get_ticker_snapshot(
        
        api_key: &String,
        ticker: String,
        ticker_type: TickerType,
    ) -> Result<TickerSnapshot, ErrorCode> {
        let ticker_snapshot_parameters = Parameters {
            api_key: api_key.to_string(),
            ticker: Some(ticker),
            ..Parameters::default()
        };
        if let Err(check) = Verification::check_parameters(
            &TickerTypes::set(true, false, false, true, true),
            PARAMETERS,
            &ticker_snapshot_parameters,
        ) {
            return Err(check);
        }
        let locale = match ticker_type {
            TickerType::Stocks => String::from("us"),
            TickerType::Forex | TickerType::Crypto => String::from("global"),
            _ => return Err(ErrorCode::TickerTypeeNotValidForAPICall),
        };
        let url = match url(&ticker_snapshot_parameters, locale, ticker_type){
            Ok(url) => url,
            Err(e) => return Err(e)
        };
        match Request::request(url) {
            Ok(mut map) => Ok(TickerSnapshot::parse(&mut map)),
            Err(e) => return Err(e),
        }
    }
}

const PARAMETERS: &'static [&'static ParameterRequirment] = &[&ParameterRequirment {
    required: true,
    parameter: Parameter::Ticker,
}];

fn url(parameters: &Parameters, locale: String, ticker_type: TickerType) -> Result<String, ErrorCode> {
    let url = String::from(format!(
        "https://api.polygon.io/v2/snapshot/locale/{}/markets/{}/tickers/{}?apiKey={}",
        locale,
        ticker_type.to_string().to_lowercase(),
        match &parameters.ticker{
            Some(ticker) => ticker,
            None => return Err(ErrorCode::TickerNotSet)
        },
        &parameters.api_key,
    ));
    Ok(url)
}
#[test]
fn test_ticker_snapshot_parse() {
    let data = serde_json::json!({
        "status": "OK",
        "request_id": "req12345",
        "ticker": {
            "day": {
                "change": 1.0,
                "change_percent": 2.0,
                "c": 3.0,
                "h": 4.0,
                "last_updated": 164545545,
                "l": 5.0,
                "o": 6.0,
                "previous_close": 7.0,
                "v": 8,
                "vw": 9.0,
                "otc": false
            },
            "lastTrade": {
                "conditions": [
                    29
                ],
                "exchange_id": 30,
                "price": 31.0,
                "sip_timestamp": 164545549,
                "size": 32,
                "trade_id": "trade",
                "timeframe": "REAL-TIME",
                "exchange": "TEST1",
                "trade_correction": 33,
                "trf_timestamp": 164545550,
                "sequence_number": 34,
                "trf_id": 35,
                "participant_timestamp": 164545551,
                "tape": 36
            },
            "lastQuote": {
                "p": 1.23,
                "s": 456,
                "P": 7.89,
                "S": 123,
                "bid_exchange": 10,
                "ask_exchange": 11,
                "t": 164545545,
                "mid_point": 4.56,
                "timeframe": "DELAYED",
                "x": 12,
                "T": "TEST",
                "c": [
                    13
                ],
                "f": 164545546,
                "i": [
                    14
                ],
                "q": 15,
                "y": 164545547,
                "z": 16
            },
            "min": {
                "av": 123456,
                "o": 1.23,
                "h": 2.34,
                "l": 0.12,
                "c": 3.45,
                "v": 456.78,
                "vw": 901.23,
                "n": 123,
                "t": 164545545
            },
            "prevDay": {
                "change": 10.0,
                "change_percent": 20.0,
                "c": 30.0,
                "h": 40.0,
                "last_updated": 164545500,
                "l": 50.0,
                "o": 60.0,
                "previous_close": 70.0,
                "v": 80,
                "vw": 90.0,
                "otc": false
            },
            "ticker": "TEST",
            "todaysChange": 100.0,
            "todaysChangePerc": 200.0,
            "updated": 164545600,
            "fmv": 1000.0
        }
    });
    let ticker_snapshot = TickerSnapshot::parse(&data.as_object().unwrap());
    assert_eq!(ticker_snapshot.status.unwrap(), "OK");
    assert_eq!(ticker_snapshot.request_id.unwrap(), "req12345");
    assert_eq!(ticker_snapshot.ticker.unwrap().ticker.unwrap(), "TEST");
}

#[test]
fn test_url() {
    let mut parameters = Parameters::default();
    parameters.api_key = String::from("apiKey");
    parameters.ticker = Some(String::from("AAPL"));
    let url = url(&parameters, String::from("us"), TickerType::Stocks).unwrap();
    assert_eq!(url, "https://api.polygon.io/v2/snapshot/locale/us/markets/stocks/tickers/AAPL?apiKey=apiKey");
}
