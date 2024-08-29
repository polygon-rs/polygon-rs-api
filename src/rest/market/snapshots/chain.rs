use crate::{polygon::error::ErrorCode, Polygon};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Chain {
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
    pub async fn chain(&self, p: Polygon) -> Result<Chain, ErrorCode> {
        let ticker = match p.verify_options_ticker(){
            Ok(t) => t,
            Err(e) =>  return Err(ErrorCode::TickerError)
        };
        let date = match &p.date {
            Some(d) => d,
            None => return Err(ErrorCode::DateError),
        };
        let url = format!(
            "https://api.polygon.io/v3/snapshot/options/{}?apiKey={}",
            ticker, &p.api_key
        );
        let result = match p.request(url) 
        {
            Ok(response) => response,
            Err(e) => return Err(ErrorCode::RequestError),
        };
        match serde_json::from_str(result.as_str()) {
            Ok(chain) => Ok(chain),
            Err(e) => return Err(ErrorCode::FormatError),
        }
    }
}
