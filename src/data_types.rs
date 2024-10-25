pub mod contract;
pub mod greeks;
pub mod day;
pub mod details;
pub mod quote;
pub mod trade;
pub mod underlying_asset;

pub trait Parse {
    fn parse(map: &serde_json::Map<String, serde_json::Value>) -> Self;
}
