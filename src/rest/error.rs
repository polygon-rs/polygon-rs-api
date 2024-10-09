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
    DateToError,
    DateToNotSet,
    DateFromError,
    DateFromNotSet,
    RegexError,
    ToNotSet,
    FromNotSet,
    AdjusteedNotSet,
    SortNotSet,
    LimitNotSet,
    TimespanNotSet,
    MultiplierNotSet,
    IncludeOTCNotSet,
    OrderNotSet,
    TimestampNotSet,
    ContractTypeNotSet,
    JSONParseError,
    StrikePriceNotSet,
    WrongParameterType,
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
            ErrorCode::OrderNotSet => f.write_str("There is no order set"),
            ErrorCode::TimestampNotSet => f.write_str("There is no timestamp set"),
            ErrorCode::ContractTypeNotSet => f.write_str("There is no contract type set"),
            ErrorCode::JSONParseError => f.write_str("There is an issue with parsing the JSON"),
            ErrorCode::StrikePriceNotSet => f.write_str("There is no strike price set"),
            ErrorCode::DateToNotSet => f.write_str("There is no to date set"),
            ErrorCode::DateFromNotSet => f.write_str("There is no from date set"),
            ErrorCode::DateToError => f.write_str("There is an issue with the to date"),
            ErrorCode::DateFromError => f.write_str("There is an issue with the from date"),
            ErrorCode::WrongParameterType => f.write_str("There is an issue with the parameter type"),
        }
    }
}

