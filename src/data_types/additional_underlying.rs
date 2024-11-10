use crate::data_types::Parse;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AdditionalUnderlying {
    pub amount: Option<f64>,
    pub assest: Option<String>,
    pub underlying: Option<String>,
}

impl Parse for AdditionalUnderlying {
    fn parse(map: &serde_json::Map<String, serde_json::Value>) -> Self {
        let amount = Self::f64_parse(map, vec!["a"]);
        let asset = Self::string_parse(map, vec!["A"]);
        let underlying = Self::string_parse(map, vec!["u"]);
        AdditionalUnderlying {
            amount,
            assest: asset,
            underlying,
        }
    }
}
