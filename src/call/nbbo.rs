use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NBBO {
    pub next_url: Option<String>,
    pub request_id: Option<String>,
    pub results: Option<Vec<Quote>>,
    pub status: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Quote {
    pub ask_exchange: Option<i32>,
    pub ask_price: Option<f64>,
    pub ask_size: Option<i32>,
    pub bid_exchange: Option<i32>,
    pub bid_price: Option<f64>,
    pub bid_size: Option<i32>,
    pub indicators: Option<Vec<i32>>,
    pub participant_timestamp: Option<i64>,
    pub sequence_number: Option<i64>,
    pub sip_timestamp: Option<i64>,
    pub tape: Option<i32>,
}