use chrono::{DateTime, NaiveDate};

use crate::rest::{
    error::ErrorCode,
    parameters::{Parameter, ParameterRequirment, Parameters, TickerType, TickerTypes},
};

use super::regex_patterns::RegexPatterns;

pub struct Verification {}

impl Verification {
    fn date_error(date_type: &Parameter) -> ErrorCode {
        match date_type {
            Parameter::From => ErrorCode::DateFromError,
            Parameter::To => ErrorCode::DateToError,
            Parameter::Date => ErrorCode::DateError,
            _ => ErrorCode::WrongParameterType,
        }
    }

    fn verify_to_from(parameters: &Parameters) -> Result<(), ErrorCode> {
        if parameters.to.is_none() || parameters.from.is_none() {
            return Ok(());
        }
        let to_string = match &parameters.to {
            Some(t) => t.as_str(),
            None => return Err(ErrorCode::ToNotSet),
        };
        let from_string = match &parameters.from {
            Some(f) => f.as_str(),
            None => return Err(ErrorCode::FromNotSet),
        };
        let from = match NaiveDate::parse_from_str(from_string, "%Y-%m-%d") {
            Ok(d) => d.and_hms_opt(0, 0, 0).unwrap().and_utc(),
            Err(_) => match from_string.parse::<i64>() {
                Ok(n) => DateTime::from_timestamp_nanos(n),
                Err(_) => return Err(ErrorCode::DateToError),
            },
        };
        let to = match NaiveDate::parse_from_str(to_string, "%Y-%m-%d") {
            Ok(d) => d.and_hms_opt(0, 0, 0).unwrap().and_utc(),
            Err(_) => match to_string.parse::<i64>() {
                Ok(n) => DateTime::from_timestamp_nanos(n),
                Err(_) => return Err(ErrorCode::DateToError),
            },
        };
        if to < from {
            return Err(ErrorCode::DateToError);
        }

        Ok(())
    }

    fn verify_to_from_strike_price(parameters: &Parameters) -> Result<(), ErrorCode> {
        if parameters.strike_price_to.is_none() || parameters.strike_price_from.is_none() {
            return Ok(());
        }
        let to_strike_price = match &parameters.strike_price_to {
            Some(t) => t,
            None => return Err(ErrorCode::ToNotSet),
        };
        let from_strike_price = match &parameters.strike_price_from {
            Some(f) => f,
            None => return Err(ErrorCode::FromNotSet),
        };
        if to_strike_price < from_strike_price {
            return Err(ErrorCode::StrikePriceToError);
        }

        Ok(())
    }

    fn verify_to_from_ticker() -> Result<(), ErrorCode> {
        Ok(())
    }

    fn verify_api_key(parameters: &Parameters) -> Result<(), ErrorCode> {
        if !RegexPatterns::api_key().is_match(&parameters.api_key.as_str()) {
            return Err(ErrorCode::APIError);
        }
        Ok(())
    }

    fn verify_timestamp<T: ToString>(
        date_value: &T,
        date_type: &Parameter,
    ) -> Result<(), ErrorCode> {
        let date = date_value.to_string();
        match RegexPatterns::epoch_nano_date().is_match(date.as_str()) {
            true => Ok(()),
            false => Err(Self::date_error(date_type)),
        }
    }

    fn verify_date<T: ToString>(date_value: &T, date_type: &Parameter) -> Result<(), ErrorCode> {
        let date = date_value.to_string();
        if date.parse::<i64>().is_ok() {
            match Self::verify_timestamp(date_value, date_type) {
                Ok(_) => return Ok(()),
                Err(e) => return Err(e),
            }
        };
        match RegexPatterns::string_date().is_match(date.as_str()) {
            true => Ok(()),
            false => Err(Self::date_error(date_type)),
        }
    }

    fn verify_stock_ticker(ticker: String) -> Result<(), ErrorCode> {
        if !RegexPatterns::stocks_ticker().is_match(ticker.as_str()) {
            return Err(ErrorCode::TickerError);
        }
        Ok(())
    }

    fn verify_stocks_ticker(required: bool, parameters: &Parameters) -> Result<(), ErrorCode> {
        match &parameters.ticker {
            Some(t) => Self::verify_stock_ticker(t.to_string()),
            None => {
                if required {
                    return Err(ErrorCode::TickerNotSet);
                };
                Ok(())
            }
        }
    }

    fn verify_option_ticker(ticker: String) -> Result<(), ErrorCode> {
        if !RegexPatterns::options_ticker().is_match(ticker.as_str()) {
            return Err(ErrorCode::TickerError);
        }
        Ok(())
    }

    fn verify_options_ticker(required: bool, parameters: &Parameters) -> Result<(), ErrorCode> {
        match &parameters.ticker {
            Some(t) => Self::verify_option_ticker(t.to_string()),
            None => {
                if required {
                    return Err(ErrorCode::TickerNotSet);
                };
                Ok(())
            }
        }
    }

    fn verify_indicie_ticker(ticker: String) -> Result<(), ErrorCode> {
        if !RegexPatterns::indicies_ticker().is_match(ticker.as_str()) {
            return Err(ErrorCode::TickerError);
        }
        Ok(())
    }

    fn verify_indices_ticker(required: bool, parameters: &Parameters) -> Result<(), ErrorCode> {
        match &parameters.ticker {
            Some(t) => Self::verify_indicie_ticker(t.to_string()),
            None => {
                if required {
                    return Err(ErrorCode::TickerNotSet);
                };
                Ok(())
            }
        }
    }

    fn verify_forex_ticker(ticker: String) -> Result<(), ErrorCode> {
        if !RegexPatterns::forex_ticker().is_match(ticker.as_str()) {
            return Err(ErrorCode::TickerError);
        }
        Ok(())
    }

    fn verify_forex_tickers(required: bool, parameters: &Parameters) -> Result<(), ErrorCode> {
        match &parameters.ticker {
            Some(t) => Self::verify_forex_ticker(t.to_string()),
            None => {
                if required {
                    return Err(ErrorCode::TickerNotSet);
                };
                Ok(())
            }
        }
    }

    fn verify_crypto_ticker(ticker: String) -> Result<(), ErrorCode> {
        if !RegexPatterns::crypto_ticker().is_match(ticker.as_str()) {
            return Err(ErrorCode::TickerError);
        }
        Ok(())
    }

    fn verify_crypto_tickers(required: bool, parameters: &Parameters) -> Result<(), ErrorCode> {
        match &parameters.ticker {
            Some(t) => Self::verify_crypto_ticker(t.to_string()),
            None => {
                if required {
                    return Err(ErrorCode::TickerNotSet);
                };
                Ok(())
            }
        }
    }

    fn get_ticker_type(ticker: &String) -> Result<TickerType, ErrorCode> {
        if RegexPatterns::stocks_check().is_match(ticker.as_str()) == true {
            return Ok(TickerType::Stocks);
        }
        if RegexPatterns::options_check().is_match(ticker.as_str()) == true {
            return Ok(TickerType::Options);
        }
        if RegexPatterns::indicies_check().is_match(ticker.as_str()) == true {
            return Ok(TickerType::Indicies);
        }
        if RegexPatterns::forex_check().is_match(ticker.as_str()) == true {
            return Ok(TickerType::Forex);
        }
        if RegexPatterns::crypto_check().is_match(ticker.as_str()) == true {
            return Ok(TickerType::Crypto);
        }
        return Err(ErrorCode::TickerError);
    }

    fn verify_ticker(
        required: bool,
        ticker_types: &TickerTypes,
        parameters: &Parameters,
    ) -> Result<(), ErrorCode> {
        match &parameters.ticker {
            Some(t) => match Self::get_ticker_type(t) {
                Ok(ticker_type) => match ticker_type {
                    TickerType::Stocks => {
                        if !ticker_types.stocks {
                            return Err(ErrorCode::TickerNotValidForAPICall);
                        }
                        Self::verify_stocks_ticker(required, parameters)
                    }
                    TickerType::Options => {
                        if !ticker_types.options {
                            return Err(ErrorCode::TickerNotValidForAPICall);
                        }
                        Self::verify_options_ticker(required, parameters)
                    }
                    TickerType::Indicies => {
                        if !ticker_types.indicies {
                            return Err(ErrorCode::TickerNotValidForAPICall);
                        }
                        Self::verify_indices_ticker(required, parameters)
                    }
                    TickerType::Forex => {
                        if !ticker_types.forex {
                            return Err(ErrorCode::TickerNotValidForAPICall);
                        }
                        Self::verify_forex_tickers(required, parameters)
                    }
                    TickerType::Crypto => {
                        if !ticker_types.crypto {
                            return Err(ErrorCode::TickerNotValidForAPICall);
                        }
                        Self::verify_crypto_tickers(required, parameters)
                    }
                },
                Err(e) => return Err(e),
            },
            None => {
                if required {
                    return Err(ErrorCode::TickerNotSet);
                };
                Ok(())
            }
        }
    }

    fn verify_tickers(
        required: bool,
        ticker_types: &TickerTypes,
        parameters: &Parameters,
    ) -> Result<(), ErrorCode> {
        match &parameters.tickers {
            Some(tickers) => {
                for ticker in tickers {
                    match Self::get_ticker_type(ticker) {
                        Ok(ticker_type) => match ticker_type {
                            //Better Error Message
                            TickerType::Stocks => {
                                if !ticker_types.stocks {
                                    return Err(ErrorCode::TickerNotValidForAPICall);
                                };
                                Self::verify_stock_ticker(ticker.to_string())?
                            }
                            TickerType::Options => {
                                if !ticker_types.options {
                                    return Err(ErrorCode::TickerNotValidForAPICall);
                                };
                                Self::verify_option_ticker(ticker.to_string())?
                            }
                            TickerType::Indicies => {
                                if !ticker_types.indicies {
                                    return Err(ErrorCode::TickerNotValidForAPICall);
                                };
                                Self::verify_indicie_ticker(ticker.to_string())?
                            }
                            TickerType::Forex => {
                                if !ticker_types.forex {
                                    return Err(ErrorCode::TickerNotValidForAPICall);
                                };
                                Self::verify_forex_ticker(ticker.to_string())?
                            }
                            TickerType::Crypto => {
                                if !ticker_types.crypto {
                                    return Err(ErrorCode::TickerNotValidForAPICall);
                                };
                                Self::verify_crypto_ticker(ticker.to_string())?
                            }
                        },
                        Err(e) => return Err(e),
                    }
                }
                Ok(())
            }
            None => {
                if required {
                    return Err(ErrorCode::TickersNotSet);
                };
                Ok(())
            }
        }
    }

    fn verify_underlying_asset(required: bool, parameters: &Parameters) -> Result<(), ErrorCode> {
        match &parameters.underlying_asset {
            Some(underlying_asset) => {
                match Self::get_ticker_type(underlying_asset) {
                    Ok(ticker_type) => match ticker_type {
                        //Better Error Message
                        TickerType::Stocks => {
                            Self::verify_stock_ticker(underlying_asset.to_string())?
                        }
                        TickerType::Options => {
                            Self::verify_option_ticker(underlying_asset.to_string())?
                        }
                        TickerType::Indicies => {
                            Self::verify_indicie_ticker(underlying_asset.to_string())?
                        }
                        TickerType::Forex => {
                            Self::verify_forex_ticker(underlying_asset.to_string())?
                        }
                        TickerType::Crypto => {
                            Self::verify_crypto_ticker(underlying_asset.to_string())?
                        }
                    },
                    Err(e) => return Err(e),
                }
                Ok(())
            }
            None => {
                if required {
                    return Err(ErrorCode::UnderlyingAssetNotSet);
                };
                Ok(())
            }
        }
    }

    fn verify<T: ToString>(
        required: bool,
        parameter_value: &Option<T>,
        parameter_type: &Parameter,
    ) -> Result<(), ErrorCode> {
        match parameter_value {
            Some(p) => match parameter_type {
                Parameter::Date => Self::verify_date(p, parameter_type),
                Parameter::To => Self::verify_date(p, parameter_type),
                Parameter::From => Self::verify_date(p, parameter_type),
                Parameter::Timestamp => Self::verify_timestamp(p, parameter_type),
                _ => Ok(()),
            },
            None => {
                if required {
                    match parameter_type {
                        Parameter::Adjusted => return Err(ErrorCode::AdjusteedNotSet),
                        Parameter::Sort => return Err(ErrorCode::SortNotSet),
                        Parameter::Limit => return Err(ErrorCode::LimitNotSet),
                        Parameter::Timespan => return Err(ErrorCode::TimespanNotSet),
                        Parameter::Multiplier => return Err(ErrorCode::MultiplierNotSet),
                        Parameter::IncludeOTC => return Err(ErrorCode::IncludeOTCNotSet),
                        Parameter::Order => return Err(ErrorCode::OrderNotSet),
                        Parameter::Sortv3 => return Err(ErrorCode::SortNotSet),
                        Parameter::Timestamp => return Err(ErrorCode::TimestampNotSet),
                        Parameter::ContractType => return Err(ErrorCode::ContractTypeNotSet),
                        Parameter::StrikePrice => return Err(ErrorCode::StrikePriceNotSet),
                        Parameter::StrikePriceFrom => return Err(ErrorCode::StrikePriceFromNotSet),
                        Parameter::StrikePriceTo => return Err(ErrorCode::StrikePriceToNotSet),
                        Parameter::Amount => return Err(ErrorCode::AmountNotSet),
                        Parameter::Precision => return Err(ErrorCode::PrecisionNotSet),
                        Parameter::Direction => return Err(ErrorCode::DirectionNotSet),
                        Parameter::TickerType => return Err(ErrorCode::TickerTypeNotSet),
                        _ => return Err(ErrorCode::WrongParameterType),
                    }
                };
                Ok(())
            }
        }
    }

    pub fn check_parameters(
        ticker_types: &TickerTypes,
        parameter_requirements: &'static [&'static ParameterRequirment],
        parameters: &Parameters,
    ) -> Result<(), ErrorCode> {
        if let Err(check) = Self::verify_api_key(parameters) {
            return Err(check);
        }
        for parameter in parameter_requirements {
            match parameter.parameter {
                Parameter::Ticker => {
                    if let Err(check) =
                        Self::verify_ticker(parameter.required, ticker_types, parameters)
                    {
                        return Err(check);
                    }
                }
                Parameter::Tickers => {
                    if let Err(check) =
                        Self::verify_tickers(parameter.required, ticker_types, parameters)
                    {
                        return Err(check);
                    }
                }
                Parameter::UnderlyingAsset => {
                    if let Err(check) =
                        Self::verify_underlying_asset(parameter.required, parameters)
                    {
                        return Err(check);
                    }
                }
                Parameter::TickerFrom => {}
                Parameter::TickerTo => {}
                Parameter::Date => {
                    if let Err(check) =
                        Self::verify(parameter.required, &parameters.date, &parameter.parameter)
                    {
                        return Err(check);
                    }
                }
                Parameter::TickerType => {
                    if let Err(check) = Self::verify(
                        parameter.required,
                        &parameters.ticker_type,
                        &parameter.parameter,
                    ) {
                        return Err(check);
                    }
                }
                Parameter::Adjusted => {
                    if let Err(check) = Self::verify(
                        parameter.required,
                        &parameters.adjusted,
                        &parameter.parameter,
                    ) {
                        return Err(check);
                    }
                }
                Parameter::Sort => {
                    if let Err(check) =
                        Self::verify(parameter.required, &parameters.sort, &parameter.parameter)
                    {
                        return Err(check);
                    }
                }
                Parameter::Limit => {
                    if let Err(check) = Self::verify(
                        parameter.required,
                        &parameters.adjusted,
                        &parameter.parameter,
                    ) {
                        return Err(check);
                    }
                }
                Parameter::Timespan => {
                    if let Err(check) = Self::verify(
                        parameter.required,
                        &parameters.timespan,
                        &parameter.parameter,
                    ) {
                        return Err(check);
                    }
                }
                Parameter::From => {
                    if let Err(check) =
                        Self::verify(parameter.required, &parameters.from, &parameter.parameter)
                    {
                        return Err(check);
                    }
                }
                Parameter::To => {
                    if let Err(check) =
                        Self::verify(parameter.required, &parameters.to, &parameter.parameter)
                    {
                        return Err(check);
                    }
                }
                Parameter::Multiplier => {
                    if let Err(check) = Self::verify(
                        parameter.required,
                        &parameters.multiplier,
                        &parameter.parameter,
                    ) {
                        return Err(check);
                    }
                }
                Parameter::IncludeOTC => {
                    if let Err(check) = Self::verify(
                        parameter.required,
                        &parameters.include_otc,
                        &parameter.parameter,
                    ) {
                        return Err(check);
                    }
                }
                Parameter::Order => {
                    if let Err(check) =
                        Self::verify(parameter.required, &parameters.order, &parameter.parameter)
                    {
                        return Err(check);
                    }
                }
                Parameter::Sortv3 => {
                    if let Err(check) =
                        Self::verify(parameter.required, &parameters.sortv3, &parameter.parameter)
                    {
                        return Err(check);
                    }
                }
                Parameter::Timestamp => {
                    if let Err(check) = Self::verify(
                        parameter.required,
                        &parameters.timestamp,
                        &parameter.parameter,
                    ) {
                        return Err(check);
                    }
                }
                Parameter::ContractType => {
                    if let Err(check) = Self::verify(
                        parameter.required,
                        &parameters.contract_type,
                        &parameter.parameter,
                    ) {
                        return Err(check);
                    }
                }
                Parameter::StrikePrice => {
                    if let Err(check) = Self::verify(
                        parameter.required,
                        &parameters.strike_price,
                        &parameter.parameter,
                    ) {
                        return Err(check);
                    }
                }
                Parameter::StrikePriceFrom => {
                    if let Err(check) = Self::verify(
                        parameter.required,
                        &parameters.strike_price_from,
                        &parameter.parameter,
                    ) {
                        return Err(check);
                    }
                }
                Parameter::StrikePriceTo => {
                    if let Err(check) = Self::verify(
                        parameter.required,
                        &parameters.strike_price_to,
                        &parameter.parameter,
                    ) {
                        return Err(check);
                    }
                }
                Parameter::Amount => {
                    if let Err(check) =
                        Self::verify(parameter.required, &parameters.amount, &parameter.parameter)
                    {
                        return Err(check);
                    }
                }
                Parameter::Precision => {
                    if let Err(check) = Self::verify(
                        parameter.required,
                        &parameters.precision,
                        &parameter.parameter,
                    ) {
                        return Err(check);
                    }
                }
                Parameter::Direction => {
                    if let Err(check) = Self::verify(
                        parameter.required,
                        &parameters.direction,
                        &parameter.parameter,
                    ) {
                        return Err(check);
                    }
                }
                Parameter::ExpandUnderlying => {
                    if let Err(check) = Self::verify(
                        parameter.required,
                        &parameters.expand_underlying,
                        &parameter.parameter,
                    ) {
                        return Err(check);
                    }
                }
                Parameter::SeriesType => {
                    if let Err(check) = Self::verify(
                        parameter.required,
                        &parameters.series_type,
                        &parameter.parameter,
                    ) {
                        return Err(check);
                    }
                }
                Parameter::Window => {
                    if let Err(check) =
                        Self::verify(parameter.required, &parameters.window, &parameter.parameter)
                    {
                        return Err(check);
                    }
                }
                Parameter::LongWindow => {
                    if let Err(check) = Self::verify(
                        parameter.required,
                        &parameters.long_window,
                        &parameter.parameter,
                    ) {
                        return Err(check);
                    }
                }
                Parameter::ShortWindow => {
                    if let Err(check) = Self::verify(
                        parameter.required,
                        &parameters.short_window,
                        &parameter.parameter,
                    ) {
                        return Err(check);
                    }
                }
                Parameter::SignalWindow => {
                    if let Err(check) = Self::verify(
                        parameter.required,
                        &parameters.signal_window,
                        &parameter.parameter,
                    ) {
                        return Err(check);
                    }
                }
            }
        }
        if let Err(check) = Self::verify_to_from(parameters) {
            return Err(check);
        }
        if let Err(check) = Self::verify_to_from_strike_price(parameters) {
            return Err(check);
        }
        if let Err(check) = Self::verify_to_from_ticker() {
            return Err(check);
        }
        Ok(())
    }
}

#[test]
fn test_verify_date() {
    assert_eq!(
        Verification::verify_date(&String::from("2020-12-01"), &Parameter::Date),
        Ok(())
    );
    assert_eq!(
        Verification::verify_date(&String::from("2020-12-01T"), &Parameter::Date),
        Err(ErrorCode::DateError)
    );
    assert_eq!(
        Verification::verify_date(&String::from("1645455450000000000"), &Parameter::Date),
        Ok(())
    );
}

#[test]
fn test_verify_to_from() {
    let mut parameters = Parameters::default();
    parameters.to = Some(String::from("2021-12-01"));
    parameters.from = Some(String::from("2020-12-01"));
    assert_eq!(Verification::verify_to_from(&parameters), Ok(()));
    parameters.to = Some(String::from("2020-12-01"));
    parameters.from = Some(String::from("2021-12-01"));
    assert_eq!(
        Verification::verify_to_from(&parameters),
        Err(ErrorCode::DateToError)
    );
    parameters.to = Some(String::from("1727951392000000000"));
    parameters.from = Some(String::from("1730629792000000000"));
    assert_eq!(
        Verification::verify_to_from(&parameters),
        Err(ErrorCode::DateToError)
    );
    parameters.to = Some(String::from("1730629792000000000"));
    parameters.from = Some(String::from("1727951392000000000"));
    assert_eq!(Verification::verify_to_from(&parameters), Ok(()));
}

#[test]
fn test_verify_to_from_strike_price() {
    let mut parameters = Parameters::default();
    parameters.strike_price_to = Some(10.0);
    parameters.strike_price_from = Some(1.0);
    assert_eq!(
        Verification::verify_to_from_strike_price(&parameters),
        Ok(())
    );
    parameters.strike_price_to = Some(1.0);
    parameters.strike_price_from = Some(10.0);
    assert_eq!(
        Verification::verify_to_from_strike_price(&parameters),
        Err(ErrorCode::StrikePriceToError)
    );
    parameters.strike_price_to = Some(10.0);
    parameters.strike_price_from = Some(10.0);
    assert_eq!(
        Verification::verify_to_from_strike_price(&parameters),
        Ok(())
    );
}

#[test]
fn test_verify_api_key() {
    let mut parameters = Parameters::default();
    parameters.api_key = String::from("12345678901234567890123456789012");
    assert_eq!(Verification::verify_api_key(&parameters), Ok(()));
    parameters.api_key = String::from("a");
    assert_eq!(
        Verification::verify_api_key(&parameters),
        Err(ErrorCode::APIError)
    );
}

#[test]
fn test_verify_stock_ticker() {
    let ticker = String::from("AAPL");
    assert_eq!(Verification::verify_stock_ticker(ticker), Ok(()));
    let ticker = String::from("A");
    assert_eq!(Verification::verify_stock_ticker(ticker), Ok(()));
    let ticker = String::from("AA");
    assert_eq!(Verification::verify_stock_ticker(ticker), Ok(()));
    let ticker = String::from("AAA");
    assert_eq!(Verification::verify_stock_ticker(ticker), Ok(()));
    let ticker = String::from("AAAA");
    assert_eq!(Verification::verify_stock_ticker(ticker), Ok(()));
    let ticker = String::from("AAAAAAA");
    assert_eq!(
        Verification::verify_stock_ticker(ticker),
        Err(ErrorCode::TickerError)
    );
}

#[test]
fn test_verify_stocks_ticker() {
    let mut parameters = Parameters::default();
    parameters.ticker = Some(String::from("AAPL"));
    assert_eq!(
        Verification::verify_stocks_ticker(true, &parameters),
        Ok(())
    );
    parameters.ticker = Some(String::from("A"));
    assert_eq!(
        Verification::verify_stocks_ticker(true, &parameters),
        Ok(())
    );
    parameters.ticker = Some(String::from("AA"));
    assert_eq!(
        Verification::verify_stocks_ticker(true, &parameters),
        Ok(())
    );
    parameters.ticker = Some(String::from("AAA"));
    assert_eq!(
        Verification::verify_stocks_ticker(true, &parameters),
        Ok(())
    );
    parameters.ticker = Some(String::from("AAAA"));
    assert_eq!(
        Verification::verify_stocks_ticker(true, &parameters),
        Ok(())
    );
    parameters.ticker = Some(String::from("AAAAAAA"));
    assert_eq!(
        Verification::verify_stocks_ticker(true, &parameters),
        Err(ErrorCode::TickerError)
    );
    parameters.ticker = None;
    assert_eq!(
        Verification::verify_stocks_ticker(true, &parameters),
        Err(ErrorCode::TickerNotSet)
    );
    assert_eq!(
        Verification::verify_stocks_ticker(false, &parameters),
        Ok(())
    );
}

#[test]
fn test_verify_option_ticker() {
    let ticker = String::from("O:AAL210820C00014000");
    assert_eq!(Verification::verify_option_ticker(ticker), Ok(()));
    let ticker = String::from("A");
    assert_eq!(
        Verification::verify_option_ticker(ticker),
        Err(ErrorCode::TickerError)
    );
}

#[test]
fn test_verify_options_ticker() {
    let mut parameters = Parameters::default();
    parameters.ticker = Some(String::from("O:AAL210820C00014000"));
    assert_eq!(
        Verification::verify_options_ticker(true, &parameters),
        Ok(())
    );
    parameters.ticker = Some(String::from("A"));
    assert_eq!(
        Verification::verify_options_ticker(true, &parameters),
        Err(ErrorCode::TickerError)
    );
    parameters.ticker = None;
    assert_eq!(
        Verification::verify_options_ticker(true, &parameters),
        Err(ErrorCode::TickerNotSet)
    );
    assert_eq!(
        Verification::verify_options_ticker(false, &parameters),
        Ok(())
    );
}

#[test]
fn test_verify_indicie_ticker() {
    let ticker = String::from("I:DJI");
    assert_eq!(Verification::verify_indicie_ticker(ticker), Ok(()));
    let ticker = String::from("A");
    assert_eq!(
        Verification::verify_indicie_ticker(ticker),
        Err(ErrorCode::TickerError)
    );
}

#[test]
fn test_verify_indices_ticker() {
    let mut parameters = Parameters::default();
    parameters.ticker = Some(String::from("I:DJI"));
    assert_eq!(
        Verification::verify_indices_ticker(true, &parameters),
        Ok(())
    );
    parameters.ticker = Some(String::from("A"));
    assert_eq!(
        Verification::verify_indices_ticker(true, &parameters),
        Err(ErrorCode::TickerError)
    );
    parameters.ticker = None;
    assert_eq!(
        Verification::verify_indices_ticker(true, &parameters),
        Err(ErrorCode::TickerNotSet)
    );
    assert_eq!(
        Verification::verify_indices_ticker(false, &parameters),
        Ok(())
    );
}

#[test]
fn test_verify_forex_ticker() {
    let ticker = String::from("C:EURUSD");
    assert_eq!(Verification::verify_forex_ticker(ticker), Ok(()));
    let ticker = String::from("A");
    assert_eq!(
        Verification::verify_forex_ticker(ticker),
        Err(ErrorCode::TickerError)
    );
}

#[test]
fn test_verify_forex_tickers() {
    let mut parameters = Parameters::default();
    parameters.ticker = Some(String::from("C:EURUSD"));
    assert_eq!(
        Verification::verify_forex_tickers(true, &parameters),
        Ok(())
    );
    parameters.ticker = Some(String::from("A"));
    assert_eq!(
        Verification::verify_forex_tickers(true, &parameters),
        Err(ErrorCode::TickerError)
    );
    parameters.ticker = None;
    assert_eq!(
        Verification::verify_forex_tickers(true, &parameters),
        Err(ErrorCode::TickerNotSet)
    );
    assert_eq!(
        Verification::verify_forex_tickers(false, &parameters),
        Ok(())
    );
}

#[test]
fn test_verify_crypto_ticker() {
    let ticker = String::from("X:BTCUSD");
    assert_eq!(Verification::verify_crypto_ticker(ticker), Ok(()));
    let ticker = String::from("A");
    assert_eq!(
        Verification::verify_crypto_ticker(ticker),
        Err(ErrorCode::TickerError)
    );
}

#[test]
fn test_verify_crypto_tickers() {
    let mut parameters = Parameters::default();
    parameters.ticker = Some(String::from("X:BTCUSD"));
    assert_eq!(
        Verification::verify_crypto_tickers(true, &parameters),
        Ok(())
    );
    parameters.ticker = Some(String::from("A"));
    assert_eq!(
        Verification::verify_crypto_tickers(true, &parameters),
        Err(ErrorCode::TickerError)
    );
    parameters.ticker = None;
    assert_eq!(
        Verification::verify_crypto_tickers(true, &parameters),
        Err(ErrorCode::TickerNotSet)
    );
    assert_eq!(
        Verification::verify_crypto_tickers(false, &parameters),
        Ok(())
    );
}

#[test]
fn test_get_ticker_type() {
    let ticker = String::from("X:BTCUSD");
    assert_eq!(
        Verification::get_ticker_type(&ticker),
        Ok(TickerType::Crypto)
    );
    let ticker = String::from("C:EURUSD");
    assert_eq!(
        Verification::get_ticker_type(&ticker),
        Ok(TickerType::Forex)
    );
    let ticker = String::from("I:DJI");
    assert_eq!(
        Verification::get_ticker_type(&ticker),
        Ok(TickerType::Indicies)
    );
    let ticker = String::from("O:AAL210820C00014000");
    assert_eq!(
        Verification::get_ticker_type(&ticker),
        Ok(TickerType::Options)
    );
    let ticker = String::from("AAPL");
    assert_eq!(
        Verification::get_ticker_type(&ticker),
        Ok(TickerType::Stocks)
    );
    let ticker = String::from("$A");
    assert_eq!(
        Verification::get_ticker_type(&ticker),
        Err(ErrorCode::TickerError)
    );
}

#[test]
fn test_verify_ticker() {
    let ticker_types = TickerTypes::set(false, false, false, false, false);
    let mut parameters = Parameters::default();
    parameters.ticker = Some(String::from("X:BTCUSD"));
    assert_eq!(
        Verification::verify_ticker(true, &ticker_types, &parameters),
        Err(ErrorCode::TickerNotValidForAPICall)
    );
    parameters.ticker = Some(String::from("C:EURUSD"));
    assert_eq!(
        Verification::verify_ticker(true, &ticker_types, &parameters),
        Err(ErrorCode::TickerNotValidForAPICall)
    );
    parameters.ticker = Some(String::from("I:DJI"));
    assert_eq!(
        Verification::verify_ticker(true, &ticker_types, &parameters),
        Err(ErrorCode::TickerNotValidForAPICall)
    );
    parameters.ticker = Some(String::from("O:AAL210820C00014000"));
    assert_eq!(
        Verification::verify_ticker(true, &ticker_types, &parameters),
        Err(ErrorCode::TickerNotValidForAPICall)
    );
    parameters.ticker = Some(String::from("AAPL"));
    assert_eq!(
        Verification::verify_ticker(true, &ticker_types, &parameters),
        Err(ErrorCode::TickerNotValidForAPICall)
    );
    parameters.ticker = Some(String::from("$A"));
    assert_eq!(
        Verification::verify_ticker(true, &ticker_types, &parameters),
        Err(ErrorCode::TickerError)
    );
    parameters.ticker = None;
    assert_eq!(
        Verification::verify_ticker(true, &ticker_types, &parameters),
        Err(ErrorCode::TickerNotSet)
    );
    assert_eq!(
        Verification::verify_ticker(false, &ticker_types, &parameters),
        Ok(())
    );
}

#[test]
fn test_verify_tickers() {
    let ticker_types = TickerTypes::set(false, false, false, false, false);
    let mut parameters = Parameters::default();
    parameters.tickers = Some(vec![String::from("X:BTCUSD")]);
    assert_eq!(
        Verification::verify_tickers(true, &ticker_types, &parameters),
        Err(ErrorCode::TickerNotValidForAPICall)
    );
    parameters.tickers = Some(vec![String::from("C:EURUSD")]);
    assert_eq!(
        Verification::verify_tickers(true, &ticker_types, &parameters),
        Err(ErrorCode::TickerNotValidForAPICall)
    );
    parameters.tickers = Some(vec![String::from("I:DJI")]);
    assert_eq!(
        Verification::verify_tickers(true, &ticker_types, &parameters),
        Err(ErrorCode::TickerNotValidForAPICall)
    );
    parameters.tickers = Some(vec![String::from("O:AAL210820C00014000")]);
    assert_eq!(
        Verification::verify_tickers(true, &ticker_types, &parameters),
        Err(ErrorCode::TickerNotValidForAPICall)
    );
    parameters.tickers = Some(vec![String::from("AAPL")]);
    assert_eq!(
        Verification::verify_tickers(true, &ticker_types, &parameters),
        Err(ErrorCode::TickerNotValidForAPICall)
    );
    parameters.tickers = Some(vec![String::from("$A")]);
    assert_eq!(
        Verification::verify_tickers(true, &ticker_types, &parameters),
        Err(ErrorCode::TickerError)
    );
    parameters.tickers = None;
    assert_eq!(
        Verification::verify_tickers(true, &ticker_types, &parameters),
        Err(ErrorCode::TickersNotSet)
    );
    assert_eq!(
        Verification::verify_tickers(false, &ticker_types, &parameters),
        Ok(())
    );
}

#[test]
fn test_verify_underlying_asset() {
    let mut parameters = Parameters::default();
    parameters.underlying_asset = Some(String::from("X:BTCUSD"));
    assert_eq!(
        Verification::verify_underlying_asset(true, &parameters),
        Ok(())
    );
    parameters.underlying_asset = Some(String::from("C:EURUSD"));
    assert_eq!(
        Verification::verify_underlying_asset(true, &parameters),
        Ok(())
    );
    parameters.underlying_asset = Some(String::from("I:DJI"));
    assert_eq!(
        Verification::verify_underlying_asset(true, &parameters),
        Ok(())
    );
    parameters.underlying_asset = Some(String::from("O:AAL210820C00014000"));
    assert_eq!(
        Verification::verify_underlying_asset(true, &parameters),
        Ok(())
    );
    parameters.underlying_asset = Some(String::from("AAPL"));
    assert_eq!(
        Verification::verify_underlying_asset(true, &parameters),
        Ok(())
    );
    parameters.underlying_asset = Some(String::from("$A"));
    assert_eq!(
        Verification::verify_underlying_asset(true, &parameters),
        Err(ErrorCode::TickerError)
    );
    parameters.underlying_asset = None;
    assert_eq!(
        Verification::verify_underlying_asset(true, &parameters),
        Err(ErrorCode::UnderlyingAssetNotSet)
    );
    assert_eq!(
        Verification::verify_underlying_asset(false, &parameters),
        Ok(())
    );
}

#[test]
fn test_verify() {
    let date = Some(String::from("2020-12-01"));
    assert_eq!(Verification::verify(true, &date, &Parameter::Date), Ok(()));
    let date = Some(String::from("2020-12-01T"));
    assert_eq!(
        Verification::verify(true, &date, &Parameter::Date),
        Err(ErrorCode::DateError)
    );
    let include_otc: Option<bool> = None;
    assert_eq!(
        Verification::verify(true, &include_otc, &Parameter::IncludeOTC),
        Err(ErrorCode::IncludeOTCNotSet)
    );
}

#[test]
fn test_check_parameters() {
    let mut parameters = Parameters::default();
    parameters.api_key = String::from("12345678901234567890123456789012");
    let parameter_requirements = &[&ParameterRequirment {
        required: true,
        parameter: Parameter::Ticker,
    }];
    let ticker_types = TickerTypes::stocks();
    parameters.ticker = Some(String::from("AAPL"));
    assert_eq!(
        Verification::check_parameters(&ticker_types, parameter_requirements, &parameters),
        Ok(())
    );
    parameters.ticker = Some(String::from("O:AAL210820C00014000"));
    assert_eq!(
        Verification::check_parameters(&ticker_types, parameter_requirements, &parameters),
        Err(ErrorCode::TickerNotValidForAPICall)
    );
    parameters.ticker = None;
    assert_eq!(
        Verification::check_parameters(&ticker_types, parameter_requirements, &parameters),
        Err(ErrorCode::TickerNotSet)
    );
}
