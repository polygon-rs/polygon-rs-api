use crate::{polygon::polygon::TickerError, Polygon};
use serde::{Deserialize, Serialize};
use std::{error::Error, fmt};

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
    pub async fn chain(p: Polygon) -> Result<Chain, ChainError> {
        let ticker = match p.verify_options_ticker(){
            Ok(t) => t,
            Err(e) => 
                return Err(ChainError::TickerError)
        };
        let date = match p.date {
            Some(d) => d,
            None => panic!("There is no date set"),
        };
        let url = format!(
            "https://api.polygon.io/v3/snapshot/options/{}?apiKey={}",
            ticker, p.api_key
        );
        let request = match reqwest::get(url).await {
            Ok(response) => match response.text().await {
                Ok(text) => text,
                Err(e) => panic!("The following error occured: {}", e),
            },
            Err(e) => panic!("The following error occured: {}", e),
        };
        match serde_json::from_str(request.as_str()) {
            Ok(chain) => Ok(chain),
            Err(e) => panic!("The following error occured: {}", e),
        }
    }
}

#[derive(Debug)]
pub enum ChainError{
    TickerError,
}

impl fmt::Display for ChainError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ChainError::TickerError =>
                write!(f, "There was a ticker error"),
            // The wrapped error contains additional information and is available
            // via the source() method.
        }
    }
}

impl std::error::Error for ChainError {}
