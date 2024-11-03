use crate::rest::error::ErrorCode;
use regex::Regex;

pub struct RegexPatterns {}

impl RegexPatterns {
    const API_KEY: &'static str = r"^\S{32}$";
    const STOCK_CHECK: &'static str = r"^\w+$";
    const STOCK_TICKER: &'static str = r"^[A-Z]{1,6}$";
    const OPTION_CHECK: &'static str = r"^O:";
    const OPTION_TICKER: &'static str =
        r"(O:)([A-Z]){1,4}([0-9]{2})(1[0-2]|0[1-9])(3[01]|[12][0-9]|0[1-9])([CP]){1}([0-9]){8}$";
    const INDICIE_CHECK: &'static str = r"^I:";
    const INDICIE_TICKER: &'static str = r"^I:[A-Z0-9]+$";
    const FOREX_CHECK: &'static str = r"^C:";
    const FOREX_TICKER: &'static str = r"C:([A-Z]){6}$";
    const CRYPTO_CHECK: &'static str = r"^X:";
    const CRYPTO_TICKER: &'static str = r"^X:[A-Z0-9]+$";
    const STRING_DATE: &'static str =
        r"^(19|20)([0-9]{2})-(1[0-2]|0[1-9])-(3[01]|[12][0-9]|0[1-9])$";
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

    pub fn stocks_check() -> Regex {
        match Regex::new(Self::STOCK_CHECK) {
            Ok(regex) => regex,
            Err(e) => panic!(
                "The follow error code: {} occurred due to {}",
                ErrorCode::RegexError,
                e
            ),
        }
    }

    pub fn options_check() -> Regex {
        match Regex::new(Self::OPTION_CHECK) {
            Ok(regex) => regex,
            Err(e) => panic!(
                "The follow error code: {} occurred due to {}",
                ErrorCode::RegexError,
                e
            ),
        }
    }

    pub fn indicies_check() -> Regex {
        match Regex::new(Self::INDICIE_CHECK) {
            Ok(regex) => regex,
            Err(e) => panic!(
                "The follow error code: {} occurred due to {}",
                ErrorCode::RegexError,
                e
            ),
        }
    }

    pub fn forex_check() -> Regex {
        match Regex::new(Self::FOREX_CHECK) {
            Ok(regex) => regex,
            Err(e) => panic!(
                "The follow error code: {} occurred due to {}",
                ErrorCode::RegexError,
                e
            ),
        }
    }

    pub fn crypto_check() -> Regex {
        match Regex::new(Self::CRYPTO_CHECK) {
            Ok(regex) => regex,
            Err(e) => panic!(
                "The follow error code: {} occurred due to {}",
                ErrorCode::RegexError,
                e
            ),
        }
    }

    pub fn stocks_ticker() -> Regex {
        match Regex::new(Self::STOCK_TICKER) {
            Ok(regex) => regex,
            Err(e) => panic!(
                "The follow error code: {} occurred due to {}",
                ErrorCode::RegexError,
                e
            ),
        }
    }

    pub fn options_ticker() -> Regex {
        match Regex::new(Self::OPTION_TICKER) {
            Ok(regex) => regex,
            Err(e) => panic!(
                "The follow error code: {} occurred due to {}",
                ErrorCode::RegexError,
                e
            ),
        }
    }

    pub fn indicies_ticker() -> Regex {
        match Regex::new(Self::INDICIE_TICKER) {
            Ok(regex) => regex,
            Err(e) => panic!(
                "The follow error code: {} occurred due to {}",
                ErrorCode::RegexError,
                e
            ),
        }
    }

    pub fn forex_ticker() -> Regex {
        match Regex::new(Self::FOREX_TICKER) {
            Ok(regex) => regex,
            Err(e) => panic!(
                "The follow error code: {} occurred due to {}",
                ErrorCode::RegexError,
                e
            ),
        }
    }

    pub fn crypto_ticker() -> Regex {
        match Regex::new(Self::CRYPTO_TICKER) {
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

#[test]
fn test_api_key() {
    assert_eq!(
        RegexPatterns::api_key().is_match("ak123456789012345678901234567890"),
        true
    );
    assert_eq!(
        RegexPatterns::api_key().is_match("ak_1234567890123456789012345678901"),
        false
    );
}

#[test]
fn test_stocks_check() {
    assert_eq!(RegexPatterns::stocks_check().is_match("AAPL"), true);
    assert_eq!(RegexPatterns::stocks_check().is_match("A"), true);
    assert_eq!(
        RegexPatterns::stocks_check().is_match("O:AAPL230421C00200000"),
        false
    );
}

#[test]
fn test_stocks_ticker() {
    assert_eq!(RegexPatterns::stocks_ticker().is_match("AAPL"), true);
    assert_eq!(RegexPatterns::stocks_ticker().is_match("A"), true);
    assert_eq!(RegexPatterns::stocks_ticker().is_match("AAPLLLLL"), false);
}

#[test]
fn test_options_check() {
    assert_eq!(RegexPatterns::options_check().is_match("AAPL"), false);
    assert_eq!(
        RegexPatterns::options_check().is_match("O:AAPL230421C00200000"),
        true
    );
}

#[test]
fn test_options_ticker() {
    assert_eq!(RegexPatterns::options_ticker().is_match("AAPL"), false);
    assert_eq!(
        RegexPatterns::options_ticker().is_match("O:AAPL230421C00200000"),
        true
    );
}

#[test]
fn test_indicies_check() {
    assert_eq!(RegexPatterns::indicies_check().is_match("AAPL"), false);
    assert_eq!(RegexPatterns::indicies_check().is_match("I:AAPL"), true);
}

#[test]
fn test_indicies_ticker() {
    assert_eq!(RegexPatterns::indicies_ticker().is_match("AAPL"), false);
    assert_eq!(RegexPatterns::indicies_ticker().is_match("I:AAPL"), true);
}

#[test]
fn test_forex_check() {
    assert_eq!(RegexPatterns::forex_check().is_match("AAPL"), false);
    assert_eq!(RegexPatterns::forex_check().is_match("C:AAPL"), true);
}

#[test]
fn test_forex_ticker() {
    assert_eq!(RegexPatterns::forex_ticker().is_match("AAPL"), false);
    assert_eq!(RegexPatterns::forex_ticker().is_match("C:CADUSD"), true);
}

#[test]
fn test_crypto_check() {
    assert_eq!(RegexPatterns::crypto_check().is_match("AAPL"), false);
    assert_eq!(RegexPatterns::crypto_check().is_match("X:AAPL"), true);
}

#[test]
fn test_crypto_ticker() {
    assert_eq!(RegexPatterns::crypto_ticker().is_match("AAPL"), false);
    assert_eq!(RegexPatterns::crypto_ticker().is_match("X:AAPL"), true);
}

#[test]
fn test_string_date() {
    assert_eq!(RegexPatterns::string_date().is_match("2022-01-01"), true);
    assert_eq!(RegexPatterns::string_date().is_match("202201-01"), false);
}

#[test]
fn test_epoch_nano_date() {
    assert_eq!(
        RegexPatterns::epoch_nano_date().is_match("1640995200000000000"),
        true
    );
    assert_eq!(
        RegexPatterns::epoch_nano_date().is_match("2022-01-01"),
        false
    );
}
