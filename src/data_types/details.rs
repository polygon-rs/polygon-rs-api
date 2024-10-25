use crate::rest::parameters::{ContractStyle,ContractType};
use crate::data_types::Parse;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Default)]
pub struct Details {
    pub contract_type: Option<ContractType>,
    pub contract_style: Option<ContractStyle>,
    pub expiration_date: Option<String>,
    pub shares_per_contract: Option<i64>,
    pub strike_price: Option<f64>,
    pub ticker: Option<String>,
}

impl Parse for Details {
    fn parse(map: &serde_json::Map<String, serde_json::Value>) -> Self {
        let contract_type = map.get("contract_type").and_then(|v| v.as_str()).map(|v| match v {
            "Call" => ContractType::Call,
            "Put" => ContractType::Put,
            _ => ContractType::Unknown,
        });
        let contract_style = map.get("contract_style").and_then(|v| v.as_str()).map(|v| match v {
            "American" => ContractStyle::American,
            "European" => ContractStyle::European,
            "Bermudan" => ContractStyle::Bermudan,
            _ => ContractStyle::Unknown,
        });
        let expiration_date = map.get("expiration_date").and_then(|v| v.as_str()).map(|v| v.to_string());
        let shares_per_contract = map.get("shares_per_contract").and_then(|v| v.as_i64());
        let strike_price = map.get("strike_price").and_then(|v| v.as_f64());
        let ticker = map.get("ticker").and_then(|v| v.as_str()).map(|v| v.to_string());
        Details {
            contract_type,
            contract_style,
            expiration_date,
            shares_per_contract,
            strike_price,
            ticker,
        }
    }
}
