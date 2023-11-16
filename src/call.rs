pub mod nbbo;
pub mod daily;
pub mod trades;
use serde::{Deserialize, Serialize};
use crate::{NBBO, Daily, Trades};

# [derive(Serialize, Deserialize, Clone, Debug)]
pub enum Call {
    Daily(Daily),
    NBBO(NBBO),
    Trades(Trades)
}