use crate::{ErrorCode, Order, Parameter, ParameterRequirment, Parameters, Request, Sortv3};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Quotes {
    quotes_parameters: Parameters,
    quotes_url: String,
    pub next_url: String,
    pub request_id: String,
    pub results: Vec<Quote>,
    pub status: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Quote {
    pub ask_exchange: i32,
    pub ask_price: f64,
    pub ask_size: i32,
    pub bid_exchange: i32,
    pub bid_price: f64,
    pub bid_size: i32,
    pub conditions: Vec<i32>,
    pub participant_timestamp: i64,
    pub sequence_number: i64,
    pub sip_timestamp: i64,
    pub tape: i32,
}

impl Quotes {
    pub fn set_parameters(
        &mut self,
        api_key: String,
        ticker: String,
        timestamp: Option<String>,
        from: Option<String>,
        to: Option<String>,
        sort: Option<Sortv3>,
        limit: Option<u16>,
        order: Option<Order>,
    ) {
        self.quotes_parameters = Parameters {
            api_key: api_key,
            ticker: Some(ticker),
            timestamp: timestamp,
            from: from,
            to: to,
            sortv3: sort,
            limit: limit,
            order: order,
            ..Parameters::default()
        }
    }
}

impl Request for Quotes {
    const VERSION: &'static str = "v3";
    const CALL: &'static str = "quotes";
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

    fn parameters(&self) -> &Parameters {
        &self.quotes_parameters
        /*match &self.quotes_parameters {
            Some(p) => p,
            None => panic!("There is no parameters set"),
        }*/
    }

    fn url(&mut self) -> &String {
        &self.quotes_url
        /*self.set_url();
        match &self.quotes_url {
            Some(u) => u.to_string(),
            None => panic!("There is no url set"),
        }*/
    }

    fn set_url(&mut self) -> Result<(), ErrorCode> {
        if let Err(check) = self.check_parameters() {
            return Err(check);
        }
        self.quotes_url = String::from(format!(
            "{}/{}/{}/{}?{}{}{}{}{}{}apiKey={}",
            Self::BASE_URL,
            Self::VERSION,
            Self::CALL,
            self.parameters().clone().ticker.unwrap(),
            if let Some(t) = self.parameters().clone().timestamp {
                format!("timestamp={}&", t)
            } else {
                "".to_string()
            },
            if let Some(tf) = self.parameters().clone().from {
                format!("timestamp.gte={}&", tf)
            } else {
                "".to_string()
            },
            if let Some(tt) = self.parameters().clone().to {
                format!("timestamp.lte={}&", tt)
            } else {
                "".to_string()
            },
            if let Some(o) = self.parameters().clone().order {
                format!("order={}&", o)
            } else {
                "".to_string()
            },
            if let Some(l) = self.parameters().clone().limit {
                format!("limit={}&", l)
            } else {
                "".to_string()
            },
            if let Some(s) = self.parameters().clone().sortv3 {
                format!("sort={}&", s)
            } else {
                "".to_string()
            },
            self.parameters().clone().api_key,
        ));
        Ok(())
    }

    fn request(&mut self) -> Result<(), ErrorCode> {
        if let Err(check) = self.set_url() {
            return Err(check);
        }
        let r = match self.get_raw_data() {
            Ok(response) => response,
            Err(e) => return Err(e),
        };
        let a: Quotes = match serde_json::from_str(r.as_str()) {
            Ok(it) => it,
            Err(err) => return Err(ErrorCode::FormatError),
        };
        *self = a;

        Ok(())
    }
}
