pub mod error;
pub mod market;
pub mod parameters;
pub mod reference;

use crate::ErrorCode;
use crate::{Parameter, ParameterRequirment, Parameters};
use regex::Regex;
//use serde::{Deserialize, Serialize};

#[derive(serde::Deserialize)]
pub enum Rest {
    Market(market::Market),
}

pub trait Request {
    const BASE_URL: &'static str = "https://api.polygon.io";
    const VERSION: &'static str;
    const CALL: &'static str;
    const PARAMETERS: &'static [&'static ParameterRequirment];

    fn parameters(&self) -> &Parameters;

    fn url(&mut self) -> &String;

    fn set_url(&mut self) -> Result<(), ErrorCode>;

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

    //Need to adjust Regex check for nano timestamp ^\d{19}$
    fn verify_date(&self, required: bool) -> Result<(), ErrorCode> {
        match &self.parameters().date {
            Some(d) => {
                match self
                    .set_regex(r"(19|20)([0-9]{2})-(1[0-2]|0[1-9])-(3[01]|[12][0-9]|0[1-9])")
                    .is_match(d.as_str())
                {
                    true => Ok(()),
                    false => Err(ErrorCode::DateError),
                }
            }
            None => {
                if required {
                    return Err(ErrorCode::DateNotSet);
                };
                Ok(())
            }
        }
    }

    //Need to adjust Regex check for nano timestamp ^\d{19}$ and verify that the date is less or equal to the to date
    fn verify_from_date(&self, required: bool) -> Result<(), ErrorCode> {
        match &self.parameters().from {
            Some(d) => {
                match self
                    .set_regex(r"(19|20)([0-9]{2})-(1[0-2]|0[1-9])-(3[01]|[12][0-9]|0[1-9])")
                    .is_match(d.as_str())
                {
                    true => Ok(()),
                    false => Err(ErrorCode::DateError),
                }
            }
            None => {
                if required {
                    return Err(ErrorCode::DateNotSet);
                };
                Ok(())
            }
        }
    }

    //Need to adjust Regex check for nano timestamp ^\d{19}$ and verify that the date is greater or equal to the from date
    fn verify_to_date(&self, required: bool) -> Result<(), ErrorCode> {
        match &self.parameters().to {
            Some(d) => {
                match self
                    .set_regex(r"(19|20)([0-9]{2})-(1[0-2]|0[1-9])-(3[01]|[12][0-9]|0[1-9])")
                    .is_match(d.as_str())
                {
                    true => Ok(()),
                    false => Err(ErrorCode::DateError),
                }
            }
            None => {
                if required {
                    return Err(ErrorCode::DateNotSet);
                };
                Ok(())
            }
        }
    }

    fn verify_options_ticker(&self, required: bool) -> Result<(), ErrorCode> {
        match &self.parameters().ticker {
            Some(t) => {
                match self.set_regex(r"(O:)([A-Z]){1,4}([0-9]{2})(1[0-2]|0[1-9])(3[01]|[12][0-9]|0[1-9])([CP]){1}([0-9]){8}").is_match(t.as_str()) {
                    true => Ok(()),
                    false => Err(ErrorCode::TickerError),
                }
            },
            None => {if required { return Err(ErrorCode::TickerNotSet)}; Ok(())},
        }
    }

    fn verify_ticker(&self, required: bool) -> Result<(), ErrorCode> {
        match &self.parameters().ticker {
            Some(t) => match self.set_regex(r"^O:").is_match(t.as_str()) {
                true => { match self.set_regex(r"(O:)([A-Z]){1,4}([0-9]{2})(1[0-2]|0[1-9])(3[01]|[12][0-9]|0[1-9])([CP]){1}([0-9]){8}").is_match(t.as_str()) {
                    true => Ok(()),
                    false => Err(ErrorCode::TickerError),
                }},
                false => Ok(()),
            },
            None => {if required { return Err(ErrorCode::TickerNotSet)}; Ok(())},
        }
    }

    fn verify_adjusted(&self, required: bool) -> Result<(), ErrorCode> {
        match &self.parameters().adjusted {
            Some(_) => Ok(()),
            None => {
                if required {
                    return Err(ErrorCode::AdjusteedNotSet);
                };
                Ok(())
            }
        }
    }

    fn verify_sort(&self, required: bool) -> Result<(), ErrorCode> {
        match &self.parameters().sort {
            Some(_) => Ok(()),
            None => {
                if required {
                    return Err(ErrorCode::SortNotSet);
                };
                Ok(())
            }
        }
    }

    fn verify_limit(&self, required: bool) -> Result<(), ErrorCode> {
        match &self.parameters().limit {
            Some(_) => Ok(()),
            None => {
                if required {
                    return Err(ErrorCode::LimitNotSet);
                };
                Ok(())
            }
        }
    }

    fn verify_timespan(&self, required: bool) -> Result<(), ErrorCode> {
        match &self.parameters().timespan {
            Some(_) => Ok(()),
            None => {
                if required {
                    return Err(ErrorCode::TimespanNotSet);
                };
                Ok(())
            }
        }
    }

    fn verify_multiplier(&self, required: bool) -> Result<(), ErrorCode> {
        match &self.parameters().multiplier {
            Some(_) => Ok(()),
            None => {
                if required {
                    return Err(ErrorCode::MultiplierNotSet);
                };
                Ok(())
            }
        }
    }

    fn verify_order(&self, required: bool) -> Result<(), ErrorCode> {
        match &self.parameters().order {
            Some(_) => Ok(()),
            None => {
                if required {
                    return Err(ErrorCode::OrderNotSet);
                };
                Ok(())
            }
        }
    }

    fn verify_sortv3(&self, required: bool) -> Result<(), ErrorCode> {
        match &self.parameters().sortv3 {
            Some(_) => Ok(()),
            None => {
                if required {
                    return Err(ErrorCode::SortNotSet);
                };
                Ok(())
            }
        }
    }

    fn verify_timestamp(&self, required: bool) -> Result<(), ErrorCode> {
        match &self.parameters().timestamp {
            Some(_) => Ok(()),
            None => {
                if required {
                    return Err(ErrorCode::TimestampNotSet);
                };
                Ok(())
            }
        }
    }

    fn verify_contract_type(&self, required: bool) -> Result<(), ErrorCode> {
        match &self.parameters().contract_type {
            Some(_) => Ok(()),
            None => {
                if required {
                    return Err(ErrorCode::MultiplierNotSet);
                };
                Ok(())
            }
        }
    }

    fn verify_include_otc(&self, required: bool) -> Result<(), ErrorCode> {
        match &self.parameters().include_otc {
            Some(_) => Ok(()),
            None => {
                if required {
                    return Err(ErrorCode::IncludeOTCNotSet);
                };
                Ok(())
            }
        }
    }

    fn check_parameters(&self) -> Result<(), ErrorCode> {
        if let Err(check) = self.verify_api_key() {
            return Err(check);
        }
        for parameter in Self::PARAMETERS {
            match parameter.parameter {
                Parameter::Ticker => {
                    if let Err(check) = self.verify_ticker(parameter.required) {
                        return Err(check);
                    }
                }
                Parameter::Date => {
                    if let Err(check) = self.verify_date(parameter.required) {
                        return Err(check);
                    }
                }
                Parameter::Adjusted => {
                    if let Err(check) = self.verify_adjusted(parameter.required) {
                        return Err(check);
                    }
                }
                Parameter::Sort => {
                    if let Err(check) = self.verify_sort(parameter.required) {
                        return Err(check);
                    }
                }
                Parameter::Limit => {
                    if let Err(check) = self.verify_limit(parameter.required) {
                        return Err(check);
                    }
                }
                Parameter::Timespan => {
                    if let Err(check) = self.verify_timespan(parameter.required) {
                        return Err(check);
                    }
                }
                Parameter::From => {
                    if let Err(check) = self.verify_from_date(parameter.required) {
                        return Err(check);
                    }
                }
                Parameter::To => {
                    if let Err(check) = self.verify_to_date(parameter.required) {
                        return Err(check);
                    }
                }
                Parameter::Multiplier => {
                    if let Err(check) = self.verify_multiplier(parameter.required) {
                        return Err(check);
                    }
                }
                Parameter::IncludeOTC => {
                    if let Err(check) = self.verify_include_otc(parameter.required) {
                        return Err(check);
                    }
                }
                Parameter::OptionsTicker => {
                    if let Err(check) = self.verify_options_ticker(parameter.required) {
                        return Err(check);
                    }
                }
                Parameter::Order => {
                    if let Err(check) = self.verify_order(parameter.required) {
                        return Err(check);
                    }
                }
                Parameter::Sortv3 => {
                    if let Err(check) = self.verify_sortv3(parameter.required) {
                        return Err(check);
                    }
                }
                Parameter::Timestamp => {
                    if let Err(check) = self.verify_timestamp(parameter.required) {
                        return Err(check);
                    }
                }
                Parameter::ContractType => {
                    if let Err(check) = self.verify_contract_type(parameter.required) {
                        return Err(check);
                    }
                }
            }
        }
        Ok(())
    }

    #[tokio::main]
    async fn get_raw_data(&mut self) -> Result<String, ErrorCode> {
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
