use regex::Regex;
use crate::ErrorCode;


pub struct RegexPatterns {}

impl RegexPatterns {
    const API_KEY: &'static str = r"\S{32}";
    const TICKER: &'static str = r"^O:";
    const OPTIONS_CHECK: &'static str = r"^O:";
    const OPTIONS_TICKER: &'static str =
        r"(O:)([A-Z]){1,4}([0-9]{2})(1[0-2]|0[1-9])(3[01]|[12][0-9]|0[1-9])([CP]){1}([0-9]){8}";
    const STRING_DATE: &'static str = r"(19|20)([0-9]{2})-(1[0-2]|0[1-9])-(3[01]|[12][0-9]|0[1-9])";
    const EPOCH_NANO_DATE: &'static str = r"^\d{19}$";

    pub fn api_key() -> Regex {
        match Regex::new(Self::API_KEY) {
            Ok(regex) => regex,
            Err(e) => panic!(
                "The follow error code: {} occurred due to {}",
                ErrorCode::RegexError,
                e
            ),
        }
    }

    pub fn ticker() -> Regex {
        match Regex::new(Self::TICKER) {
            Ok(regex) => regex,
            Err(e) => panic!(
                "The follow error code: {} occurred due to {}",
                ErrorCode::RegexError,
                e
            ),
        }
    }

    pub fn options_check() -> Regex {
        match Regex::new(Self::OPTIONS_CHECK) {
            Ok(regex) => regex,
            Err(e) => panic!(
                "The follow error code: {} occurred due to {}",
                ErrorCode::RegexError,
                e
            ),
        }
    }

    pub fn options_ticker() -> Regex {
        match Regex::new(Self::OPTIONS_TICKER) {
            Ok(regex) => regex,
            Err(e) => panic!(
                "The follow error code: {} occurred due to {}",
                ErrorCode::RegexError,
                e
            ),
        }
    }

    pub fn string_date() -> Regex {
        match Regex::new(Self::STRING_DATE) {
            Ok(regex) => regex,
            Err(e) => panic!(
                "The follow error code: {} occurred due to {}",
                ErrorCode::RegexError,
                e
            ),
        }
    }

    pub fn epoch_nano_date() -> Regex {
        match Regex::new(Self::EPOCH_NANO_DATE) {
            Ok(regex) => regex,
            Err(e) => panic!(
                "The follow error code: {} occurred due to {}",
                ErrorCode::RegexError,
                e
            ),
        }
    }
}
