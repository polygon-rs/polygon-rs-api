use crate::data_types::{contract::Contract, Parse};
use crate::rest::{
    error::ErrorCode,
    parameters::{
        ContractType, Order, Parameter, ParameterRequirment, Parameters, Sortv3, TickerTypes,
    },
};
use crate::tools::{request::Request, verification::Verification};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct OptionsChain {
    pub request_id: Option<String>,
    pub next_url: Option<String>,
    pub chain: Option<Vec<Contract>>,
    pub status: Option<String>,
}

impl OptionsChainRequest for OptionsChain {}

impl Parse for OptionsChain {
    fn parse(map: &serde_json::Map<String, serde_json::Value>) -> Self {
        let request_id = Self::string_parse(map, vec!["request_id"]);
        let next_url = Self::string_parse(map, vec!["next_url"]);
        let status = Self::string_parse(map, vec!["status"]);
        let chain = Self::array_parse(map, vec!["results"]);
        OptionsChain {
            request_id,
            next_url,
            status,
            chain,
        }
    }
}

pub trait OptionsChainRequest {
    fn get_options_chain(
        &self,
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
        if let Err(check) = Verification::check_parameters(
            &TickerTypes::options(),
            PARAMETERS,
            &options_chain_parameters,
        ) {
            return Err(check);
        }
        let url = match url(&options_chain_parameters){
            Ok(url) => url,
            Err(e) => return Err(e)
        };
        match Request::request(url) {
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

fn url(parameters: &Parameters) -> Result<String, ErrorCode> {
    let url = String::from(format!(
        "https://api.polygon.io/v3/snapshot/options/{}?{}{}{}{}{}{}{}{}{}{}apiKey={}",
        match &parameters.underlying_asset {
            Some(underlying_asset) => underlying_asset,
            None => return Err(ErrorCode::UnderlyingAssetNotSet),
        },
        if let Some(strike_price) = &parameters.strike_price {
            format!("strike_price={}&", strike_price)
        } else {
            "".to_string()
        },
        if let Some(strike_price_from) = &parameters.strike_price_from {
            format!("strike_price.gte={}&", strike_price_from)
        } else {
            "".to_string()
        },
        if let Some(strike_price_to) = &parameters.strike_price_to {
            format!("strike_price.lte={}&", strike_price_to)
        } else {
            "".to_string()
        },
        if let Some(date) = &parameters.date {
            format!("expiration_date={}&", date)
        } else {
            "".to_string()
        },
        if let Some(from) = &parameters.from {
            format!("expiration_date.gte={}&", from)
        } else {
            "".to_string()
        },
        if let Some(to) = &parameters.to {
            format!("expiration_date.lte={}&", to)
        } else {
            "".to_string()
        },
        if let Some(contract_type) = &parameters.contract_type {
            format!("contract_type={}&", contract_type)
        } else {
            "".to_string()
        },
        if let Some(order) = &parameters.order {
            format!("order={}&", order)
        } else {
            "".to_string()
        },
        if let Some(limit) = &parameters.limit {
            format!("limit={}&", limit)
        } else {
            "".to_string()
        },
        if let Some(sort) = &parameters.sortv3 {
            format!("sort={}&", sort)
        } else {
            "".to_string()
        },
        &parameters.api_key,
    ));
    Ok(url)
}
