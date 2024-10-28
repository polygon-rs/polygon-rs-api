use crate::data_types::Parse;
use crate::rest::parameters::Timeframe;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Trade {
    pub conditions: Option<Vec<i64>>,
    pub exchange_id: Option<i64>,
    pub price: Option<f64>,
    pub sip_timestamp: Option<i64>,
    pub size: Option<i64>,
    pub trade_id: Option<String>,
    pub timeframe: Option<Timeframe>,
    pub exchange: Option<String>,
    pub trade_correction: Option<i64>,
    pub trf_timestamp: Option<i64>,
    pub sequence_number: Option<i64>,
    pub trf_id: Option<i64>,
    pub participant_timestamp: Option<i64>,
    pub tape: Option<i64>,
}

impl Parse for Trade {
    fn parse(map: &mut serde_json::Map<String, serde_json::Value>) -> Self {
        
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
        if let Some(exchange_id) = map.get("exchange") {
            map.insert(String::from("exchange_id"), exchange_id.clone());
        };
        if let Some(exchange_id) = map.get("x") {
            map.insert(String::from("exchange_id"), exchange_id.clone());
        };
        let exchange_id = map.get("exchange_id").and_then(|v| v.as_i64());
        if let Some(price) = map.get("p") {
            map.insert(String::from("price"), price.clone());
        };
        let price = map.get("price").and_then(|v| v.as_f64());
        if let Some(sip_timestamp) = map.get("timestamp") {
            map.insert(String::from("sip_timestamp"), sip_timestamp.clone());
        };
        if let Some(sip_timestamp) = map.get("t") {
            map.insert(String::from("sip_timestamp"), sip_timestamp.clone());
        };
        let sip_timestamp = map.get("sip_timestamp").and_then(|v| v.as_i64());
        if let Some(size) = map.get("s") {
            map.insert(String::from("size"), size.clone());
        };
        let size = map.get("size").and_then(|v| v.as_i64());
        if let Some(trade_id) = map.get("i") {
            map.insert(String::from("id"), trade_id.clone());
        };
        let trade_id = map.get("id").and_then(|v| v.as_str()).map(|v|{
            String::from(v)
        });
        let timeframe = map
            .get("timeframe")
            .and_then(|v| v.as_str())
            .map(|v| match v {
                "DELAYED" => Timeframe::Delayed,
                "REAL-TIME" => Timeframe::RealTime,
                _ => Timeframe::Unknown,
            });
        let exchange = map
            .get("T")
            .and_then(|v| v.as_str())
            .map(|v| v.to_string());
        if let Some(trade_correction) = map.get("e") {
            map.insert(String::from("correction"), trade_correction.clone());
        };
        let trade_correction = map.get("correction").and_then(|v| v.as_i64());
        if let Some(trf_timestamp) = map.get("f") {
            map.insert(String::from("trf_timestamp"), trf_timestamp.clone());
        };
        let trf_timestamp = map.get("trf_timestamp").and_then(|v| v.as_i64());
        if let Some(sequence_number) = map.get("q") {
            map.insert(String::from("sequence_number"), sequence_number.clone());
        };
        let sequence_number = map.get("sequence_number").and_then(|v| v.as_i64());
        if let Some(trf_id) = map.get("r") {
            map.insert(String::from("trf_id"), trf_id.clone());
        };
        let trf_id = map.get("trf_id").and_then(|v| v.as_i64());
        if let Some(participant_timestamp) = map.get("y") {
            map.insert(String::from("participant_timestamp"), participant_timestamp.clone());
        };
        let participant_timestamp = map.get("participant_timestamp").and_then(|v| v.as_i64());
        if let Some(tape) = map.get("z") {
            map.insert(String::from("tape"), tape.clone());
        };
        let tape = map.get("tape").and_then(|v| v.as_i64());
        Trade {
            conditions,
            exchange_id,
            price,
            sip_timestamp,
            size,
            trade_id,
            timeframe,
            exchange,
            trade_correction,
            trf_timestamp,
            sequence_number,
            trf_id,
            participant_timestamp,
            tape,
        }
    }
}
