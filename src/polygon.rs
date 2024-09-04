pub mod error;
pub mod sort;
pub mod timespan;
use crate::{
    polygon::{error::ErrorCode, sort::Sort, timespan::Timespan},
    rest::Rest
};
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
    

    pub fn new(
        &self,
        api_key: String,
        ticker: Parameter,
        multiplier: Parameter,
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
    

    fn verify_api_key(&self) -> Result<(), ErrorCode> {
        if !self.set_regex(r"\S{32}").is_match(self.api_key.as_str()) {
            return Err(ErrorCode::APIError);
        };
        Ok(())
    }

    

    fn verify_date(&self) -> Result<(), ErrorCode> {
        match &self.date {
            Some(d) => {
                let date_pattern =
                    match Regex::new(r"(19|20)([0-9]{2})-(1[0-2]|0[1-9])-(3[01]|[12][0-9]|0[1-9])")
                    {
                        Ok(r) => r,
                        Err(e) => return Err(ErrorCode::DateError),
                    };
                match date_pattern.is_match(d.as_str()) {
                    true => Ok(()),
                    false => Err(ErrorCode::DateError),
                }
            }
            None => Err(ErrorCode::DateNotSetError),
        }
    }
    
}

pub trait Set {
    fn new(&self, api_key: String) 
}

pub trait Get {
    const BASE_URL: &'static str;
    const PARAMETERS: Vec<Parameters>;

    fn set_regex(&self, pattern: &str) -> Regex {
        match Regex::new(pattern) {
            Ok(r) => r,
            Err(e) => panic!("The following error occured: {}", ErrorCode::RegexError),
        }
    }

    fn verify_options_ticker(&self, ticker: Option<String>) -> Result<(), ErrorCode> {
        match ticker {
            Some(t) => {
                match self.set_regex(r"(O:)([A-Z]){1,4}([0-9]{2})(1[0-2]|0[1-9])(3[01]|[12][0-9]|0[1-9])([CP]){1}([0-9]){8}").is_match(t.as_str()) {
                    true => Ok(()),
                    false => Err(ErrorCode::TickerError),
                }
            },
            None => Err(ErrorCode::TickerNotSetError),
        }
    }

    fn verify_ticker(&self, ticker: Option<String>) -> Result<(), ErrorCode> {
        match ticker {
            Some(t) => match self.set_regex(r"^O:").is_match(t.as_str()) {
                true => { match self.set_regex(r"(O:)([A-Z]){1,4}([0-9]{2})(1[0-2]|0[1-9])(3[01]|[12][0-9]|0[1-9])([CP]){1}([0-9]){8}").is_match(t.as_str()) {
                    true => Ok(()),
                    false => Err(ErrorCode::TickerError),
                }},
                false => Ok(()),
            },
            None => Err(ErrorCode::TickerNotSetError),
        }
    }


    fn url_builder(&self) -> Result<String, ErrorCode> {
        let mut url_options = String::from("");
        Self::PARAMETERS.into_iter().for_each(|parameter| {
            match parameter.parameter {
                Parameter::Ticker => { if parameter.required { self.verify_ticker(); } match t { Some(ti) => url_options = format!(",{}", ti), None => ()} }
                Parameter::Date => {}
                Parameter::Adjusted => {}
                Parameter::Sort => {}
                Parameter::Limit => {}
                Parameter::Timespan => {}
                Parameter::From => {}
                Parameter::To => {}
            }
        });
        Ok(format!("{}{}",Self::BASE_URL.to_string(), url_options))
    }

    #[tokio::main]
    async fn request(&self) -> Result<String, ErrorCode> {
        match self.url_builder() {
            Ok(url) => {
                match reqwest::get(url).await {
                Ok(response) => match response.text().await {
                    Ok(text) => Ok(text),
                    Err(e) => Err(ErrorCode::RequestError),
                },
                Err(e) => return Err(ErrorCode::RequestError),
            }},
            Err(e) => return Err(e),
        }
    }

    fn get(&self) -> Result<Rest, ErrorCode>;
}

#[derive(Clone, Debug)]
pub enum Parameter{
    Ticker,
    Date,
    Adjusted,
    Sort,
    Limit,
    Timespan,
    From,
    To,
}

#[derive(Clone, Debug)]
pub struct Parameters {
    pub required: bool,
    pub parameter: Parameter
}
