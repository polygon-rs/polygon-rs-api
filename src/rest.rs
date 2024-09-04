pub mod market;
pub mod reference;
use strum::IntoEnumIterator; 
use strum_macros::EnumIter;

#[derive(serde::Deserialize)]
pub enum Rest {
    Market(market::Market),
}