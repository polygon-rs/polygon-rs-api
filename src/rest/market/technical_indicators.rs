pub mod ema;
pub mod macd;
pub mod rsi;
pub mod sma;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum TechnicalIndicators {
    EMA,
    MACD,
    RSI,
    SMA,
}
