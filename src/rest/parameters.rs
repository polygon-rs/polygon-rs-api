use serde::{Deserialize, Serialize};
use strum_macros::Display;

#[derive(Clone, Debug)]
pub enum Parameter {
    Ticker,
    Date,
    Adjusted,
    Sort,
    Limit,
    Timespan,
    From,
    To,
    Multiplier,
    IncludeOTC,
    OptionsTicker,
    Order,
    ContractType,
    Timestamp,
    Sortv3,
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
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ContractType {
    Call,
    Put,
}

#[derive(Serialize, Deserialize, Clone, Debug, Display)]
pub enum Sort {
    Asc,
    Desc,
}
#[derive(Serialize, Deserialize, Clone, Debug, Display)]
pub enum Order {
    Asc,
    Desc,
}
#[derive(Serialize, Deserialize, Clone, Debug, Display)]
pub enum Sortv3 {
    Timestamp,
}

#[derive(Serialize, Deserialize, Clone, Debug, Display)]
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
