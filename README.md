# polygon-rs-api

Unofficial Library for interacting with the polygon.io api.

This library has no affliation with polygon.io and makes no effort to do so.  Use at your own risk.

```rust
use polygon_rs_api::{rest::market::{daily::{Daily, DailyRequest}, previous::PreviousRequest}, Stocks};

fn main() {
    let api_key = String::from("<API_KEY>");
    
    if let Ok(previous) = Stocks::get_previous(&api_key, String::from("AAPL"), None) {
        match serde_json::to_string(&previous) {
            Ok(prev) => println!("{}", prev),
            Err(e) => println!("Error: {}", e),
        }
    }

    let use_traits_for_custom_structs = MyStruct::new();
    println!("{:#?}", use_traits_for_custom_structs);
}

#[derive(Debug)]
pub struct MyStruct {
    pub daily: Daily,
}

impl DailyRequest for MyStruct {}

impl MyStruct {
    pub fn new() -> Self {
        let daily = Self::get_daily(&String::from("<API_KEY>"), String::from("AAPL"), String::from("2024-10-29"), None).unwrap();
        Self { daily }
    }
}
```