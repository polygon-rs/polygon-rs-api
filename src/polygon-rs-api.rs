#![allow(unused)]
#![allow(non_snake_case)] //Required so response objects can be properly parsed

pub mod rest;
pub mod web_socket;

use crate::rest::{
    error::ErrorCode,
    market::{daily::Daily, nbbo::NBBO, snapshots::chain::Chain, trades::Trades},
    parameters::{ContractType, Parameter, ParameterRequirment, Parameters, Sort, Timespan},
    reference, Request,
};
