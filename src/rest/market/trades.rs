use crate::{
    ErrorCode, Order, Parameter, ParameterRequirment, Parameters, Request, Sortv3};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Trades {
    trades_parameters: Parameters,
    trades_url: String,
    pub next_url: String,
    pub request_id: String,
    pub results: Vec<Trade>,
    pub status: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Trade {
    pub conditions: Vec<i32>,
    pub exchange: i32,
    pub id: i32,
    pub participant_timestamp: i64,
    pub price: f64,
    pub sequence_number: i64,
    pub sip_timestamp: i64,
    pub size: i32,
    pub tape: i32,
}

impl Trades {
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
        self.trades_parameters = Parameters {
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

impl Request for Trades {
    const VERSION: &'static str = "v3";
    const CALL: &'static str = "trades";
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
        &self.trades_parameters
        /*match &self.trades_parameters {
            Some(p) => p,
            None => panic!("There is no parameters set"),
        }*/
    }

    fn url(&mut self) -> &String {
        &self.trades_url
        /*self.set_url();
        match &self.trades_url {
            Some(u) => u.to_string(),
            None => panic!("There is no url set"),
        }*/
    }

    fn set_url(&mut self) -> Result<(), ErrorCode> {
        if let Err(check) = self.check_parameters() { return Err(check)}
        self.trades_url = String::from(format!(
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
        if let Err(check) = self.set_url() { return Err(check)}
        let r = match self.get_raw_data() {
            Ok(response) => response,
            Err(e) => return Err(e),
        };
        let a: Trades = match serde_json::from_str(r.as_str()) {
            Ok(it) => it,
            Err(err) => return Err(ErrorCode::FormatError),
        };
        *self = a;

        Ok(())
    }
}
