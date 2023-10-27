use crate::Polygon;
use serde::{Deserialize, Serialize};
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
        let api_key = match p.api_key {
            Some(a) => a,
            None => panic!("There is no api key set"),
        };
        let mut urlOptions = String::from("");
        match p.ticker {
            Some(t) => {
                urlOptions = format!("{}", t);
            }
            None => panic!("There is no ticker set"),
        };
        match p.date {
            Some(d) => {
                urlOptions = format!("{}&timestamp={}", urlOptions, d);
            }
            None => {
                println!("There is no date set, trying from and to.");
                match p.from {
                    Some(f) => {
                        urlOptions = format!("{}&timestamp.gte={}", urlOptions, f);
                    }
                    None => println!("There is no from set"),
                };
                match p.to {
                    Some(t) => {
                        urlOptions = format!("{}&timestamp.lt={}", urlOptions, t);
                    }
                    None => println!("There is no to set"),
                };
            }
        };

        match p.sort {
            Some(s) => {
                urlOptions = format!("{}&order={:?}&sort=timestamp", urlOptions, s);
            }
            None => println!("There is no sort set"),
        };
        match p.limit {
            Some(l) => {
                urlOptions = format!("{}&limit={}", urlOptions, l);
            }
            None => println!("There is no limit set"),
        };

        let url = format!(
            "https://api.polygon.io/v3/quotes/{}?apiKey={}",
            urlOptions, api_key
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
