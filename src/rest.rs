pub mod error;
pub mod market;
pub mod parameters;
pub mod reference;

pub enum Rest {
    Market(market::Market),
}
