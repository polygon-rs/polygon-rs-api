pub mod rest;
pub mod web_socket;

use crate::rest::{
    error::ErrorCode,
    parameters::{Order, Parameter, ParameterRequirment, Parameters, Sort, Sortv3, Timespan, ContractType, ContractStyle, Timeframe},
    Request,
};
