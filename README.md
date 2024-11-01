# polygon-rs-api

Unofficial Library for interacting with the polygon.io api.

This library has no affliation with polygon.io and makes no effort to do so.  Use at your own risk.

```rust
use polygon_rs_api::{rest::{self, market::{daily::DailyRequest, previous::PreviousRequest}}, Polygon};

let polygon_api = Polygon{ api_key: String::from("<API_KEY>"), rest: rest::RestRequest{}};

if let Ok(previous) = polygon_api.rest.get_previous(String::from(polygon_api.api_key.clone()), String::from("AAPL"), None) {
        match serde_json::to_string(&previous) {
            Ok(prev) => println!("{}", prev),
            Err(e) => println!("Error: {}", e),
        }
    }

    if let Ok(daily) = polygon_api.rest.get_daily(String::from(polygon_api.api_key.clone()), String::from("AAPL"), String::from("2024-10-29"), None){
        match serde_json::to_string(&daily){
            Ok(daily) => println!("{}", daily),
            Err(e) => println!("Error: {}", e),
        }
    }

```