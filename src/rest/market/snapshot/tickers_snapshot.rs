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

#[test]
fn test_tickers_snapshot_parse() {
    let data = serde_json::json!({
        "status": "OK",
        "tickers": [
            {
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
        ],
        "count": 1
    });
    let tickers_snapshot = TickersSnapshot::parse(&data.as_object().unwrap());
    assert_eq!(tickers_snapshot.status.unwrap(), "OK");
    assert_eq!(tickers_snapshot.count.unwrap(), 1);
    assert_eq!(tickers_snapshot.tickers.unwrap()[0].ticker.clone().unwrap(), "TEST");
}

#[test]
fn test_url() {
    let mut parameters = Parameters::default();
    parameters.api_key = String::from("apiKey");
    parameters.tickers = Some(vec![String::from("AAPL"), String::from("MSFT")]);
    parameters.include_otc = Some(true);
    let url = url(&parameters, String::from("us"), TickerType::Stocks).unwrap();
    assert_eq!(url, "https://api.polygon.io/v2/snapshot/locale/us/markets/stocks/tickers?tickers=AAPL,MSFT&include_otc=true&apiKey=apiKey");
}
