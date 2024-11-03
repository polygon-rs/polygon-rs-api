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

#[test]
fn test_greeks_parse() {
    let data = serde_json::json!({
        "delta": 1.23,
        "gamma": 2.34,
        "theta": 3.45,
        "vega": 4.56
    });
    let greeks = Greeks::parse(&data.as_object().unwrap());
    assert_eq!(greeks.delta.unwrap(), 1.23);
    assert_eq!(greeks.gamma.unwrap(), 2.34);
    assert_eq!(greeks.theta.unwrap(), 3.45);
    assert_eq!(greeks.vega.unwrap(), 4.56);
}

