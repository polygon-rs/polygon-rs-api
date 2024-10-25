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
    fn parse(map: &mut serde_json::Map<String, serde_json::Value>) -> Self {
        let delta = map.get("delta").and_then(|v| v.as_f64());
        let gamma = map.get("gamma").and_then(|v| v.as_f64());
        let theta = map.get("theta").and_then(|v| v.as_f64());
        let vega = map.get("vega").and_then(|v| v.as_f64());
        Self {
            delta,
            gamma,
            theta,
            vega,
        }
    }
}
