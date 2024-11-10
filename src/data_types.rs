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

struct ParseTests {}

impl Parse for ParseTests {
    fn parse(map: &serde_json::Map<String, serde_json::Value>) -> Self {
        ParseTests {}
    }
}

    #[test]
    fn test_bool_parse_found() {
        let mut map: serde_json::Map<String, serde_json::Value> = serde_json::Map::new();
        map.insert("key1".to_string(), serde_json::json!(true));
        let result: Option<bool> = ParseTests::bool_parse(&map, vec!["key1", "key2"]);
        assert_eq!(result, Some(true));
    }

    #[test]
    fn test_bool_parse_not_found() {
        let mut map: serde_json::Map<String, serde_json::Value> = serde_json::Map::new();
        map.insert("key1".to_string(), serde_json::json!(true));
        let result = ParseTests::bool_parse(&map, vec!["key2"]);
        assert_eq!(result, None);
    }

    #[test]
    fn test_string_parse_found() {
        let mut map: serde_json::Map<String, serde_json::Value> = serde_json::Map::new();
        map.insert("key1".to_string(), serde_json::json!("value1"));
        let result = ParseTests::string_parse(&map, vec!["key1", "key2"]);
        assert_eq!(result, Some("value1".to_string()));
    }

    #[test]
    fn test_string_parse_not_found() {
        let mut map: serde_json::Map<String, serde_json::Value> = serde_json::Map::new();
        map.insert("key1".to_string(), serde_json::json!("value1"));
        let result = ParseTests::string_parse(&map, vec!["key2"]);
        assert_eq!(result, None);
    }

    #[test]
    fn test_f64_parse_found() {
        let mut map: serde_json::Map<String, serde_json::Value> = serde_json::Map::new();
        map.insert("key1".to_string(), serde_json::json!(1.23));
        let result = ParseTests::f64_parse(&map, vec!["key1", "key2"]);
        assert_eq!(result, Some(1.23));
    }

    #[test]
    fn test_f64_parse_not_found() {
        let mut map: serde_json::Map<String, serde_json::Value> = serde_json::Map::new();
        map.insert("key1".to_string(), serde_json::json!(1.23));
        let result = ParseTests::f64_parse(&map, vec!["key2"]);
        assert_eq!(result, None);
    }

    #[test]
    fn test_i64_parse_found() {
        let mut map: serde_json::Map<String, serde_json::Value> = serde_json::Map::new();
        map.insert("key1".to_string(), serde_json::json!(123));
        let result = ParseTests::i64_parse(&map, vec!["key1", "key2"]);
        assert_eq!(result, Some(123));
    }

    #[test]
    fn test_i64_parse_not_found() {
        let mut map: serde_json::Map<String, serde_json::Value> = serde_json::Map::new();
        map.insert("key1".to_string(), serde_json::json!(123));
        let result = ParseTests::i64_parse(&map, vec!["key2"]);
        assert_eq!(result, None);
    }

    /*#[test]
    fn test_object_parse_found() {
        let mut map: serde_json::Map<String, serde_json::Value> = serde_json::Map::new();
        let mut inner_map: serde_json::Map<String, serde_json::Value> = serde_json::Map::new();
        inner_map.insert("inner_key".to_string(), serde_json::json!("inner_value"));
        map.insert("key1".to_string(), serde_json::json!(inner_map));
        let result = ParseTests::object_parse::<HashMap<String, String>>(
            &map,
            vec!["key1", "key2"],
        );
        assert_eq!(
            result,
            Some(HashMap::from([("inner_key".to_string(), "inner_value".to_string())]))
        );
    }

    #[test]
    fn test_object_parse_not_found() {
        let mut map: serde_json::Map<String, serde_json::Value> = serde_json::Map::new();
        let mut inner_map: serde_json::Map<String, serde_json::Value> = serde_json::Map::new();
        inner_map.insert("inner_key".to_string(), serde_json::json!("inner_value"));
        map.insert("key1".to_string(), serde_json::json!(inner_map));
        let result = ParseTests::object_parse::<HashMap<String, String>>(&map, vec!["key2"]);
        assert_eq!(result, None);
    }

    #[test]
    fn test_array_parse_found() {
        let mut map: serde_json::Map<String, serde_json::Value> = serde_json::Map::new();
        let mut inner_map1: serde_json::Map<String, serde_json::Value> = serde_json::Map::new();
        inner_map1.insert("inner_key".to_string(), serde_json::json!("inner_value1"));
        let mut inner_map2: serde_json::Map<String, serde_json::Value> = serde_json::Map::new();
        inner_map2.insert("inner_key".to_string(), serde_json::json!("inner_value2"));
        map.insert(
            "key1".to_string(),
            serde_json::json!(vec![inner_map1, inner_map2]),
        );
        let result =
        ParseTests::array_parse::<HashMap<String, String>>(&map, vec!["key1", "key2"]);
        assert_eq!(
            result,
            Some(vec![
                HashMap::from([("inner_key".to_string(), "inner_value1".to_string())]),
                HashMap::from([("inner_key".to_string(), "inner_value2".to_string())])
            ])
        );
    }

    #[test]
    fn test_array_parse_not_found() {
        let mut map: serde_json::Map<String, serde_json::Value> = serde_json::Map::new();
        let mut inner_map1: serde_json::Map<String, serde_json::Value> = serde_json::Map::new();
        inner_map1.insert("inner_key".to_string(), serde_json::json!("inner_value1"));
        let mut inner_map2: serde_json::Map<String, serde_json::Value> = serde_json::Map::new();
        inner_map2.insert("inner_key".to_string(), serde_json::json!("inner_value2"));
        map.insert(
            "key1".to_string(),
            serde_json::json!(vec![inner_map1, inner_map2]),
        );
        let result = ParseTests::array_parse::<HashMap<String, String>>(&map, vec!["key2"]);
        assert_eq!(result, None);
    }*/

    #[test]
    fn test_array_i64_parse_found() {
        let mut map: serde_json::Map<String, serde_json::Value> = serde_json::Map::new();
        map.insert("key1".to_string(), serde_json::json!(vec![1, 2, 3]));
        let result = ParseTests::array_i64_parse(&map, vec!["key1", "key2"]);
        assert_eq!(result, Some(vec![1, 2, 3]));
    }

    #[test]
    fn test_array_i64_parse_not_found() {
        let mut map: serde_json::Map<String, serde_json::Value> = serde_json::Map::new();
        map.insert("key1".to_string(), serde_json::json!(vec![1, 2, 3]));
        let result = ParseTests::array_i64_parse(&map, vec!["key2"]);
        assert_eq!(result, None);
    }

    #[test]
    fn test_hashmap_parse_found() {
        let mut map: serde_json::Map<String, serde_json::Value> = serde_json::Map::new();
        let mut inner_map: serde_json::Map<String, serde_json::Value> = serde_json::Map::new();
        inner_map.insert("key1".to_string(), serde_json::json!(1.23));
        inner_map.insert("key2".to_string(), serde_json::json!(4.56));
        map.insert("size".to_string(), serde_json::json!(inner_map));
        let result = ParseTests::hashmap_parse(&map, vec!["size", "key2"]);
        assert_eq!(
            result,
            Some(HashMap::from([
                ("key1".to_string(), 1.23),
                ("key2".to_string(), 4.56)
            ]))
        );
    }

    #[test]
    fn test_hashmap_parse_not_found() {
        let mut map: serde_json::Map<String, serde_json::Value> = serde_json::Map::new();
        let mut inner_map: serde_json::Map<String, serde_json::Value> = serde_json::Map::new();
        inner_map.insert("key1".to_string(), serde_json::json!(1.23));
        inner_map.insert("key2".to_string(), serde_json::json!(4.56));
        map.insert("size".to_string(), serde_json::json!(inner_map));
        let result = ParseTests::hashmap_parse(&map, vec!["key2"]);
        assert_eq!(result, None);
    }
