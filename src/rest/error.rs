use std::error;
use std::error::Error;
use std::f32::consts::E;
use std::fmt;

#[derive(Debug)]
pub enum ErrorCode {
    TickerError,
    OptionsTickerError,
    TickerNotSet,
    APIError,
    RequestError,
    FormatError,
    DateError,
    DateNotSet,
    RegexError,
    ToNotSet,
    FromNotSet,
    AdjusteedNotSet,
    SortNotSet,
    LimitNotSet,
    TimespanNotSet,
    MultiplierNotSet,
    IncludeOTCNotSet,
}

impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ErrorCode::TickerError => f.write_str("There is an issue with the Ticker format"),
            ErrorCode::OptionsTickerError => f.write_str("There is an issue with the Options Ticker format"),
            ErrorCode::TickerNotSet => f.write_str("The ticker does not appear to have any value set, please set a value for the ticker"),
            ErrorCode::APIError => f.write_str("There is an issue with the API Key"),
            ErrorCode::RequestError => f.write_str("There is an issue with the Request"),
            ErrorCode::FormatError => f.write_str("There is an issue with the Format"),
            ErrorCode::DateError => f.write_str("There is an issue with the Date"),
            ErrorCode::DateNotSet => f.write_str("There is no date set"),
            ErrorCode::RegexError => f.write_str("There is an issue with the Regex"),
            ErrorCode::ToNotSet => f.write_str("There is no to date set"),
            ErrorCode::FromNotSet => f.write_str("There is no from date set"),
            ErrorCode::AdjusteedNotSet => f.write_str("There is no adjusted set"),
            ErrorCode::SortNotSet => f.write_str("There is no sort set"),
            ErrorCode::LimitNotSet => f.write_str("There is no limit set"),
            ErrorCode::TimespanNotSet => f.write_str("There is no timespan set"),
            ErrorCode::MultiplierNotSet => f.write_str("There is no multiplier set"),
            ErrorCode::IncludeOTCNotSet => f.write_str("There is no include otc set"),
        }
    }
}
