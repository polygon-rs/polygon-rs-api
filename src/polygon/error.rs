use std::error;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum ErrorCode {
    TickerError,
    TickerNotSetError,
    APIError,
    RequestError,
    FormatError,
    DateError,
    DateNotSetError,
    RegexError,
}

impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ErrorCode::TickerError => f.write_str("There is an issue with the Ticker format"),
            ErrorCode::TickerNotSetError => f.write_str("The ticker does not appear to have any value set, please set a value for the ticker"),
            ErrorCode::APIError => f.write_str("There is an issue with the API Key"),
            ErrorCode::RequestError => f.write_str("There is an issue with the Request"),
            ErrorCode::FormatError => f.write_str("There is an issue with the Format"),
            ErrorCode::DateError => f.write_str("There is an issue with the Date"),
            ErrorCode::DateNotSetError => f.write_str("There is no date set"),
            ErrorCode::RegexError => f.write_str("There is an issue with the Regex"),
        }
    }
}
