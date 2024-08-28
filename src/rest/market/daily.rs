use serde::{Deserialize, Serialize};
use crate::Polygon;
use std::error::Error;

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
    pub async fn daily(p: Polygon) -> Result<Daily, Box<dyn Error>> {
        let ticker = match &p.ticker {
            Some(t) => t,
            None => panic!("There is no ticker set"),
        };
        let date = match & p.date {
            Some(d) => d,
            None => panic!("There is no date set"),
        };
        let url = format!(
            "https://api.polygon.io/v1/open-close/{}/{}?apiKey={}",
            ticker, date, &p.api_key
        );
        let result = match p.request(url) 
        {
            Ok(response) => response,
            Err(e) => panic!("The following error occured: {}", e),
        };
        match serde_json::from_str(result.as_str()) {
            Ok(daily) => Ok(daily),
            Err(e) => panic!("The following error occured: {}", e),
        }
    }
}
