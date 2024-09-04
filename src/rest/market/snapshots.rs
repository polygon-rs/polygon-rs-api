pub mod chain;
pub mod ticker;
pub mod tickers;
pub mod universal;
pub mod gainers_losers;
pub mod contract;
pub mod indices;
pub mod l2;

#[derive(serde::Deserialize)]
pub enum Snapshots {
    Chain(chain::Chain),
    Ticker,
    Tickers,
    Universal,
    GainersLosers,
    Contract,
    Indices,
    L2,
}