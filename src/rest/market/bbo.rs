use crate::{
    data_types::{quote::Quote, Parse},
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

impl Next for BBO {}

pub trait BBORequest {
    fn get_bbo(
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
    ) -> Result<BBO, ErrorCode> {
        let ts = if to.is_some() || from.is_some() {
            None
        } else {
            timestamp
        };
        let bbo_parameters = Parameters {
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
        if let Err(check) =
            verification.check_parameters(&TickerTypes::forex(), PARAMETERS, &bbo_parameters)
        {
            return Err(check);
        }
        let url = url(&bbo_parameters);
        match request.request(url) {
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

fn url(parameters: &Parameters) -> String {
    String::from(format!(
        "https://api.polygon.io/v3/quotes/{}?{}{}{}{}{}{}apiKey={}",
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
        if let Some(s) = parameters.clone().sortv3 {
            format!("sort={}&", s)
        } else {
            "".to_string()
        },
        parameters.api_key,
    ))
}
