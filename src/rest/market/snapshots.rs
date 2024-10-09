pub mod chain;
pub mod contract;
pub mod gainers_losers;
pub mod indices;
pub mod l2;
pub mod ticker;
pub mod tickers;
pub mod universal;

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
