use crate::data_types::{ticker::Ticker, Parse};
use crate::rest::{
    error::ErrorCode,
    parameters::{Direction, Parameter, ParameterRequirment, Parameters, TickerType, TickerTypes},
};
use crate::tools::{request::Request, verification::Verification};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GainersLosers {
    pub status: Option<String>,
    pub tickers: Option<Vec<Ticker>>,
}

impl GainersLosersRequest for GainersLosers {}

impl Parse for GainersLosers {
    fn parse(map: &serde_json::Map<String, serde_json::Value>) -> Self {
        let status = Self::string_parse(map, vec!["status"]);
        let tickers = Self::array_parse(map, vec!["tickers"]);
        GainersLosers { status, tickers }
    }
}

pub trait GainersLosersRequest {
    fn get_gainers_losers(
        api_key: &String,
        direction: Direction,
        include_otc: Option<bool>,
        ticker_type: TickerType,
    ) -> Result<GainersLosers, ErrorCode> {
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
        let gainers_losers_parameters = Parameters {
            api_key: api_key.to_string(),
            direction: Some(direction),
            include_otc: includeotc,
            ..Parameters::default()
        };
        if let Err(check) =
            Verification::check_parameters(&ticker_types, PARAMETERS, &gainers_losers_parameters)
        {
            return Err(check);
        }
        let locale = match ticker_type {
            TickerType::Stocks => String::from("us"),
            TickerType::Forex | TickerType::Crypto => String::from("global"),
            _ => return Err(ErrorCode::TickerTypeeNotValidForAPICall),
        };
        let url = match url(&gainers_losers_parameters, locale, ticker_type) {
            Ok(url) => url,
            Err(e) => return Err(e),
        };
        match Request::request(url) {
            Ok(mut map) => Ok(GainersLosers::parse(&mut map)),
            Err(e) => return Err(e),
        }
    }
}

const PARAMETERS: &'static [&'static ParameterRequirment] = &[
    &ParameterRequirment {
        required: true,
        parameter: Parameter::Direction,
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
    let url = String::from(format!(
        "https://api.polygon.io/v2/snapshot/locale/{}/markets/{}/{}?{}apiKey={}",
        locale,
        ticker_type.to_string().to_lowercase(),
        if let Some(s) = &parameters.direction {
            format!("{}", s.to_string().to_lowercase())
        } else {
            "".to_string()
        },
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
fn test_gainers_losers_parse() {
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
        ]
    });
    let gainers_losers = GainersLosers::parse(&data.as_object().unwrap());
    assert_eq!(gainers_losers.status.unwrap(), "OK");
    assert_eq!(gainers_losers.tickers.unwrap()[0].day.as_ref().unwrap().change.unwrap(), 1.0);
}

#[test]
fn test_url() {
    let mut parameters = Parameters::default();
    parameters.api_key = String::from("apiKey");
    parameters.direction = Some(Direction::Gainers);
    parameters.include_otc = Some(true);
    let url = url(&parameters, String::from("us"), TickerType::Stocks).unwrap();
    assert_eq!(url, "https://api.polygon.io/v2/snapshot/locale/us/markets/stocks/gainers?include_otc=true&apiKey=apiKey");
}