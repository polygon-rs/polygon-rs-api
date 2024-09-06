use crate::rest::Parameters;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Trades {
    pub next_url: Option<String>,
    pub request_id: Option<String>,
    pub results: Option<Vec<Trade>>,
    pub status: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Trade {
    pub conditions: Option<Vec<i32>>,
    pub correction: Option<i32>,
    pub exchange: Option<i32>,
    pub price: Option<f64>,
    pub size: Option<i32>,
    pub participant_timestamp: Option<i64>,
    pub sequence_number: Option<i64>,
    pub sip_timestamp: Option<i64>,
    pub id: Option<i32>,
}

impl Trades {
    #[tokio::main]
    pub async fn trades(p: Parameters) -> Result<Trades, Box<dyn Error>> {
        let mut url_options = String::from("");
        match p.ticker {
            Some(t) => {
                url_options = format!("{}?", t);
            }
            None => panic!("There is no ticker set"),
        };
        match p.date {
            Some(d) => {
                url_options = format!("{}timestamp={}&", url_options, d);
            }
            None => {
                if p.verbose == Some(true) {
                    println!("There is no date set, trying from and to.");
                }
                match p.from {
                    Some(f) => {
                        url_options = format!("{}timestamp.gte={}&", url_options, f);
                    }
                    None => { if p.verbose == Some(true) { println!("There is no from set")} },
                };
                match p.to {
                    Some(t) => {
                        url_options = format!("{}timestamp.lt={}&", url_options, t);
                    }
                    None => { if p.verbose == Some(true) { println!("There is no to set")} },
                };
            }
        };

        match p.sort {
            Some(s) => {
                url_options = format!("{}order={:?}&sort=timestamp&", url_options, s);
            }
            None => { if p.verbose == Some(true) { println!("There is no sort set")} },
        };
        match p.limit {
            Some(l) => {
                url_options = format!("{}limit={}&", url_options, l);
            }
            None => { if p.verbose == Some(true) { println!("There is no limit set")} },
        };

        let url = format!(
            "https://api.polygon.io/v3/trades/{}apiKey={}",
            url_options, p.api_key
        );
        let request = match reqwest::get(url).await {
            Ok(response) => match response.text().await {
                Ok(text) => text,
                Err(e) => panic!("The following error occured: {}", e),
            },
            Err(e) => panic!("The following error occured: {}", e),
        };
        match serde_json::from_str(request.as_str()) {
            Ok(trades) => Ok(trades),
            Err(e) => panic!("The following error occured: {}", e),
        }
    }
}
