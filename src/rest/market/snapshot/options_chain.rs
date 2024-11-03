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
        
        api_key: &String,
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
            api_key: api_key.to_string(),
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

#[test]
fn test_options_chain_parse() {
    let data = serde_json::json!({
        "request_id": "req12345",
        "next_url": "https://api.polygon.io/v3/snapshot/options/AAPL?cursor=YWN0aXZlPXRydWUmZGF0ZT0yMDIzLTA0LTAxJmxpbWl0PTEmb3JkZXI9YXNjJnBhZ2VfbWFya2VyPUElMjBWU1MjQyMCU3QzIwMjMtMDQtMDElN0M5JTNBNDElN0MwMCUzQTAwJnNvcnQ9dGlja2Vy",
        "status": "OK",
        "results": [
            {
                "break_even_price": 1.23,
                "day": {
                    "change": 1.0,
                    "change_percent": 2.0,
                    "close": 3.0,
                    "high": 4.0,
                    "last_updated": 164545545,
                    "low": 5.0,
                    "open": 6.0,
                    "previous_close": 7.0,
                    "volume": 8,
                    "volume_weighted_average_price": 9.0,
                    "otc": false
                },
                "details": {
                    "contract_type": "Call",
                    "contract_style": "American",
                    "expiration_date": "2023-03-03",
                    "shares_per_contract": 100,
                    "strike_price": 10.0,
                    "ticker": "TEST"
                },
                "fair_market_value": 11.0,
                "greeks": {
                    "delta": 12.0,
                    "gamma": 13.0,
                    "theta": 14.0,
                    "vega": 15.0
                },
                "implied_volatility": 16.0,
                "quote": {
                    "bid": 17.0,
                    "bid_size": 18,
                    "ask": 19.0,
                    "ask_size": 20,
                    "bid_exchange_id": 21,
                    "ask_exchange_id": 22,
                    "last_updated": 164545546,
                    "mid_point": 23.0,
                    "timeframe": "DELAYED",
                    "exchange_id": 24,
                    "exchange": "TEST",
                    "conditions": [
                        25
                    ],
                    "trf_timestamp": 164545547,
                    "indicators": [
                        26
                    ],
                    "sequence_number": 27,
                    "participant_timestamp": 164545548,
                    "tape": 28
                },
                "trade": {
                    "conditions": [
                        29
                    ],
                    "exchange_id": 30,
                    "price": 31.0,
                    "sip_timestamp": 164545549,
                    "size": 32,
                    "trade_id": "trade",
                    "timeframe": "REAL-TIME",
                    "exchange": "TEST1",
                    "trade_correction": 33,
                    "trf_timestamp": 164545550,
                    "sequence_number": 34,
                    "trf_id": 35,
                    "participant_timestamp": 164545551,
                    "tape": 36
                },
                "open_interest": 37,
                "underlying_asset": {
                    "change_to_break_even": 38.0,
                    "last_updated": 164545552,
                    "price": 39.0,
                    "ticker": "TEST2",
                    "timeframe": "REAL-TIME",
                    "value": 40.0
                }
            }
        ]
    });
    let options_chain = OptionsChain::parse(&data.as_object().unwrap());
    assert_eq!(options_chain.request_id.unwrap(), "req12345");
    assert_eq!(options_chain.next_url.unwrap(), "https://api.polygon.io/v3/snapshot/options/AAPL?cursor=YWN0aXZlPXRydWUmZGF0ZT0yMDIzLTA0LTAxJmxpbWl0PTEmb3JkZXI9YXNjJnBhZ2VfbWFya2VyPUElMjBWU1MjQyMCU3QzIwMjMtMDQtMDElN0M5JTNBNDElN0MwMCUzQTAwJnNvcnQ9dGlja2Vy");
    assert_eq!(options_chain.status.unwrap(), "OK");
    assert_eq!(options_chain.chain.unwrap()[0].break_even_price.unwrap(), 1.23);
}

#[test]
fn test_url() {
    let mut parameters = Parameters::default();
    parameters.api_key = String::from("apiKey");
    parameters.underlying_asset = Some(String::from("AAPL"));
    parameters.strike_price = Some(100.0);
    parameters.strike_price_from = Some(90.0);
    parameters.strike_price_to = Some(110.0);
    parameters.date = Some(String::from("2023-04-01"));
    parameters.from = Some(String::from("2023-03-01"));
    parameters.to = Some(String::from("2023-05-01"));
    parameters.contract_type = Some(ContractType::Call);
    parameters.order = Some(Order::Asc);
    parameters.limit = Some(1);
    parameters.sortv3 = Some(Sortv3::ExpirationDate);
    let url = url(&parameters).unwrap();
    assert_eq!(url, "https://api.polygon.io/v3/snapshot/options/AAPL?strike_price=100&strike_price.gte=90&strike_price.lte=110&expiration_date=2023-04-01&expiration_date.gte=2023-03-01&expiration_date.lte=2023-05-01&contract_type=call&order=asc&limit=1&sort=expiration_date&apiKey=apiKey");
}
