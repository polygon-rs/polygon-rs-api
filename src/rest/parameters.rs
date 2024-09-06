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
    pub limit: Option<u16>,
    pub date: Option<String>,
    pub verbose: Option<bool>,
    pub contract_type: Option<ContractType>,
    pub include_otc: Option<bool>,
}

/*impl Default for Parameters {
    fn default() -> Parameters {
        Parameters {
            api_key: String::from(""),
            ticker: None,
            multiplier: None,
            timespan: None,
            from: None,
            to: None,
            adjusted: None,
            sort: None,
            limit: None,
            date: None,
            verbose: None,
        }
    }
}
*/
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
