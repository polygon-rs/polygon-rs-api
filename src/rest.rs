pub mod market;
pub mod reference;
pub mod parameters;
pub mod error;

use crate::{Parameter, ParameterRequirment, Parameters};
use crate::ErrorCode;
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(serde::Deserialize)]
pub enum Rest {
    Market(market::Market),
}

pub trait Request{
    const BASE_URL: &'static str;
    const PARAMETERS: &'static [&'static ParameterRequirment];

    fn parameters(&self) -> &Parameters;

    fn set_regex(&self, pattern: &str) -> Regex {
        match Regex::new(pattern) {
            Ok(r) => r,
            Err(e) => panic!("The following error occured: {}", ErrorCode::RegexError),
        }
    }

    fn verify_api_key(&self) -> Result<(), ErrorCode> {
        if !self.set_regex(r"\S{32}").is_match(&self.parameters().api_key.as_str()) {
            return Err(ErrorCode::APIError);
        };
        Ok(())
    }

    fn verify_date(&self) -> Result<(), ErrorCode> {
        match &self.parameters().date {
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

    fn verify_options_ticker(&self) -> Result<(), ErrorCode> {
        match &self.parameters().ticker {
            Some(t) => {
                match self.set_regex(r"(O:)([A-Z]){1,4}([0-9]{2})(1[0-2]|0[1-9])(3[01]|[12][0-9]|0[1-9])([CP]){1}([0-9]){8}").is_match(t.as_str()) {
                    true => Ok(()),
                    false => Err(ErrorCode::TickerError),
                }
            },
            None => Err(ErrorCode::TickerNotSetError),
        }
    }

    fn verify_ticker(&self) -> Result<(), ErrorCode> {
        match &self.parameters().ticker {
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
        self.verify_api_key();
        Self::PARAMETERS.into_iter().for_each(|parameter| {
            match parameter.parameter {
                Parameter::Ticker => { if parameter.required { self.verify_ticker(); } match &self.parameters().ticker { Some(ti) => url_options = format!(",{}", ti), None => ()} }
                Parameter::Date => {}
                Parameter::Adjusted => {}
                Parameter::Sort => {}
                Parameter::Limit => {}
                Parameter::Timespan => {}
                Parameter::From => {}
                Parameter::To => {}
            }
        });
        Ok(format!("{}{}?api_key={}",Self::BASE_URL.to_string(), url_options,self.parameters().api_key))
    }

    #[tokio::main]
    async fn get_raw_data(&self) -> Result<String, ErrorCode> {
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

    fn request(&self) -> Result<Rest, ErrorCode>;
}