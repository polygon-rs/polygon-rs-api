use crate::data_types::{contract::Contract, Parse};
use crate::rest::{
    error::ErrorCode,
    parameters::{Parameter, ParameterRequirment, Parameters, TickerTypes},
};
use crate::tools::{request::Request, verification::Verification};

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

pub trait OptionsContractRequest {
    fn options_contract_request(
        
        api_key: &String,
        ticker: String,
        underlying_asset: String,
    ) -> Result<OptionsContract, ErrorCode> {
        let options_contract_parameters = Parameters {
            api_key: api_key.to_string(),
            ticker: Some(ticker),
            underlying_asset: Some(underlying_asset),
            ..Parameters::default()
        };
        if let Err(check) = Verification::check_parameters(
            &TickerTypes::options(),
            PARAMETERS,
            &options_contract_parameters,
        ) {
            return Err(check);
        }
        let url = match url(&options_contract_parameters){
            Ok(url) => url,
            Err(e) => return Err(e)
        };
        match Request::request(url) {
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

fn url(parameters: &Parameters) -> Result<String, ErrorCode> {
    let url = String::from(format!(
        "https://api.polygon.io/v3/snapshot/options/{}/{}?apiKey={}",
        match &parameters.underlying_asset{
            Some(asset) => asset,
            None => return Err(ErrorCode::UnderlyingAssetNotSet)
        },
        match &parameters.ticker{
            Some(ticker) => ticker,
            None => return Err(ErrorCode::TickerNotSet)
        },
        &parameters.api_key,
    ));
    Ok(url)
}

#[test]
fn test_options_contract_parse() {
    let data = serde_json::json!({
        "request_id": "req12345",
        "status": "OK",
        "contract": {
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
    });
    let options_contract = OptionsContract::parse(&data.as_object().unwrap());
    assert_eq!(options_contract.request_id.unwrap(), "req12345");
    assert_eq!(options_contract.status.unwrap(), "OK");
    assert_eq!(options_contract.contract.unwrap().break_even_price.unwrap(), 1.23);
}

#[test]
fn test_url() {
    let mut parameters = Parameters::default();
    parameters.api_key = String::from("apiKey");
    parameters.ticker = Some(String::from("O:AAL210820C00014000"));
    parameters.underlying_asset = Some(String::from("AAL"));
    let url = url(&parameters).unwrap();
    assert_eq!(url, "https://api.polygon.io/v3/snapshot/options/AAL/O:AAL210820C00014000?apiKey=apiKey");
}
