use serde::{Deserialize, Serialize};
use crate::Polygon;
use std::error::Error;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NBBO {
    pub next_url: Option<String>,
    pub request_id: Option<String>,
    pub results: Option<Vec<Quote>>,
    pub status: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Quote {
    pub ask_exchange: Option<i32>,
    pub ask_price: Option<f64>,
    pub ask_size: Option<i32>,
    pub bid_exchange: Option<i32>,
    pub bid_price: Option<f64>,
    pub bid_size: Option<i32>,
    pub indicators: Option<Vec<i32>>,
    pub participant_timestamp: Option<i64>,
    pub sequence_number: Option<i64>,
    pub sip_timestamp: Option<i64>,
    pub tape: Option<i32>,
}

impl NBBO {
    #[tokio::main]
    pub async fn nbbo(p: Polygon) -> Result<NBBO, Box<dyn Error>> {
        let ticker = match p.ticker {
            Some(t) => t,
            None => panic!("There is no ticker set"),
        };
        let api_key = match p.api_key {
            Some(a) => a,
            None => panic!("There is no api key set"),
        };
        let url = format!(
            "https://api.polygon.io/v3/quotes/{}?apiKey={}",
            ticker, api_key
        );
        let request = match reqwest::get(url).await {
            Ok(response) => match response.text().await {
                Ok(text) => text,
                Err(e) => panic!("The following error occured: {}", e),
            },
            Err(e) => panic!("The following error occured: {}", e),
        };
        match serde_json::from_str(request.as_str()) {
            Ok(nbbo) => Ok(nbbo),
            Err(e) => panic!("The following error occured: {}", e),
        }
    }
}