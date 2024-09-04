#![allow(unused)]

pub mod rest;
pub mod web_socket;

use crate::rest::{
    error::ErrorCode,
    market::{daily::Daily, nbbo::NBBO, snapshots::chain::Chain, trades::Trades},
    parameters::{ContractType, Parameter, ParameterRequirment, Parameters, Sort, Timespan},
    reference, Request,
};
