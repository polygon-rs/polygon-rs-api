use std::collections::HashMap;

pub mod ask;
pub mod bar;
pub mod bid;
pub mod contract;
pub mod day;
pub mod details;
pub mod greeks;
pub mod indicie;
pub mod l2;
pub mod macd;
pub mod min;
pub mod moving_average;
pub mod quote;
pub mod relative_strength;
pub mod session;
pub mod ticker;
pub mod trade;
pub mod underlying_asset;
pub mod universal;

pub trait Parse {
    fn parse(map: &serde_json::Map<String, serde_json::Value>) -> Self;

    fn bool_parse(
        map: &serde_json::Map<String, serde_json::Value>,
        keys: Vec<&str>,
    ) -> Option<bool> {
        for key in keys {
            if !map.contains_key(key) {
                continue;
            }
            return map.get(key).and_then(|v| v.as_bool());
        }
        None
    }

    fn string_parse(
        map: &serde_json::Map<String, serde_json::Value>,
        keys: Vec<&str>,
    ) -> Option<String> {
        for key in keys {
            if !map.contains_key(key) {
                continue;
            }
            return map.get(key).and_then(|v| v.as_str()).map(|v| v.to_string());
        }
        None
    }

    fn f64_parse(map: &serde_json::Map<String, serde_json::Value>, keys: Vec<&str>) -> Option<f64> {
        for key in keys {
            if !map.contains_key(key) {
                continue;
            }
            return map.get(key).and_then(|v| v.as_f64());
        }
        None
    }

    fn i64_parse(map: &serde_json::Map<String, serde_json::Value>, keys: Vec<&str>) -> Option<i64> {
        for key in keys {
            if !map.contains_key(key) {
                continue;
            }
            return map.get(key).and_then(|v| v.as_i64());
        }
        None
    }

    fn object_parse<T: Parse>(
        map: &serde_json::Map<String, serde_json::Value>,
        keys: Vec<&str>,
    ) -> Option<T> {
        for key in keys {
            if !map.contains_key(key) {
                continue;
            }
            return map
                .get(key)
                .and_then(|v| v.as_object())
                .map(|v| T::parse(v));
        }
        None
    }

    fn object<'a>(
        map: &'a serde_json::Map<String, serde_json::Value>,
        keys: Vec<&'a str>,
    ) -> Option<&'a serde_json::Map<String, serde_json::Value>> {
        for key in keys {
            if !map.contains_key(key) {
                continue;
            }
            return map.get(key).and_then(|v| v.as_object());
        }
        None
    }

    fn array_parse<T: Parse>(
        map: &serde_json::Map<String, serde_json::Value>,
        keys: Vec<&str>,
    ) -> Option<Vec<T>> {
        for key in keys {
            if !map.contains_key(key) {
                continue;
            }
            let mut array = Vec::new();
            if let Some(values) = map.get(key).and_then(|v| v.as_array()) {
                for object in values {
                    match object.as_object() {
                        Some(o) => array.push(T::parse(o)),
                        None => (),
                    }
                }
            };
            return Some(array);
        }
        None
    }

    fn array_i64_parse(
        map: &serde_json::Map<String, serde_json::Value>,
        keys: Vec<&str>,
    ) -> Option<Vec<i64>> {
        for key in keys {
            if !map.contains_key(key) {
                continue;
            }
            let mut i64_array = Vec::new();
            if let Some(values) = map.get(key).and_then(|v| v.as_array()) {
                for i64 in values {
                    match i64.as_i64() {
                        Some(i) => i64_array.push(i),
                        None => (),
                    }
                }
            };
            return Some(i64_array);
        }
        None
    }

    fn hashmap_parse(
        map: &serde_json::Map<String, serde_json::Value>,
        keys: Vec<&str>,
    ) -> Option<HashMap<String, f64>> {
        for key in keys {
            if !map.contains_key(key) {
                continue;
            }
            return match map.get("size").and_then(|v| v.as_object()) {
                Some(size_object) => {
                    let mut ask_hash_map = HashMap::new();
                    size_object.keys().for_each(|v| {
                        match size_object.get(v).and_then(|v| v.as_f64()) {
                            Some(value) => {
                                ask_hash_map.insert(v.to_string(), value);
                            }
                            None => (),
                        };
                    });
                    Some(ask_hash_map)
                }
                None => None,
            };
        }
        None
    }
}
