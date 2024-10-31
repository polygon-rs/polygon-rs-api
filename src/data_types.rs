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
            return map.get("results").and_then(|v| v.as_array()).map(|v| {
                v.iter()
                    .map(|v| T::parse(v.clone().as_object().unwrap()))
                    .collect()
            });
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
            return map
                .get("results")
                .and_then(|v| v.as_array())
                .map(|v| v.iter().map(|v| v.as_i64().unwrap()).collect());
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
                    for size_object_key in size_object.keys() {
                        match size_object.get(size_object_key).and_then(|v| v.as_f64()) {
                            Some(value) => {
                                ask_hash_map.insert(size_object_key.clone(), value);
                            }
                            None => continue,
                        }
                    }
                    Some(ask_hash_map)
                }
                None => None,
            };
        }
        None
    }
}
