use polygon_rs_api::data_types::{Parse, ask::Ask};
use serde_json::{json, Map, Value};

#[test]
fn test_ask_parse() {
    let mut test_map: Map<String, Value> = Map::new();
    test_map.insert("price".to_string(), json!(1.1));
    let mut size_map: Map<String, Value> = Map::new();
    size_map.insert("test".to_string(), json!(2.2));
    test_map.insert("size".to_string(), json!(size_map));
    let ask = Ask::parse(&mut test_map);
    assert_eq!(ask.price.unwrap(), 1.1);
    assert_eq!(ask.size.unwrap().get("test").unwrap(), &2.2);
}

#[test]
fn test_ask_parse_empty() {
    let mut test_map: Map<String, Value> = Map::new();
    let ask = Ask::parse(&mut test_map);
    assert_eq!(ask.price, None);
    assert_eq!(ask.size, None);
}
