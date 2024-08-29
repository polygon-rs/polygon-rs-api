use core::fmt::{self, Debug, Display};


pub enum ErrorCode {
    TickerError,
    APIError,
    RequestError,
    FormatError,
    DateError,
}

impl Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorCode::TickerError => f.write_str("There is an issue with the Ticker format "),
            ErrorCode::APIError => f.write_str("There is an issue with the API Key"),
            ErrorCode::RequestError => f.write_str("There is an issue with the Request"),
            ErrorCode::FormatError => f.write_str("There is an issue with the Format"),
            ErrorCode::DateError => f.write_str("There is an issue with the Date"),
        }
    }
}
