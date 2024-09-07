//#![allow(unused)]
#![allow(non_snake_case)] //Required so response objects can be properly parsed

pub mod rest;
pub mod web_socket;

use crate::rest::{
    error::ErrorCode,
    market::{
        daily::Daily, quotes::Quotes, snapshots::chain::Chain,
        technical_indicators::TechnicalIndicators, trades::Trades,
    },
    parameters::{
        ContractType, Order, Parameter, ParameterRequirment, Parameters, Sort, Sortv3, Timespan,
    },
    reference, Request,
};
