pub mod data_types;
pub mod rest;
pub mod tools;
pub mod web_socket;

use rest::RestRequest;

pub struct Polygon {
    pub api_key: String,
}

impl Polygon {}

impl RestRequest for Polygon {}
