pub mod stocks;
pub mod options;
pub mod indices;
pub mod forex;
pub mod crypto;
use serde::{Deserialize, Serialize};
use crate::{Stocks, Options, Indices, Forex, Crypto};


#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Secuirty {
    Stocks(Stocks),
    Options(Options),
    Indices(Indices),
    Forex(Forex),
    Crypto(Crypto),
}