use crate::{
    data_types::{quote::Quote, Parse},
    rest::{
        error::ErrorCode,
        parameters::{Order, Parameter, ParameterRequirment, Parameters, Sortv3, TickerTypes},
    },
    tools::{request::Request, verification::Verification},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BBO {
    pub next_url: Option<String>,
    pub request_id: Option<String>,
    pub results: Option<Vec<Quote>>,
    pub status: Option<String>,
}

impl BBORequest for BBO {}

impl Parse for BBO {
    fn parse(map: &serde_json::Map<String, serde_json::Value>) -> Self {
        let next_url = Self::string_parse(map, vec!["next_url"]);
        let request_id = Self::string_parse(map, vec!["request_id"]);
        let status = Self::string_parse(map, vec!["status"]);
        let results = Self::array_parse(map, vec!["results"]);

        BBO {
            request_id,
            next_url,
            status,
            results,
        }
    }
}

pub trait BBORequest {
    fn get_bbo(
        api_key: &String,
        ticker: String,
        timestamp: Option<String>,
        from: Option<String>,
        to: Option<String>,
        sort: Option<Sortv3>,
        limit: Option<u16>,
        order: Option<Order>,
    ) -> Result<BBO, ErrorCode> {
        let ts = if to.is_some() || from.is_some() {
            None
        } else {
            timestamp
        };
        let bbo_parameters = Parameters {
            api_key: api_key.to_string(),
            ticker: Some(ticker),
            timestamp: ts,
            from: from,
            to: to,
            sortv3: sort,
            limit: limit,
            order: order,
            ..Parameters::default()
        };
        if let Err(check) =
            Verification::check_parameters(&TickerTypes::forex(), PARAMETERS, &bbo_parameters)
        {
            return Err(check);
        }
        let url = match url(&bbo_parameters) {
            Ok(url) => url,
            Err(e) => return Err(e),
        };
        match Request::request(url) {
            Ok(mut map) => Ok(BBO::parse(&mut map)),
            Err(e) => return Err(e),
        }
    }
}

const PARAMETERS: &'static [&'static ParameterRequirment] = &[
    &ParameterRequirment {
        required: true,
        parameter: Parameter::Ticker,
    },
    &ParameterRequirment {
        required: false,
        parameter: Parameter::Timestamp,
    },
    &ParameterRequirment {
        required: false,
        parameter: Parameter::From,
    },
    &ParameterRequirment {
        required: false,
        parameter: Parameter::To,
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
    let url = String::from(format!(
        "https://api.polygon.io/v3/quotes/{}?{}{}{}{}{}{}apiKey={}",
        match &parameters.ticker {
            Some(ticker) => ticker,
            None => return Err(ErrorCode::TickerNotSet),
        },
        if let Some(t) = &parameters.timestamp {
            format!("timestamp={}&", t)
        } else {
            "".to_string()
        },
        if let Some(tf) = &parameters.from {
            format!("timestamp.gte={}&", tf)
        } else {
            "".to_string()
        },
        if let Some(tt) = &parameters.to {
            format!("timestamp.lte={}&", tt)
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
fn test_bbo_parse() {
    let data = serde_json::json!({
        "request_id": "req12345",
        "next_url": "https://api.polygon.io/v3/quotes/C:EURUSD?cursor=YWN0aXZlPXRydWUmZGF0ZT0yMDIzLTA0LTAxJmxpbWl0PTEmb3JkZXI9YXNjJnBhZ2VfbWFya2VyPUElMjBWU1MjQyMCU3QzIwMjMtMDQtMDElN0M5JTNBNDElN0MwMCUzQTAwJnNvcnQ9dGlja2Vy",
        "status": "OK",
        "results": [
            {
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
            }
        ]
    });
    let bbo = BBO::parse(&data.as_object().unwrap());
    assert_eq!(bbo.request_id.unwrap(), "req12345");
    assert_eq!(bbo.next_url.unwrap(), "https://api.polygon.io/v3/quotes/C:EURUSD?cursor=YWN0aXZlPXRydWUmZGF0ZT0yMDIzLTA0LTAxJmxpbWl0PTEmb3JkZXI9YXNjJnBhZ2VfbWFya2VyPUElMjBWU1MjQyMCU3QzIwMjMtMDQtMDElN0M5JTNBNDElN0MwMCUzQTAwJnNvcnQ9dGlja2Vy");
    assert_eq!(bbo.status.unwrap(), "OK");
    assert_eq!(bbo.results.unwrap()[0].bid.unwrap(), 1.23);
}

#[test]
fn test_url() {
    let mut parameters = Parameters::default();
    parameters.api_key = String::from("apiKey");
    parameters.ticker = Some(String::from("C:EURUSD"));
    parameters.from = Some(String::from("2023-03-01"));
    parameters.to = Some(String::from("2023-04-01"));
    parameters.sortv3 = Some(Sortv3::Timestamp);
    parameters.limit = Some(1);
    parameters.order = Some(Order::Asc);
    let url = url(&parameters).unwrap();
    assert_eq!(url, "https://api.polygon.io/v3/quotes/C:EURUSD?timestamp.gte=2023-03-01&timestamp.lte=2023-04-01&order=asc&limit=1&sort=timestamp&apiKey=apiKey");
}
