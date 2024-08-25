use crate::polygon::{timespan::Timespan, sort::Sort};
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Polygon {
    pub api_key: Option<String>,
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
}

impl Polygon {
    pub fn polygon(
        api_key: Option<String>,
        ticker: Option<String>,
        multiplier: Option<u16>,
        timespan: Option<Timespan>,
        from: Option<String>,
        to: Option<String>,
        adjusted: Option<bool>,
        sort: Option<Sort>,
        limit: Option<u16>,
        date: Option<String>,
        verbose: Option<bool>,
    ) -> Polygon {
        Polygon {
            api_key,
            ticker,
            multiplier,
            timespan,
            from,
            to,
            adjusted,
            sort,
            limit,
            date,
            verbose
        }
    }
}
