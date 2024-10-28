use crate::data_types::{contract::Contract, Parse};
use crate::rest::{
    parameters::{Parameter, ParameterRequirment, Parameters, TickerTypes},
    error::ErrorCode,
};
use crate::tools::{request::Request, verification::Verification};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct OptionsContract {
    pub request_id: Option<String>,
    pub next_url: Option<String>,
    pub contract: Option<Contract>,
    pub status: Option<String>,
}

impl OptionsContract {
    fn next(&mut self, api_key: String, request: &impl Request) -> Result<(), ErrorCode> {
        if self.next_url.is_none() {
            return Err(ErrorCode::NoNextURL);
        }
        let next_url = if let Some(next_url) = &self.next_url {
            format!("{}&apiKey={}",next_url, api_key)
        } else { return Err(ErrorCode::NoNextURL); };
        match request.request(next_url) {
            Ok(mut map) => {*self = OptionsContract::parse(&mut map); Ok(())},
            Err(e) => return Err(e),
        }
    }
}

impl OptionsContractRequest for OptionsContract {}

impl Parse for OptionsContract {
    fn parse(map: &mut serde_json::Map<String, serde_json::Value>) -> Self {
        let request_id = map.get("request_id").unwrap().as_str().unwrap().to_string();
        let next_url = map.get("next_url").unwrap().as_str().unwrap().to_string();
        let status = map.get("status").unwrap().as_str().unwrap().to_string();
        let contract = map
            .get_mut("results")
            .and_then(|v| v.as_object_mut())
            .map(|v| Contract::parse(v));
        OptionsContract {
            request_id: Some(request_id),
            next_url: Some(next_url),
            status: Some(status),
            contract
        }
    }
}

pub trait OptionsContractRequest {
    fn options_contract_request(
        api_key: String,
        ticker: String,
        underlying_asset: String,
        request: &impl Request,
        verification: &impl Verification,
    ) -> Result<OptionsContract, ErrorCode> {
        let options_contract_parameters = Parameters {
            api_key: api_key,
            ticker: Some(ticker),
            underlying_asset: Some(underlying_asset),
            ..Parameters::default()
        };
        if let Err(check) = verification.check_parameters(
            &TickerTypes::options(),
            PARAMETERS,
            &options_contract_parameters,
        ) {
            return Err(check);
        }
        let url = url(&options_contract_parameters);
        match request.request(url) {
            Ok(mut map) => Ok(OptionsContract::parse(&mut map)),
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
        required: true,
        parameter: Parameter::UnderlyingAsset,
    },
];

//Address unwrap and clone
fn url(parameters: &Parameters) -> String {
    String::from(format!(
        "https://api.polygon.io/v3/snapshot/options/{}/{}?apiKey={}",
        parameters.clone().underlying_asset.unwrap(),
        parameters.clone().ticker.unwrap(),
        parameters.api_key,
    ))
}
