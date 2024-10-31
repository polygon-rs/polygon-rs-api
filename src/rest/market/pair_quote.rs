use crate::{
    data_types::{quote::Quote, Parse},
    rest::{
        error::ErrorCode,
        parameters::{Parameter, ParameterRequirment, Parameters, TickerTypes},
    },
    tools::{request::Request, verification::Verification},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PairQuote {
    pub request_id: Option<String>,
    pub pair_quote: Option<Quote>,
    pub status: Option<String>,
    pub symbol: Option<String>,
}

impl PairQuoteRequest for PairQuote {}

impl Parse for PairQuote {
    fn parse(map: &serde_json::Map<String, serde_json::Value>) -> Self {
        let request_id = Self::string_parse(map, vec!["request_id"]);
        let pair_quote = Self::object_parse(map, vec!["last"]);
        let status = Self::string_parse(map, vec!["status"]);
        let symbol = Self::string_parse(map, vec!["symbol"]);

        PairQuote {
            request_id,
            pair_quote,
            status,
            symbol,
        }
    }
}

pub trait PairQuoteRequest {
    fn get_pair_quote(
        api_key: String,
        ticker: String,
        request: &impl Request,
        verification: &impl Verification,
    ) -> Result<PairQuote, ErrorCode> {
        let pair_quote_parameters = Parameters {
            api_key: api_key,
            ticker: Some(ticker),
            ..Parameters::default()
        };
        if let Err(check) =
            verification.check_parameters(&TickerTypes::forex(), PARAMETERS, &pair_quote_parameters)
        {
            return Err(check);
        }
        let url = url(&pair_quote_parameters);
        match request.request(url) {
            Ok(mut map) => Ok(PairQuote::parse(&mut map)),
            Err(e) => return Err(e),
        }
    }
}

const PARAMETERS: &'static [&'static ParameterRequirment] = &[&ParameterRequirment {
    required: true,
    parameter: Parameter::Ticker,
}];

fn url(parameters: &Parameters) -> String {
    let from = parameters.ticker.clone().unwrap()[2..4].to_string();
    let to = parameters.ticker.clone().unwrap()[5..7].to_string();
    String::from(format!(
        "https://api.polygon.io/v1/lastquote/currencies/{}/{}?apiKey={}",
        from, to, parameters.api_key,
    ))
}
