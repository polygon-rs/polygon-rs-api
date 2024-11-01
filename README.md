# polygon-rs-api

Unofficial Library for interacting with the polygon.io api.

This library has no affliation with polygon.io and makes no effort to do so.  Use at your own risk.

```rust
use polygon_rs_api::rest::market::{daily::{Daily, DailyRequest}, previous::{Previous, PreviousRequest}};

fn main() {
    let api_key = String::from("<API_KEY>");
    
    
    if let Ok(previous) = Previous::get_previous(&api_key, String::from("AAPL"), None) {
        match serde_json::to_string(&previous) {
            Ok(prev) => println!("{}", prev),
            Err(e) => println!("Error: {}", e),
        }
    }

    if let Ok(daily) = Daily::get_daily(&api_key, String::from("AAPL"), String::from("2024-10-29"), None){
        match serde_json::to_string(&daily){
            Ok(daily) => println!("{}", daily),
            Err(e) => println!("Error: {}", e),
        }
    }
}
```