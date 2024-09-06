pub mod error;
pub mod market;
pub mod parameters;
pub mod reference;

use std::f32::consts::E;

use crate::ErrorCode;
use crate::{Parameter, ParameterRequirment, Parameters};
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(serde::Deserialize)]
pub enum Rest {
    Market(market::Market),
}

pub trait Request {
    const PARAMETERS: &'static [&'static ParameterRequirment];

    fn parameters(&self) -> &Parameters;

    fn url(&self) -> String;

    fn set_parameters(&self);

    fn set_url(&self);


    fn set_regex(&self, pattern: &str) -> Regex {
        match Regex::new(pattern) {
            Ok(r) => r,
            Err(e) => panic!("The following error occured: {}", ErrorCode::RegexError),
        }
    }

    fn verify_api_key(&self) -> Result<(), ErrorCode> {
        if !self
            .set_regex(r"\S{32}")
            .is_match(&self.parameters().api_key.as_str())
        {
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

    fn check_parameters(&self) -> Result<(), ErrorCode> {
        //let mut url_options = String::from("");
        if let Err(check) = self.verify_api_key() {
            return Err(check);
        }
        for parameter in Self::PARAMETERS {
            match parameter.parameter {
                Parameter::Ticker => {
                    if parameter.required {
                        if let Err(check) = self.verify_ticker() {
                            return Err(check);
                        }
                    }
                    //if Self::VERSION == "v1" { continue }
                    /*if let Some(t) = &self.parameters().ticker  {
                        url_options = format!("{}ticker={}&", url_options, t);
                    }*/
                }
                Parameter::Date => {
                    if parameter.required {
                        if let Err(check) = self.verify_date() {
                            return Err(check);
                        }
                    }
                    /*if Self::VERSION == "v1" { continue }
                    if let Some(d) = &self.parameters().date  {
                        url_options = format!("{}timestamp={}?", url_options, d);
                    }*/
                }
                Parameter::Adjusted => {}
                Parameter::Sort => {}
                Parameter::Limit => {}
                Parameter::Timespan => {}
                Parameter::From => {}
                Parameter::To => {}
            }
        }
        /*let mut url = String::from(Self::BASE_URL);
        if Self::VERSION == "v1" { if let Some(t) = &self.parameters().ticker  { url = format!("{}{}/",url,t); if let Some(d) = &self.parameters().date  { url = format!("{}{}",url,d); }}}; 
        Ok(format!(
            "{}{}?apiKey={}",
            url,
            url_options,
            self.parameters().api_key
        ))*/
        Ok(())
    }

    #[tokio::main]
    async fn get_raw_data(&self) -> Result<String, ErrorCode> {
        match reqwest::get(self.url()).await {
                Ok(response) => match response.text().await {
                    Ok(text) => Ok(text),
                    Err(e) => Err(ErrorCode::RequestError),
                },
                Err(e) => return Err(ErrorCode::RequestError),
            }
        }
    

    fn request(&mut self) -> Result<(), ErrorCode>;
}
