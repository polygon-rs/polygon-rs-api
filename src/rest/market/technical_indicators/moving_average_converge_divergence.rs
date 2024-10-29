use crate::{
    data_types::{bar::Bar, macd::MACD, Parse},
    rest::{
        error::ErrorCode,
        parameters::{
            Order, Parameter, ParameterRequirment, Parameters, SeriesType, TickerTypes, Timespan,
        },
    },
    tools::{
        request::{Next, Request},
        verification::Verification,
    },
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
    fn parse(map: &mut serde_json::Map<String, serde_json::Value>) -> Self {
        let next_url = map
            .get("next_url")
            .and_then(|v| v.as_str())
            .map(|v| v.to_string());
        let request_id = map
            .get("request_id")
            .and_then(|v| v.as_str())
            .map(|v| v.to_string());
        let status = map
            .get("status")
            .and_then(|v| v.as_str())
            .map(|v| v.to_string());
        let underyling = map.get("results").and_then(|v| {
            v.as_object();
            v.get("underlying").and_then(|v| v.as_object())
        });
        let bars = match underyling {
            Some(u) => u.get("aggregates").and_then(|v| v.as_array()).map(|v| {
                v.iter()
                    .map(|v| Bar::parse(v.clone().as_object_mut().unwrap()))
                    .collect()
            }),
            None => None,
        };
        let bars_url = match underyling {
            Some(u) => u
                .get("next_url")
                .and_then(|v| v.as_str())
                .map(|v| v.to_string()),
            None => None,
        };
        let macd = map.get("results").and_then(|v| {
            v.as_object();
            v.get("values").and_then(|v| v.as_array()).map(|v| {
                v.iter()
                    .map(|v| MACD::parse(v.clone().as_object_mut().unwrap()))
                    .collect()
            })
        });

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

impl Next for MovingAverageConvergenceDivergence {}

pub trait MovingAverageConvergenceDivergenceRequest {
    fn get_relatvie_strength(
        api_key: String,
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
        request: &impl Request,
        verification: &impl Verification,
    ) -> Result<MovingAverageConvergenceDivergence, ErrorCode> {
        let ts = if to.is_some() || from.is_some() {
            None
        } else {
            timestamp
        };
        let moving_average_convergence_divergence_parameters = Parameters {
            api_key: api_key,
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
        if let Err(check) = verification.check_parameters(
            &TickerTypes::all(),
            PARAMETERS,
            &moving_average_convergence_divergence_parameters,
        ) {
            return Err(check);
        }
        let url = url(&moving_average_convergence_divergence_parameters);
        match request.request(url) {
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

fn url(parameters: &Parameters) -> String {
    String::from(format!(
        "https://api.polygon.io/v1/indicators/macd/{}?{}{}{}{}{}{}{}{}{}{}{}{}apiKey={}",
        parameters.ticker.clone().unwrap(),
        if let Some(t) = parameters.clone().timestamp {
            format!("timestamp={}&", t)
        } else {
            "".to_string()
        },
        if let Some(tf) = parameters.clone().from {
            format!("timestamp.gte={}&", tf)
        } else {
            "".to_string()
        },
        if let Some(tt) = parameters.clone().to {
            format!("timestamp.lte={}&", tt)
        } else {
            "".to_string()
        },
        if let Some(ts) = parameters.clone().timespan {
            format!("timespan={}&", ts)
        } else {
            "".to_string()
        },
        if let Some(adj) = parameters.adjusted {
            format!("adjusted={}&", adj)
        } else {
            "".to_string()
        },
        if let Some(w) = parameters.clone().long_window {
            format!("long_window={}&", w)
        } else {
            "".to_string()
        },
        if let Some(w) = parameters.clone().short_window {
            format!("short_window={}&", w)
        } else {
            "".to_string()
        },
        if let Some(w) = parameters.clone().signal_window {
            format!("signal_window={}&", w)
        } else {
            "".to_string()
        },
        if let Some(st) = parameters.clone().series_type {
            format!("series_type={}&", st)
        } else {
            "".to_string()
        },
        if let Some(eu) = parameters.clone().expand_underlying {
            format!("expand_underlying={}&", eu)
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
        parameters.api_key,
    ))
}
