pub mod data_types;
pub mod rest;
pub mod tools;
pub mod web_socket;

use rest::RestRequest;

use crate::rest::market::{daily::DailyRequest, previous::PreviousRequest};

pub struct Polygon {
    pub api_key: String,
    pub rest: RestRequest,
}

pub struct Stocks {}

impl DailyRequest for Stocks {}

impl PreviousRequest for Stocks {}