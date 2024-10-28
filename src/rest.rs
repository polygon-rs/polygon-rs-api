pub mod error;
pub mod market;
pub mod parameters;
pub mod reference;

use market::MarketRequest;

pub enum Rest {
    Market(market::Market),
}

pub trait RestRequest {}

impl MarketRequest for dyn RestRequest {}
