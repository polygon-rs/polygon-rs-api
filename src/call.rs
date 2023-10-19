pub mod nbbo;
pub mod daily;
use serde::{Deserialize, Serialize};
use crate::{NBBO, Daily};

# [derive(Serialize, Deserialize, Clone, Debug)]
pub enum Call {
    Daily(Daily),
    NBBO(NBBO),
}