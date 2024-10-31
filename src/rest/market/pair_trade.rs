use crate::{
    data_types::{trade::Trade, Parse},
    rest::{
        error::ErrorCode,
        parameters::{Parameter, ParameterRequirment, Parameters, TickerTypes},
    },
    tools::{request::Request, verification::Verification},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PairTrade {
    pub request_id: Option<String>,
    pub pair_trade: Option<Trade>,
    pub status: Option<String>,
    pub symbol: Option<String>,
}

impl PairTradeRequest for PairTrade {}

impl Parse for PairTrade {
    fn parse(map: &serde_json::Map<String, serde_json::Value>) -> Self {
        let request_id = Self::string_parse(map, vec!["request_id"]);
        let pair_trade = Self::object_parse(map, vec!["last"]);
        let status = Self::string_parse(map, vec!["status"]);
        let symbol = Self::string_parse(map, vec!["symbol"]);

        PairTrade {
            request_id,
            pair_trade,
            status,
            symbol,
        }
    }
}

pub trait PairTradeRequest {
    fn set_parameters(
        api_key: String,
        ticker: String,
        request: &impl Request,
        verification: &impl Verification,
    ) -> Result<PairTrade, ErrorCode> {
        let pair_trade_parameters = Parameters {
            api_key: api_key,
            ticker: Some(ticker),
            ..Parameters::default()
        };
        if let Err(check) = verification.check_parameters(
            &&TickerTypes::crypto(),
            PARAMETERS,
            &pair_trade_parameters,
        ) {
            return Err(check);
        }
        let url = url(&pair_trade_parameters);
        match request.request(url) {
            Ok(mut map) => Ok(PairTrade::parse(&mut map)),
            Err(e) => return Err(e),
        }
    }
}

const PARAMETERS: &'static [&'static ParameterRequirment] = &[&ParameterRequirment {
    required: true,
    parameter: Parameter::Ticker,
}];

fn url(parameters: &Parameters) -> String {
    //Need a different method to extract to and from as Crypto can be different lengths
    let from = parameters.ticker.clone().unwrap()[2..4].to_string();
    let to = parameters.ticker.clone().unwrap()[5..7].to_string();
    String::from(format!(
        "https://api.polygon.io/v1/last/crypto/{}/{}?apiKey={}",
        from,
        to,
        parameters.clone().api_key,
    ))
}
