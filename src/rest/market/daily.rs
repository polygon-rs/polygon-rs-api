use serde::{Deserialize, Serialize};
use crate::Polygon;
use std::error::Error;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Daily {
    pub afterHours: Option<f64>,
    pub close: Option<f64>,
    pub from: Option<String>,
    pub high: Option<f64>,
    pub low: Option<f64>,
    pub open: Option<f64>,
    pub preMarket: Option<f64>,
    pub status: Option<String>,
    pub symbol: Option<String>,
    pub volume: Option<f64>,
}

impl Daily {
    #[tokio::main]
    pub async fn daily(p: Polygon) -> Result<Daily, Box<dyn Error>> {
        let ticker = match p.ticker {
            Some(t) => t,
            None => panic!("There is no ticker set"),
        };
        let api_key = match p.api_key {
            Some(a) => a,
            None => panic!("There is no api key set"),
        };
        let date = match p.date {
            Some(d) => d,
            None => panic!("There is no date set"),
        };
        let url = format!(
            "https://api.polygon.io/v1/open-close/{}/{}?apiKey={}",
            ticker, date, api_key
        );
        let request = match reqwest::get(url).await {
            Ok(response) => match response.text().await {
                Ok(text) => text,
                Err(e) => panic!("The following error occured: {}", e),
            },
            Err(e) => panic!("The following error occured: {}", e),
        };
        match serde_json::from_str(request.as_str()) {
            Ok(daily) => Ok(daily),
            Err(e) => panic!("The following error occured: {}", e),
        }
    }
}
