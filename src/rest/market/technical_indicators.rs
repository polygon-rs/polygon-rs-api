pub mod ema;
pub mod macd;
pub mod rsi;
pub mod sma;

#[derive(serde::Deserialize)]
pub enum TechnicalIndicators {
    EMA,
    MACD,
    RSI,
    SMA,
}
