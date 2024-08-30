pub mod error;
pub mod sort;
pub mod timespan;
use crate::polygon::{error::ErrorCode, sort::Sort, timespan::Timespan};
use regex::Regex;
use serde::{Deserialize, Serialize};

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
    fn verify_api_key(&self) -> Result<(), ErrorCode> {
        let api_pattern = match Regex::new(r"\S{32}") {
            Ok(r) => r,
            Err(e) => return Err(ErrorCode::APIError),
        };
        if !api_pattern.is_match(self.api_key.as_str()) {
            return Err(ErrorCode::APIError);
        };
        Ok(())
    }

    pub fn verify_ticker(&self) -> Result<(), ErrorCode> {
        match &self.ticker {
            Some(_) => Ok(()),
            None => Err(ErrorCode::TickerNotSetError),
        }
    }

    pub fn verify_options_ticker(&self) -> Result<(), ErrorCode> {
        match &self.ticker {
            Some(t) => {
                let option_pattern = match Regex::new(
                    r"(O:)([A-Z]){1,4}([0-9]{2})(1[0-2]|0[1-9])(3[01]|[12][0-9]|0[1-9])([CP]){1}([0-9]){8}",
                ) {
                    Ok(r) => r,
                    Err(e) => return Err(ErrorCode::TickerError),
                };
                match option_pattern.is_match(t.as_str()) {
                    true => Ok(()),
                    false => Err(ErrorCode::TickerError),
                }
            },
            None => Err(ErrorCode::TickerNotSetError),
        }
    }

    pub fn verify_date(&self) -> Result<(), ErrorCode> {
        match &self.date {
            Some(d) =>{ 
                let date_pattern = match Regex::new(r"(19|20)([0-9]{2})-(1[0-2]|0[1-9])-(3[01]|[12][0-9]|0[1-9])") {
                    Ok(r) => r,
                    Err(e) => return Err(ErrorCode::DateError),
                };
                match date_pattern.is_match(d.as_str()) {
                    true => Ok(()),
                    false => Err(ErrorCode::DateError),
                }
            },
            None => Err(ErrorCode::DateError),
        }
    }

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
    ) -> Result<Polygon, ErrorCode> {
        match self.verify_api_key() {
            Ok(_) => (),
            Err(e) => return Err(e),
        };
        Ok(Polygon {
            api_key,
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
        })
    }


    #[tokio::main]
    pub async fn request(&self, url: String) -> Result<String, ErrorCode> {
        match reqwest::get(url).await {
            Ok(response) => match response.text().await {
                Ok(text) => Ok(text),
                Err(e) => Err(ErrorCode::RequestError),
            },
            Err(e) => Err(ErrorCode::RequestError),
        }
    }
}
