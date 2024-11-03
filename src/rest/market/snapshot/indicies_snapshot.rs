use crate::data_types::{indicie::Indicie, Parse};
use crate::rest::{
    error::ErrorCode,
    parameters::{Order, Parameter, ParameterRequirment, Parameters, Sortv3, TickerTypes},
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

impl IndiciesSnapshotRequest for IndiciesSnapshot {}

impl Parse for IndiciesSnapshot {
    fn parse(map: &serde_json::Map<String, serde_json::Value>) -> Self {
        let status = Self::string_parse(map, vec!["status"]);
        let request_id = Self::string_parse(map, vec!["request_id"]);
        let next_url = Self::string_parse(map, vec!["next_url"]);
        let indicies = Self::array_parse(map, vec!["results"]);

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
        api_key: &String,
        tickers: Option<Vec<String>>,
        ticker_from: Option<String>,
        ticker_to: Option<String>,
        sort: Option<Sortv3>,
        limit: Option<u16>,
        order: Option<Order>,
    ) -> Result<IndiciesSnapshot, ErrorCode> {
        let tickers = if ticker_from.is_some() || ticker_to.is_some() {
            None
        } else {
            tickers
        };
        let indicies_snapshot_parameters = Parameters {
            api_key: api_key.to_string(),
            tickers: tickers,
            ticker_from: ticker_from,
            ticker_to: ticker_to,
            sortv3: sort,
            limit: limit,
            order: order,
            ..Parameters::default()
        };
        if let Err(check) = Verification::check_parameters(
            &TickerTypes::indicies(),
            PARAMETERS,
            &indicies_snapshot_parameters,
        ) {
            return Err(check);
        }
        let url = match url(&indicies_snapshot_parameters) {
            Ok(url) => url,
            Err(e) => return Err(e),
        };
        match Request::request(url) {
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
        "https://api.polygon.io/v3/snapshot/indicies?{}{}{}{}{}{}apiKey={}",
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
        if let Some(o) = &parameters.order {
            format!("order={}&", o.to_string().to_lowercase())
        } else {
            "".to_string()
        },
        if let Some(l) = &parameters.limit {
            format!("limit={}&", l)
        } else {
            "".to_string()
        },
        if let Some(s) = &parameters.sortv3 {
            format!("sort={}&", s.to_string().to_lowercase())
        } else {
            "".to_string()
        },
        &parameters.api_key,
    ));
    Ok(url)
}

#[test]
fn test_indicies_snapshot_parse() {
    let data = serde_json::json!({
        "status": "OK",
        "request_id": "req12345",
        "next_url": "https://api.polygon.io/v3/snapshot/indicies?cursor=YWN0aXZlPXRydWUmZGF0ZT0yMDIzLTA0LTAxJmxpbWl0PTEmb3JkZXI9YXNjJnBhZ2VfbWFya2VyPUElMjBWU1MjQyMCU3QzIwMjMtMDQtMDElN0M5JTNBNDElN0MwMCUzQTAwJnNvcnQ9dGlja2Vy",
        "results": [
            {
                "ticker": "DJIA",
                "session": {
                    "change": 1.23,
                    "change_percent": 2.34,
                    "close": 3.45,
                    "high": 4.56,
                    "low": 5.67,
                    "open": 6.78,
                    "previous_close": 7.89
                },
                "value": 12345.67,
                "ticker_type": "indicies",
                "timeframe": "2023-04-01",
                "name": "Dow Jones Industrial Average",
                "market_status": "PRE"
            }
        ]
    });
    let indicies_snapshot = IndiciesSnapshot::parse(&data.as_object().unwrap());
    assert_eq!(indicies_snapshot.status.unwrap(), "OK");
    assert_eq!(indicies_snapshot.request_id.unwrap(), "req12345");
    assert_eq!(indicies_snapshot.next_url.unwrap(), "https://api.polygon.io/v3/snapshot/indicies?cursor=YWN0aXZlPXRydWUmZGF0ZT0yMDIzLTA0LTAxJmxpbWl0PTEmb3JkZXI9YXNjJnBhZ2VfbWFya2VyPUElMjBWU1MjQyMCU3QzIwMjMtMDQtMDElN0M5JTNBNDElN0MwMCUzQTAwJnNvcnQ9dGlja2Vy");
    assert_eq!(indicies_snapshot.indicies.unwrap()[0].ticker.clone().unwrap(), "DJIA");
}

#[test]
fn test_url() {
    let mut parameters = Parameters::default();
    parameters.api_key = String::from("apiKey");
    parameters.tickers = Some(vec![String::from("I:DJI"), String::from("I:SPX")]);
    parameters.sortv3 = Some(Sortv3::Ticker);
    parameters.limit = Some(1);
    parameters.order = Some(Order::Asc);
    let url = url(&parameters).unwrap();
    assert_eq!(url, "https://api.polygon.io/v3/snapshot/indicies?tickers.any_of=I:DJI,I:SPX&order=asc&limit=1&sort=ticker&apiKey=apiKey");
}