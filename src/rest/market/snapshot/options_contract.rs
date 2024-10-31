use crate::data_types::{contract::Contract, Parse};
use crate::rest::{
    error::ErrorCode,
    parameters::{Parameter, ParameterRequirment, Parameters, TickerTypes},
};
use crate::tools::{
    request::{Next, Request},
    verification::Verification,
};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct OptionsContract {
    pub request_id: Option<String>,
    pub next_url: Option<String>,
    pub contract: Option<Contract>,
    pub status: Option<String>,
}

impl OptionsContractRequest for OptionsContract {}

impl Parse for OptionsContract {
    fn parse(map: &serde_json::Map<String, serde_json::Value>) -> Self {
        let request_id = Self::string_parse(map, vec!["request_id"]);
        let next_url = Self::string_parse(map, vec!["next_url"]);
        let status = Self::string_parse(map, vec!["status"]);
        let contract = Self::object_parse(map, vec!["contract"]);
        OptionsContract {
            request_id,
            next_url,
            status,
            contract,
        }
    }
}

impl Next for OptionsContract {}

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
