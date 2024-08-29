use crate::{polygon::error::ErrorCode, Polygon};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Daily {
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

impl Daily {
    #[tokio::main]
    pub async fn daily(p: Polygon) -> Result<Daily,  ErrorCode> {
        let ticker = match &p.ticker {
            Some(t) => t,
            None => return Err(ErrorCode::TickerError),
        };
        let date = match &p.date {
            Some(d) => d,
            None => return Err(ErrorCode::DateError),
        };
        let url = format!(
            "https://api.polygon.io/v1/open-close/{}/{}?apiKey={}",
            ticker, date, &p.api_key
        );
        let result = match p.request(url) 
        {
            Ok(response) => response,
            Err(e) => return Err(ErrorCode::RequestError),
        };
        match serde_json::from_str(result.as_str()) {
            Ok(daily) => Ok(daily),
            Err(e) => return Err(ErrorCode::FormatError),
        }
    }
}
