use crate::{
    data_types::{trade::Trade, Parse},
    rest::{
        error::ErrorCode,
        parameters::{Order, Parameter, ParameterRequirment, Parameters, Sortv3, TickerTypes},
    },
    tools::{
        request::{Next, Request},
        verification::Verification,
    },
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Trades {
    pub next_url: Option<String>,
    pub request_id: Option<String>,
    pub trades: Option<Vec<Trade>>,
    pub status: Option<String>,
}

impl TradesRequest for Trades {}

impl Parse for Trades {
    fn parse(map: &serde_json::Map<String, serde_json::Value>) -> Self {
        let request_id = Self::string_parse(map, vec!["request_id"]);
        let next_url: Option<String> = Self::string_parse(map, vec!["next_url"]);
        let trades = Self::array_parse(map, vec!["results"]);
        let status = Self::string_parse(map, vec!["status"]);

        Trades {
            request_id,
            next_url,
            trades,
            status,
        }
    }
}

impl Next for Trades {}

pub trait TradesRequest {
    fn get_trades(
        api_key: String,
        ticker: String,
        timestamp: Option<String>,
        from: Option<String>,
        to: Option<String>,
        sort: Option<Sortv3>,
        limit: Option<u16>,
        order: Option<Order>,
        request: &impl Request,
        verification: &impl Verification,
    ) -> Result<Trades, ErrorCode> {
        let ts = if to.is_some() || from.is_some() {
            None
        } else {
            timestamp
        };
        let trades_parameters = Parameters {
            api_key: api_key,
            ticker: Some(ticker),
            timestamp: ts,
            from: from,
            to: to,
            sortv3: sort,
            limit: limit,
            order: order,
            ..Parameters::default()
        };
        if let Err(check) = verification.check_parameters(
            &TickerTypes::set(true, true, false, false, true),
            PARAMETERS,
            &trades_parameters,
        ) {
            return Err(check);
        }
        let url = url(&trades_parameters);
        match request.request(url) {
            Ok(mut map) => Ok(Trades::parse(&mut map)),
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
        parameter: Parameter::Order,
    },
    &ParameterRequirment {
        required: false,
        parameter: Parameter::Limit,
    },
    &ParameterRequirment {
        required: false,
        parameter: Parameter::Sortv3,
    },
];

fn url(parametes: &Parameters) -> String {
    String::from(format!(
        "https://api.polygon.io/v3/trades/{}?{}{}{}{}{}{}apiKey={}",
        parametes.ticker.clone().unwrap(),
        if let Some(t) = parametes.clone().timestamp {
            format!("timestamp={}&", t)
        } else {
            "".to_string()
        },
        if let Some(tf) = parametes.clone().from {
            format!("timestamp.gte={}&", tf)
        } else {
            "".to_string()
        },
        if let Some(tt) = parametes.clone().to {
            format!("timestamp.lte={}&", tt)
        } else {
            "".to_string()
        },
        if let Some(o) = parametes.clone().order {
            format!("order={}&", o)
        } else {
            "".to_string()
        },
        if let Some(l) = parametes.clone().limit {
            format!("limit={}&", l)
        } else {
            "".to_string()
        },
        if let Some(s) = parametes.clone().sortv3 {
            format!("sort={}&", s)
        } else {
            "".to_string()
        },
        parametes.api_key,
    ))
}
