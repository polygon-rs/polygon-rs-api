use serde::{Deserialize, Serialize};
use strum_macros::Display;

#[derive(Serialize, Deserialize, Clone, Debug, Display, Copy, PartialEq)]
pub enum TickerType {
    Stocks,
    Options,
    Indicies,
    Forex,
    Crypto,
}

#[derive(Serialize, Deserialize, Clone, Debug, Copy)]
pub struct TickerTypes {
    pub stocks: bool,
    pub options: bool,
    pub indicies: bool,
    pub forex: bool,
    pub crypto: bool,
}

impl TickerTypes {
    pub fn set(stocks: bool, options: bool, indicies: bool, forex: bool, crypto: bool) -> Self {
        Self {
            stocks: stocks,
            options: options,
            indicies: indicies,
            forex: forex,
            crypto: crypto,
        }
    }

    pub fn stocks() -> Self {
        Self {
            stocks: true,
            options: false,
            indicies: false,
            forex: false,
            crypto: false,
        }
    }

    pub fn options() -> Self {
        Self {
            stocks: false,
            options: true,
            indicies: false,
            forex: false,
            crypto: false,
        }
    }

    pub fn indicies() -> Self {
        Self {
            stocks: false,
            options: false,
            indicies: true,
            forex: false,
            crypto: false,
        }
    }

    pub fn forex() -> Self {
        Self {
            stocks: false,
            options: false,
            indicies: false,
            forex: true,
            crypto: false,
        }
    }

    pub fn crypto() -> Self {
        Self {
            stocks: false,
            options: false,
            indicies: false,
            forex: false,
            crypto: true,
        }
    }

    pub fn all() -> Self {
        Self {
            stocks: true,
            options: true,
            indicies: true,
            forex: true,
            crypto: true,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Parameter {
    Ticker,
    Tickers,
    TickerFrom,
    TickerTo,
    TickerType,
    Date,
    Adjusted,
    Sort,
    Limit, //Configure per request Max limits as it can be different depending on the call
    Timespan,
    From,
    To,
    Multiplier,
    IncludeOTC,
    Order,
    ContractType,
    Timestamp,
    Sortv3,
    StrikePrice,
    StrikePriceFrom,
    StrikePriceTo,
    Amount,
    Precision,
    Direction,
    UnderlyingAsset,
    Window,
    SeriesType,
    ExpandUnderlying,
    LongWindow,
    ShortWindow,
    SignalWindow,
    Expired,
    AsOf,
}

#[derive(Clone, Debug)]
pub struct ParameterRequirment {
    pub required: bool,
    pub parameter: Parameter,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Parameters {
    pub api_key: String,
    pub ticker: Option<String>,
    pub tickers: Option<Vec<String>>,
    pub ticker_from: Option<String>,
    pub ticker_to: Option<String>,
    pub multiplier: Option<u16>,
    pub timespan: Option<Timespan>,
    pub from: Option<String>,
    pub to: Option<String>,
    pub adjusted: Option<bool>,
    pub sort: Option<Sort>,
    pub order: Option<Order>,
    pub sortv3: Option<Sortv3>,
    pub timestamp: Option<String>,
    pub limit: Option<u16>,
    pub date: Option<String>,
    pub verbose: Option<bool>,
    pub contract_type: Option<ContractType>,
    pub include_otc: Option<bool>,
    pub strike_price: Option<f64>,
    pub strike_price_from: Option<f64>,
    pub strike_price_to: Option<f64>,
    pub amount: Option<f64>,
    pub precision: Option<u8>,
    pub direction: Option<Direction>,
    pub underlying_asset: Option<String>,
    pub ticker_type: Option<TickerType>,
    pub window: Option<i64>,
    pub long_window: Option<i64>,
    pub short_window: Option<i64>,
    pub signal_window: Option<i64>,
    pub series_type: Option<SeriesType>,
    pub expand_underlying: Option<bool>,
    pub expired: Option<bool>,
    pub as_of: Option<String>,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug, Display, PartialEq)]
pub enum ContractType {
    Call,
    Put,
    Other,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug, Display, PartialEq)]
pub enum ContractStyle {
    American,
    European,
    Bermudan,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug, Display, PartialEq)]
pub enum Timeframe {
    Delayed,
    RealTime,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug, Display)]
pub enum Sort {
    Asc,
    Desc,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug, Display)]
pub enum Order {
    Asc,
    Desc,
}
#[derive(Serialize, Deserialize, Copy, Clone, Debug, Display)]
pub enum Sortv3 {
    Timestamp,
    Ticker,
    ExpirationDate,
    StrikePrice,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug, Display)]
pub enum Timespan {
    Second,
    Minute,
    Hour,
    Day,
    Week,
    Month,
    Quater,
    Year,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug, Display)]
pub enum Direction {
    Gainers,
    Losers,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug, Display)]
pub enum SeriesType {
    Open,
    High,
    Low,
    Close,
}
