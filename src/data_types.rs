pub mod ask;
pub mod bar;
pub mod bid;
pub mod contract;
pub mod day;
pub mod details;
pub mod greeks;
pub mod indicie;
pub mod l2;
pub mod macd;
pub mod min;
pub mod moving_average;
pub mod quote;
pub mod relative_strength;
pub mod session;
pub mod ticker;
pub mod trade;
pub mod underlying_asset;
pub mod universal;

pub trait Parse {
    fn parse(map: &mut serde_json::Map<String, serde_json::Value>) -> Self;
}
