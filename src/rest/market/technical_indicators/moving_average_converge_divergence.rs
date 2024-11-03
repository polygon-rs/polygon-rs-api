use crate::{
    data_types::{bar::Bar, macd::MACD, Parse},
    rest::{
        error::ErrorCode,
        parameters::{
            Order, Parameter, ParameterRequirment, Parameters, SeriesType, TickerTypes, Timespan,
        },
    },
    tools::{request::Request, verification::Verification},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MovingAverageConvergenceDivergence {
    pub next_url: Option<String>,
    pub request_id: Option<String>,
    pub status: Option<String>,
    pub bars: Option<Vec<Bar>>,
    pub bars_url: Option<String>,
    pub macd: Option<Vec<MACD>>,
}

impl MovingAverageConvergenceDivergenceRequest for MovingAverageConvergenceDivergence {}

impl Parse for MovingAverageConvergenceDivergence {
    fn parse(map: &serde_json::Map<String, serde_json::Value>) -> Self {
        let next_url = Self::string_parse(map, vec!["next_url"]);
        let request_id = Self::string_parse(map, vec!["request_id"]);
        let status = Self::string_parse(map, vec!["status"]);
        let results = Self::object(map, vec!["results"]);
        let bars = match results {
            Some(bars) => Self::array_parse(bars, vec!["aggregates"]),
            None => None,
        };
        let bars_url = match results {
            Some(bars_url) => Self::string_parse(bars_url, vec!["next_url"]),
            None => None,
        };
        let macd = match results {
            Some(macd) => Self::array_parse(macd, vec!["values"]),
            None => None,
        };

        MovingAverageConvergenceDivergence {
            next_url,
            request_id,
            status,
            bars,
            bars_url,
            macd,
        }
    }
}

pub trait MovingAverageConvergenceDivergenceRequest {
    fn get_relatvie_strength(
        api_key: &String,
        ticker: String,
        timestamp: Option<String>,
        from: Option<String>,
        to: Option<String>,
        timespan: Option<Timespan>,
        adjusted: Option<bool>,
        long_window: Option<i64>,
        short_window: Option<i64>,
        signal_window: Option<i64>,
        series_type: Option<SeriesType>,
        expand_underlying: Option<bool>,
        order: Option<Order>,
        limit: Option<u16>,
    ) -> Result<MovingAverageConvergenceDivergence, ErrorCode> {
        let ts = if to.is_some() || from.is_some() {
            None
        } else {
            timestamp
        };
        let moving_average_convergence_divergence_parameters = Parameters {
            api_key: api_key.to_string(),
            ticker: Some(ticker),
            timestamp: ts,
            from: from,
            to: to,
            timespan: timespan,
            adjusted: adjusted,
            long_window: long_window,
            short_window: short_window,
            signal_window: signal_window,
            series_type: series_type,
            expand_underlying: expand_underlying,
            order: order,
            limit: limit,
            ..Parameters::default()
        };
        if let Err(check) = Verification::check_parameters(
            &TickerTypes::all(),
            PARAMETERS,
            &moving_average_convergence_divergence_parameters,
        ) {
            return Err(check);
        }
        let url = match url(&moving_average_convergence_divergence_parameters){
            Ok(url) => url,
            Err(e) => return Err(e),
        };
        match Request::request(url) {
            Ok(mut map) => Ok(MovingAverageConvergenceDivergence::parse(&mut map)),
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
        parameter: Parameter::Timespan,
    },
    &ParameterRequirment {
        required: false,
        parameter: Parameter::Adjusted,
    },
    &ParameterRequirment {
        required: false,
        parameter: Parameter::LongWindow,
    },
    &ParameterRequirment {
        required: false,
        parameter: Parameter::ShortWindow,
    },
    &ParameterRequirment {
        required: false,
        parameter: Parameter::SignalWindow,
    },
    &ParameterRequirment {
        required: false,
        parameter: Parameter::SeriesType,
    },
    &ParameterRequirment {
        required: false,
        parameter: Parameter::ExpandUnderlying,
    },
    &ParameterRequirment {
        required: false,
        parameter: Parameter::Order,
    },
    &ParameterRequirment {
        required: false,
        parameter: Parameter::Limit,
    },
];

fn url(parameters: &Parameters) -> Result<String, ErrorCode> {
    let url = String::from(format!(
        "https://api.polygon.io/v1/indicators/macd/{}?{}{}{}{}{}{}{}{}{}{}{}{}apiKey={}",
        match &parameters.ticker{
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
        if let Some(ts) = &parameters.timespan {
            format!("timespan={}&", ts.to_string().to_lowercase())
        } else {
            "".to_string()
        },
        if let Some(adj) = parameters.adjusted {
            format!("adjusted={}&", adj)
        } else {
            "".to_string()
        },
        if let Some(w) = &parameters.long_window {
            format!("long_window={}&", w)
        } else {
            "".to_string()
        },
        if let Some(w) = &parameters.short_window {
            format!("short_window={}&", w)
        } else {
            "".to_string()
        },
        if let Some(w) = &parameters.signal_window {
            format!("signal_window={}&", w)
        } else {
            "".to_string()
        },
        if let Some(st) = &parameters.series_type {
            format!("series_type={}&", st.to_string().to_lowercase())
        } else {
            "".to_string()
        },
        if let Some(eu) = &parameters.expand_underlying {
            format!("expand_underlying={}&", eu)
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
        &parameters.api_key,
    ));
    Ok(url)
}
#[test]
fn test_moving_average_converge_divergence_parse() {
    let data = serde_json::json!({
        "next_url": "https://api.polygon.io/v1/indicators/macd/AAPL?cursor=YWN0aXZlPXRydWUmZGF0ZT0yMDIzLTA0LTAxJmxpbWl0PTEmb3JkZXI9YXNjJnBhZ2VfbWFya2VyPUElMjBWU1MjQyMCU3QzIwMjMtMDQtMDElN0M5JTNBNDElN0MwMCUzQTAwJnNvcnQ9dGlja2Vy",
        "request_id": "req12345",
        "status": "OK",
        "results": {
            "aggregates": [
                {
                    "c": 1.23,
                    "h": 2.34,
                    "l": 0.12,
                    "n": 123,
                    "o": 0.12,
                    "t": 164545545,
                    "v": 456.78,
                    "vw": 901.23
                }
            ],
            "values": [
                {
                    "histogram": 1.23,
                    "signal": 2.34,
                    "timestamp": 164545545,
                    "value": 3.45
                }
            ],
            "next_url": "https://api.polygon.io/v1/indicators/macd/AAPL?cursor=YWN0aXZlPXRydWUmZGF0ZT0yMDIzLTA0LTAxJmxpbWl0PTEmb3JkZXI9YXNjJnBhZ2VfbWFya2VyPUElMjBWU1MjQyMCU3QzIwMjMtMDQtMDElN0M5JTNBNDElN0MwMCUzQTAwJnNvcnQ9dGlja2Vy"
        }
    });
    let moving_average_convergence_divergence = MovingAverageConvergenceDivergence::parse(&data.as_object().unwrap());
    assert_eq!(moving_average_convergence_divergence.next_url.unwrap(), "https://api.polygon.io/v1/indicators/macd/AAPL?cursor=YWN0aXZlPXRydWUmZGF0ZT0yMDIzLTA0LTAxJmxpbWl0PTEmb3JkZXI9YXNjJnBhZ2VfbWFya2VyPUElMjBWU1MjQyMCU3QzIwMjMtMDQtMDElN0M5JTNBNDElN0MwMCUzQTAwJnNvcnQ9dGlja2Vy");
    assert_eq!(moving_average_convergence_divergence.request_id.unwrap(), "req12345");
    assert_eq!(moving_average_convergence_divergence.status.unwrap(), "OK");
    assert_eq!(moving_average_convergence_divergence.bars.unwrap()[0].close.unwrap(), 1.23);
    assert_eq!(moving_average_convergence_divergence.macd.unwrap()[0].histogram.unwrap(), 1.23);
}

#[test]
fn test_url() {
    let mut parameters = Parameters::default();
    parameters.api_key = String::from("apiKey");
    parameters.ticker = Some(String::from("AAPL"));
    parameters.from = Some(String::from("2023-03-01"));
    parameters.to = Some(String::from("2023-04-01"));
    parameters.timespan = Some(Timespan::Minute);
    parameters.adjusted = Some(true);
    parameters.long_window = Some(26);
    parameters.short_window = Some(12);
    parameters.signal_window = Some(9);
    parameters.series_type = Some(SeriesType::Close);
    parameters.expand_underlying = Some(true);
    parameters.order = Some(Order::Asc);
    parameters.limit = Some(1000);
    let url = url(&parameters).unwrap();
    assert_eq!(url, "https://api.polygon.io/v1/indicators/macd/AAPL?timestamp.gte=2023-03-01&timestamp.lte=2023-04-01&timespan=minute&adjusted=true&long_window=26&short_window=12&signal_window=9&series_type=close&expand_underlying=true&order=asc&limit=1000&apiKey=apiKey");
}
