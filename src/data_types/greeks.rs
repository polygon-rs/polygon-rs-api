use crate::data_types::Parse;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Greeks {
    pub delta: Option<f64>,
    pub gamma: Option<f64>,
    pub theta: Option<f64>,
    pub vega: Option<f64>,
}

impl Parse for Greeks {
    fn parse(map: &serde_json::Map<String, serde_json::Value>) -> Self {
        let delta = Self::f64_parse(map, vec!["delta"]);
        let gamma = Self::f64_parse(map, vec!["gamma"]);
        let theta = Self::f64_parse(map, vec!["theta"]);
        let vega = Self::f64_parse(map, vec!["vega"]);
        Self {
            delta,
            gamma,
            theta,
            vega,
        }
    }
}
