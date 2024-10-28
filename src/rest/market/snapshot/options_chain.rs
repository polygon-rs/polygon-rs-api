use crate::data_types::{contract::Contract, Parse};
use crate::rest::{
    parameters::{
        ContractType, Order, Parameter, ParameterRequirment, Parameters, Sortv3, TickerTypes,
    },
    error::ErrorCode,
};
use crate::tools::{request::{Next, Request}, verification::Verification};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct OptionsChain {
    pub request_id: Option<String>,
    pub next_url: Option<String>,
    pub chain: Option<Vec<Contract>>,
    pub status: Option<String>,
}

impl OptionsChainRequest for OptionsChain {}

impl Parse for OptionsChain {
    fn parse(map: &mut serde_json::Map<String, serde_json::Value>) -> Self {
        let request_id = map
            .get("request_id")
            .and_then(|v| v.as_str())
            .map(|v| v.to_string());
        let next_url = map
            .get("next_url")
            .and_then(|v| v.as_str())
            .map(|v| v.to_string());
        let status = map
            .get("status")
            .and_then(|v| v.as_str())
            .map(|v| v.to_string());
        let chain = map
            .get_mut("results")
            .and_then(|v| v.as_array_mut())
            .map(|v| {
                let mut contracts = Vec::new();
                for contract in v {
                    if let Some(c) = contract.as_object_mut().map(|v| Contract::parse(v)) {
                        contracts.push(c);
                    }
                }
                contracts
            });
        OptionsChain {
            request_id: request_id,
            next_url: next_url,
            status: status,
            chain,
        }
    }
}

impl Next for OptionsChain {}

pub trait OptionsChainRequest {
    fn get_options_chain(
        api_key: String,
        underlying_asset: String,
        date: Option<String>,
        from: Option<String>,
        to: Option<String>,
        strike_price: Option<f64>,
        strike_price_from: Option<f64>,
        strike_price_to: Option<f64>,
        contract_type: Option<ContractType>,
        order: Option<Order>,
        limit: Option<u16>,
        sort: Option<Sortv3>,
        request: &impl Request,
        verification: &impl Verification,
    ) -> Result<OptionsChain, ErrorCode> {
        let ts = if from.is_some() || from.is_some() {
            None
        } else {
            date
        };
        let sp = if strike_price_from.is_some() || strike_price_to.is_some() {
            None
        } else {
            strike_price
        };
        let options_chain_parameters = Parameters {
            api_key: api_key,
            underlying_asset: Some(underlying_asset),
            date: ts,
            from: from,
            to: to,
            contract_type: contract_type,
            order: order,
            limit: limit,
            sortv3: sort,
            strike_price: sp,
            strike_price_from: strike_price_from,
            strike_price_to: strike_price_to,
            ..Parameters::default()
        };
        if let Err(check) = verification.check_parameters(
            &TickerTypes::options(),
            PARAMETERS,
            &options_chain_parameters,
        ) {
            return Err(check);
        }
        let url = url(&options_chain_parameters);
        match request.request(url) {
            Ok(mut map) => Ok(OptionsChain::parse(&mut map)),
            Err(e) => return Err(e),
        }
    }
}

const PARAMETERS: &'static [&'static ParameterRequirment] = &[
    &ParameterRequirment {
        required: true,
        parameter: Parameter::UnderlyingAsset,
    },
    &ParameterRequirment {
        required: false,
        parameter: Parameter::StrikePrice,
    },
    &ParameterRequirment {
        required: false,
        parameter: Parameter::StrikePriceFrom,
    },
    &ParameterRequirment {
        required: false,
        parameter: Parameter::StrikePriceTo,
    },
    &ParameterRequirment {
        required: false,
        parameter: Parameter::Date,
    },
    &ParameterRequirment {
        required: false,
        parameter: Parameter::To,
    },
    &ParameterRequirment {
        required: false,
        parameter: Parameter::From,
    },
    &ParameterRequirment {
        required: false,
        parameter: Parameter::ContractType,
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

fn url(parameters: &Parameters) -> String {
    String::from(format!(
        "https://api.polygon.io/v3/snapshot/options/{}?{}{}{}{}{}{}{}{}{}{}apiKey={}",
        parameters.clone().underlying_asset.unwrap(),
        if let Some(strike_price) = parameters.clone().strike_price {
            format!("strike_price={}&", strike_price)
        } else {
            "".to_string()
        },
        if let Some(strike_price_from) = parameters.clone().strike_price_from {
            format!("strike_price.gte={}&", strike_price_from)
        } else {
            "".to_string()
        },
        if let Some(strike_price_to) = parameters.clone().strike_price_to {
            format!("strike_price.lte={}&", strike_price_to)
        } else {
            "".to_string()
        },
        if let Some(date) = parameters.clone().date {
            format!("expiration_date={}&", date)
        } else {
            "".to_string()
        },
        if let Some(from) = parameters.clone().from {
            format!("expiration_date.gte={}&", from)
        } else {
            "".to_string()
        },
        if let Some(to) = parameters.clone().to {
            format!("expiration_date.lte={}&", to)
        } else {
            "".to_string()
        },
        if let Some(contract_type) = parameters.clone().contract_type {
            format!("contract_type={}&", contract_type)
        } else {
            "".to_string()
        },
        if let Some(order) = parameters.clone().order {
            format!("order={}&", order)
        } else {
            "".to_string()
        },
        if let Some(limit) = parameters.clone().limit {
            format!("limit={}&", limit)
        } else {
            "".to_string()
        },
        if let Some(sort) = parameters.clone().sortv3 {
            format!("sort={}&", sort)
        } else {
            "".to_string()
        },
        parameters.clone().api_key,
    ))
}
