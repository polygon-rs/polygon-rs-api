use crate::Polygon;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Aggregates {
    pub adjusted: Option<bool>,
    pub next_url: Option<String>,
    pub request_id: Option<String>,
    pub results: Option<Vec<Bar>>,
    pub status: Option<String>,
    pub resultsCount: Option<i32>,
    pub ticker: Option<String>,
    pub query_count: Option<i32>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Bar {
    pub c: Option<f64>,
    pub h: Option<f64>,
    pub l: Option<f64>,
    pub n: Option<i32>,
    pub o: Option<f64>,
    pub t: Option<i64>,
    pub v: Option<f64>,
    pub vw: Option<f64>,
}

impl Aggregates {
    #[tokio::main]
    pub async fn daily(p: Polygon) -> Result<Aggregates, Box<dyn Error>> {
        let ticker = match p.ticker {
            Some(t) => t,
            None => panic!("There is no ticker set"),
        };
        let date = match p.date {
            Some(d) => d,
            None => panic!("There is no date set"),
        };
        let url = format!(
            "https://api.polygon.io/v1/open-close/{}/{}?apiKey={}",
            ticker, date, p.api_key
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
