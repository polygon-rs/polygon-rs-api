use crate::data_types::{option_contract::OptionContract, Parse};
use crate::rest::{
    error::ErrorCode,
    parameters::{
        ContractType, Order, Parameter, ParameterRequirment, Parameters, Sortv3, TickerTypes,
    },
};
use crate::tools::{request::Request, verification::Verification};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct OptionContracts {
    pub request_id: Option<String>,
    pub next_url: Option<String>,
    pub chain: Option<Vec<OptionContract>>,
    pub status: Option<String>,
}

impl OptionContractsRequest for OptionContracts {}

impl Parse for OptionContracts {
    fn parse(map: &serde_json::Map<String, serde_json::Value>) -> Self {
        let request_id = Self::string_parse(map, vec!["request_id"]);
        let next_url = Self::string_parse(map, vec!["next_url"]);
        let status = Self::string_parse(map, vec!["status"]);
        let chain = Self::array_parse(map, vec!["results"]);
        OptionContracts {
            request_id,
            next_url,
            status,
            chain,
        }
    }
}

pub trait OptionContractsRequest {
    fn get_options_contracts(
        api_key: &String,
        ticker: Option<String>,
        ticker_from: Option<String>,
        ticker_to: Option<String>,
        date: Option<String>,
        from: Option<String>,
        to: Option<String>,
        as_of: Option<String>,
        expired: Option<bool>,
        strike_price: Option<f64>,
        strike_price_from: Option<f64>,
        strike_price_to: Option<f64>,
        contract_type: Option<ContractType>,
        order: Option<Order>,
        limit: Option<u16>,
        sort: Option<Sortv3>,
    ) -> Result<OptionContracts, ErrorCode> {
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
        let ticker = if ticker_from.is_some() || ticker_to.is_some() {
            None
        } else {
            ticker
        };
        let options_chain_parameters = Parameters {
            api_key: api_key.to_string(),
            ticker: ticker,
            ticker_from: ticker_from,
            ticker_to: ticker_to,
            date: ts,
            from: from,
            to: to,
            contract_type: contract_type,
            as_of: as_of,
            expired: expired,
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
            Ok(mut map) => Ok(OptionContracts::parse(&mut map)),
            Err(e) => return Err(e),
        }
    }
}

const PARAMETERS: &'static [&'static ParameterRequirment] = &[
    &ParameterRequirment {
        required: false,
        parameter: Parameter::Ticker,
    },
    &ParameterRequirment {
        required: false,
        parameter: Parameter::TickerFrom,
    },
    &ParameterRequirment {
        required: false,
        parameter: Parameter::TickerTo,
    },
    &ParameterRequirment {
        required: false,
        parameter: Parameter::AsOf,
    },
    &ParameterRequirment {
        required: false,
        parameter: Parameter::Expired,
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
        "https://api.polygon.io/v3/reference/options/contracts?{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}apiKey={}",
        if let Some(ticker) = &parameters.ticker {
            format!("ticker={}&", ticker)
        } else {
            "".to_string()
        },
        if let Some(tf) = &parameters.ticker_from {
            format!("ticker.gte={}&", tf)
        } else {
            "".to_string()
        },
        if let Some(tt) = &parameters.ticker_to {
            format!("ticker.lte={}&", tt)
        } else {
            "".to_string()
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
        if let Some(as_of) = &parameters.as_of {
            format!("as_of={}&", as_of)
        } else {
            "".to_string()
        },
        if let Some(expired) = &parameters.expired {
            format!("expired={}&", expired)
        } else {
            "".to_string()
        },
        if let Some(contract_type) = &parameters.contract_type {
            format!("contract_type={}&", contract_type.to_string().to_lowercase())
        } else {
            "".to_string()
        },
        if let Some(order) = &parameters.order {
            format!("order={}&", order.to_string().to_lowercase())
        } else {
            "".to_string()
        },
        if let Some(limit) = &parameters.limit {
            format!("limit={}&", limit)
        } else {
            "".to_string()
        },
        if let Some(sort) = &parameters.sortv3 {
            match sort {
                Sortv3::ExpirationDate => format!("sort=expiration_date&"),
                Sortv3::StrikePrice => format!("sort=strike_price&"),
                _ => format!("sort={}&", sort.to_string().to_lowercase())
            }
        } else {
            "".to_string()
        },
        &parameters.api_key,
    ));
    Ok(url)
}
