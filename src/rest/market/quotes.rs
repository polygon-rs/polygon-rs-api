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
pub struct Quotes {
    pub next_url: Option<String>,
    pub request_id: Option<String>,
    pub quotes: Option<Vec<Quote>>,
    pub status: Option<String>,
}

impl QuotesRequest for Quotes {}

impl Parse for Quotes {
    fn parse(map: &serde_json::Map<String, serde_json::Value>) -> Self {
        let request_id = Self::string_parse(map, vec!["request_id"]);
        let next_url: Option<String> = Self::string_parse(map, vec!["next_url"]);
        let quotes = Self::array_parse(map, vec!["results"]);
        let status = Self::string_parse(map, vec!["status"]);

        Quotes {
            request_id,
            next_url,
            quotes,
            status,
        }
    }
}

pub trait QuotesRequest {
    fn get_quotes(
        &self,
        api_key: String,
        ticker: String,
        timestamp: Option<String>,
        from: Option<String>,
        to: Option<String>,
        sort: Option<Sortv3>,
        limit: Option<u16>,
        order: Option<Order>,
    ) -> Result<Quotes, ErrorCode> {
        let ts = if to.is_some() || from.is_some() {
            None
        } else {
            timestamp
        };
        let quotes_parameters = Parameters {
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
        if let Err(check) = Verification::check_parameters(
            &TickerTypes::set(true, true, false, false, false),
            PARAMETERS,
            &quotes_parameters,
        ) {
            return Err(check);
        }
        let url = match url(&quotes_parameters){
            Ok(url) => url,
            Err(e) => return Err(e),
        };
        match Request::request(url) {
            Ok(mut map) => Ok(Quotes::parse(&mut map)),
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
            format!("order={}&", o)
        } else {
            "".to_string()
        },
        if let Some(l) = &parameters.limit {
            format!("limit={}&", l)
        } else {
            "".to_string()
        },
        if let Some(s) = &parameters.sortv3 {
            format!("sort={}&", s)
        } else {
            "".to_string()
        },
        &parameters.api_key,
    ));
    Ok(url)
}
