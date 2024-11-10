use crate::data_types::{Parse, additional_underlying::AdditionalUnderlying};
use crate::rest::parameters::ContractStyle;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OptionContract {
    pub additional_underlyings: Option<Vec<AdditionalUnderlying>>,
    pub cfi: Option<String>,
    pub contract: Option<String>,
    pub correction: Option<i64>,
    pub exercise_style: Option<ContractStyle>,
    pub expiration_date: Option<String>,
    pub primary_exchange: Option<String>,
    pub shares_per_contract: Option<f64>,
    pub strike_price: Option<f64>,
    pub ticker: Option<String>,
    pub underlying_ticker: Option<String>,
}

impl Parse for OptionContract {
    fn parse(map: &serde_json::Map<String, serde_json::Value>) -> Self {
        let additional_underlyings = Self::array_parse(map, vec!["additional_underlyings"]);
        let cfi = Self::string_parse(map, vec!["cfi"]);
        let contract = Self::string_parse(map, vec!["contract"]);
        let correction = Self::i64_parse(map, vec!["correction"]);
        let contract_style = match Self::string_parse(map, vec!["contract_type"]) {
            Some(contract_style) => match contract_style.as_str() {
                "American" => Some(ContractStyle::American),
                "European" => Some(ContractStyle::European),
                "Bermudan" => Some(ContractStyle::Bermudan),
                _ => None,
            },
            None => None,
        };
        let expiration_date = Self::string_parse(map, vec!["expiration_date"]);
        let primary_exchange = Self::string_parse(map, vec!["primary_exchange"]);
        let shares_per_contract = Self::f64_parse(map, vec!["shares_per_contract"]);
        let strike_price = Self::f64_parse(map, vec!["strike_price"]);
        let ticker = Self::string_parse(map, vec!["ticker"]);
        let underlying_ticker = Self::string_parse(map, vec!["underlying_ticker"]);

        OptionContract {
            additional_underlyings,
            cfi,
            contract,
            correction,
            exercise_style: contract_style,
            expiration_date,
            primary_exchange,
            shares_per_contract,
            strike_price,
            ticker,
            underlying_ticker,
        }
    }
}
