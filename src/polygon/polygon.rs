use crate::polygon::{sort::Sort, timespan::Timespan};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{error::{self, Error}, fmt};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Polygon {
    pub api_key: String,
    pub ticker: Option<String>,
    pub multiplier: Option<u16>,
    pub timespan: Option<Timespan>,
    pub from: Option<String>,
    pub to: Option<String>,
    pub adjusted: Option<bool>,
    pub sort: Option<Sort>,
    pub limit: Option<u16>,
    pub date: Option<String>,
    pub verbose: Option<bool>,
}

impl Polygon {
    pub fn polygon(
        &self,
        api_key: String,
        ticker: Option<String>,
        multiplier: Option<u16>,
        timespan: Option<Timespan>,
        from: Option<String>,
        to: Option<String>,
        adjusted: Option<bool>,
        sort: Option<Sort>,
        limit: Option<u16>,
        date: Option<String>,
        verbose: Option<bool>,
    ) -> Polygon {
        Polygon {
            api_key: {
                let api_pattern = match Regex::new(r"\S{32}") {
                    Ok(r) => r,
                    Err(e) => panic!("The following error occured: {}", e),
                };
                if !api_pattern.is_match(api_key.as_str()) {
                    panic!("There is no api key set")
                };
                api_key
            },
            ticker,
            multiplier,
            timespan,
            from,
            to,
            adjusted,
            sort,
            limit,
            date,
            verbose,
        }
    }

    pub fn verify_options_ticker(&self) -> Result<String, TickerError>{
        match &self.ticker {
            Some(t) => {
                let option_pattern = match Regex::new(r"(O:)([A-Z]){1,4}([0-9]{2})(1[0-2]|0[1-9])(3[01]|[12][0-9]|0[1-9])([CP]){1}([0-9]){8}") {
                    Ok(r) => r,
                    Err(e) => panic!()
                };
                match option_pattern.is_match(t.as_str()){
                    true => Ok(t.to_string()),
                    false => Err(TickerError),
                }
                //Ok(t.to_string())
            }
            None => Err(TickerError),
        }
    }

    #[tokio::main]
    pub async fn request(&self, url: String) -> Result<String, Box<dyn Error>> {
        match reqwest::get(url).await {
            Ok(response) => match response.text().await {
                Ok(text) => Ok(text),
                Err(e) => panic!("The following error occured: {}", e),
            },
            Err(e) => panic!("The following error occured: {}", e),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TickerError;

impl fmt::Display for TickerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid ticker item")
    }
}

impl error::Error for TickerError {}
