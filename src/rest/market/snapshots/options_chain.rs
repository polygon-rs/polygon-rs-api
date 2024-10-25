use crate::data_types::{contract::Contract, Parse};
use crate::rest::{
    parameters::{
        ContractType, Order, Parameter, ParameterRequirment, Parameters, Sortv3, TickerTypes,
    },
    ErrorCode, Request,
};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Default)]
pub struct Options Chain {
    chain_parameters: Parameters,
    chain_url: String,
    pub request_id: Option<String>,
    pub next_url: Option<String>,
    pub chain: Option<Vec<Contract>>,
    pub status: Option<String>,
}

impl Chain {
    pub fn set_parameters(
        &mut self,
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
    ) {
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
        self.chain_parameters = Parameters {
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
        }
    }
}


impl Parse for Chain{
    fn parse(map: &serde_json::Map<String, serde_json::Value>) -> Self {
        let request_id = map.get("request_id").unwrap().as_str().unwrap().to_string();
        let next_url = map.get("next_url").unwrap().as_str().unwrap().to_string();
        let status = map.get("status").unwrap().as_str().unwrap().to_string();
        let chain = map.get("results").and_then(|v| v.as_array()).map(|v| {
            let mut contracts = Vec::new();
            for contract in v {
                if let Some(c) = contract.as_object().map(|v| Contract::parse(v)) {
                    contracts.push(c);
                }
            }
            contracts
        });
        Chain {
            request_id: Some(request_id),
            next_url: Some(next_url),
            status: Some(status),
            chain,
            ..Chain::default()
        }
    }
}

impl Request for Chain {
    const VERSION: &'static str = "v3";
    const CALL: &'static str = "snapshot/options";
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

    fn parameters(&self) -> &Parameters {
        &self.chain_parameters
    }

    fn url(&mut self) -> &String {
        &self.chain_url
    }

    fn set_url(&mut self) -> Result<(), ErrorCode> {
        if let Err(check) = self.check_parameters(&TickerTypes::options()) {
            return Err(check);
        }
        self.chain_url = String::from(format!(
            "{}/{}/{}/{}?{}{}{}{}{}{}{}{}{}{}apiKey={}",
            Self::BASE_URL,
            Self::VERSION,
            Self::CALL,
            self.parameters().clone().underlying_asset.unwrap(),
            if let Some(strike_price) = self.parameters().clone().strike_price {
                format!("strike_price={}&", strike_price)
            } else {
                "".to_string()
            },
            if let Some(strike_price_from) = self.parameters().clone().strike_price_from {
                format!("strike_price.gte={}&", strike_price_from)
            } else {
                "".to_string()
            },
            if let Some(strike_price_to) = self.parameters().clone().strike_price_to {
                format!("strike_price.lte={}&", strike_price_to)
            } else {
                "".to_string()
            },
            if let Some(date) = self.parameters().clone().date {
                format!("expiration_date={}&", date)
            } else {
                "".to_string()
            },
            if let Some(from) = self.parameters().clone().from {
                format!("expiration_date.gte={}&", from)
            } else {
                "".to_string()
            },
            if let Some(to) = self.parameters().clone().to {
                format!("expiration_date.lte={}&", to)
            } else {
                "".to_string()
            },
            if let Some(contract_type) = self.parameters().clone().contract_type {
                format!("contract_type={}&", contract_type)
            } else {
                "".to_string()
            },
            if let Some(order) = self.parameters().clone().order {
                format!("order={}&", order)
            } else {
                "".to_string()
            },
            if let Some(limit) = self.parameters().clone().limit {
                format!("limit={}&", limit)
            } else {
                "".to_string()
            },
            if let Some(sort) = self.parameters().clone().sortv3 {
                format!("sort={}&", sort)
            } else {
                "".to_string()
            },
            self.parameters().clone().api_key,
        ));
        Ok(())
    }


    fn request(&mut self) -> Result<(), ErrorCode> {
        match self.polygon_request() {
            Ok(response) => {
                *self = Self::parse(&response);
            }
            Err(e) => return Err(e),
        }
        Ok(())
    }
}
