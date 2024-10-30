use crate::data_types::Parse;
use crate::rest::parameters::Timeframe;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Quote {
    pub bid: Option<f64>,
    pub bid_size: Option<i64>,
    pub ask: Option<f64>,
    pub ask_size: Option<i64>,
    pub bid_exchange_id: Option<i64>,
    pub ask_exchange_id: Option<i64>,
    pub last_updated: Option<i64>,
    pub mid_point: Option<f64>,
    pub timeframe: Option<Timeframe>,
    pub exchange_id: Option<i64>,
    pub exchange: Option<String>,
    pub conditions: Option<Vec<i64>>,
    pub trf_timestamp: Option<i64>,
    pub indicators: Option<Vec<i64>>,
    pub sequence_number: Option<i64>,
    pub participant_timestamp: Option<i64>,
    pub tape: Option<i64>,
}

impl Parse for Quote {
    fn parse(map: &mut serde_json::Map<String, serde_json::Value>) -> Self {
        if let Some(bid) = map.get("p") {
            map.insert(String::from("bid"), bid.clone());
        };
        if let Some(bid) = map.get("b") {
            map.insert(String::from("bid"), bid.clone());
        };
        if let Some(bid) = map.get("bid_price") {
            map.insert(String::from("bid"), bid.clone());
        };
        let bid = map.get("bid").and_then(|v| v.as_f64());
        if let Some(bid_size) = map.get("s") {
            map.insert(String::from("bid_size"), bid_size.clone());
        };
        let bid_size = map.get("bid_size").and_then(|v| v.as_i64());
        if let Some(ask) = map.get("P") {
            map.insert(String::from("ask"), ask.clone());
        };
        if let Some(ask) = map.get("a") {
            map.insert(String::from("ask"), ask.clone());
        };
        if let Some(ask) = map.get("ask_price") {
            map.insert(String::from("ask"), ask.clone());
        };
        let ask = map.get("ask").and_then(|v| v.as_f64());
        if let Some(ask_size) = map.get("S") {
            map.insert(String::from("ask_size"), ask_size.clone());
        };
        let ask_size = map.get("ask_size").and_then(|v| v.as_i64());
        if let Some(bid_exchange) = map.get("bid_exchange") {
            map.insert(String::from("bid_exchange_id"), bid_exchange.clone());
        };
        let bid_exchange_id = map.get("bid_exchange_id").and_then(|v| v.as_i64());
        if let Some(ask_exchange) = map.get("ask_exchange") {
            map.insert(String::from("ask_exchange_id"), ask_exchange.clone());
        };
        let ask_exchange_id = map.get("ask_exchange_id").and_then(|v| v.as_i64());
        if let Some(last_updated) = map.get("t") {
            map.insert(String::from("last_updated"), last_updated.clone());
        };
        if let Some(last_updated) = map.get("sip_timestamp") {
            map.insert(String::from("last_updated"), last_updated.clone());
        };
        if let Some(last_updated) = map.get("timestamp") {
            map.insert(String::from("last_updated"), last_updated.clone());
        };
        let last_updated = map.get("last_updated").and_then(|v| v.as_i64());
        let mid_point = map.get("mid_point").and_then(|v| v.as_f64());
        let timeframe = map
            .get("timeframe")
            .and_then(|v| v.as_str())
            .map(|v| match v {
                "DELAYED" => Timeframe::Delayed,
                "REAL-TIME" => Timeframe::RealTime,
                _ => Timeframe::Unknown,
            });
        if let Some(exchange_id) = map.get("exchange") {
            map.insert(String::from("x"), exchange_id.clone());
        };
        let exchange_id = map.get("x").and_then(|v| v.as_i64());
        let exchange = map.get("T").and_then(|v| v.as_str()).map(|v| v.to_string());
        if let Some(conditions) = map.get("c") {
            map.insert(String::from("conditions"), conditions.clone());
        };
        let conditions = map.get("conditions").and_then(|v| v.as_array()).map(|v| {
            let mut conditions = Vec::new();
            for condition in v {
                if let Some(c) = condition.as_i64() {
                    conditions.push(c);
                }
            }
            conditions
        });
        if let Some(trf_timestamp) = map.get("f") {
            map.insert(String::from("trf_timestamp"), trf_timestamp.clone());
        };
        let trf_timestamp = map.get("trf_timestamp").and_then(|v| v.as_i64());
        if let Some(indicators) = map.get("i") {
            map.insert(String::from("indicators"), indicators.clone());
        };
        let indicators = map.get("indicators").and_then(|v| v.as_array()).map(|v| {
            let mut indicators = Vec::new();
            for indicator in v {
                if let Some(i) = indicator.as_i64() {
                    indicators.push(i);
                }
            }
            indicators
        });
        if let Some(sequence_number) = map.get("q") {
            map.insert(String::from("sequence_number"), sequence_number.clone());
        };
        let sequence_number = map.get("sequence_number").and_then(|v| v.as_i64());
        if let Some(participant_timestamp) = map.get("y") {
            map.insert(
                String::from("participant_timestamp"),
                participant_timestamp.clone(),
            );
        };
        let participant_timestamp = map.get("participant_timestamp").and_then(|v| v.as_i64());
        if let Some(tape) = map.get("z") {
            map.insert(String::from("tape"), tape.clone());
        };
        let tape = map.get("tape").and_then(|v| v.as_i64());

        Quote {
            bid,
            bid_size,
            ask,
            ask_size,
            bid_exchange_id,
            ask_exchange_id,
            last_updated,
            mid_point,
            timeframe,
            exchange_id,
            exchange,
            conditions,
            trf_timestamp,
            indicators,
            sequence_number,
            participant_timestamp,
            tape,
        }
    }
}
