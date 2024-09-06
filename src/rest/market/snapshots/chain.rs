use crate::{Parameters, ErrorCode};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Chain {
    chain_parameters: Option<Parameters>,
    pub after_hours: Option<f64>,
    pub close: Option<f64>,
    pub from: Option<String>,
    pub high: Option<f64>,
    pub low: Option<f64>,
    pub open: Option<f64>,
    pub pre_market: Option<f64>,
    pub status: Option<String>,
    pub symbol: Option<String>,
    pub volume: Option<f64>,
}

impl Chain {
    #[tokio::main]
    pub async fn chain(&self, p: Parameters) -> Result<Chain, ErrorCode> {
        /*match p.verify_ticker(){
            Ok(t) => t,
            Err(e) =>  return Err(e)
        };*/
        let date = match &p.date {
            Some(d) => d,
            None => return Err(ErrorCode::DateError),
        };
        let url = format!(
            "https://api.polygon.io/v3/snapshot/options/{}?apiKey={}",
            p.ticker.clone().unwrap(), &p.api_key
        );
        /*let result = match p.request(url) 
        {
            Ok(response) => response,
            Err(e) => return Err(ErrorCode::RequestError),
        };*/
        let result = String::from("");
        match serde_json::from_str(result.as_str()) {
            Ok(chain) => Ok(chain),
            Err(e) => return Err(ErrorCode::FormatError),
        }
    }
}
