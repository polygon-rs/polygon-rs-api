use crate::{data_types, rest::error::ErrorCode};
use serde_json::Value;

pub struct Request {}

impl Request {
    #[tokio::main]
    pub async fn request(url: String) -> Result<serde_json::Map<String, Value>, ErrorCode> {
        let request = reqwest::get(url).await;
        let r = match request {
            Ok(response) => {
                let response_text = response.text().await;
                match response_text {
                    Ok(text) => text,
                    Err(e) => {
                        println!("{}", e);
                        return Err(ErrorCode::RequestError);
                    }
                }
            }
            Err(e) => {
                println!("{}", e);
                return Err(ErrorCode::RequestError);
            }
        };
        match serde_json::from_str(r.as_str()) {
            Ok(map) => Ok(map),
            Err(err) => {
                println!("{}", err);
                return Err(ErrorCode::JSONParseError);
            }
        }
    }
    pub fn next<T: data_types::Parse>(
        url: Option<String>,
        api_key: String,
    ) -> Result<T, ErrorCode> {
        if url.is_none() {
            return Err(ErrorCode::NoNextURL);
        }
        let next_url = if let Some(next_url) = url {
            format!("{}&apiKey={}", next_url, api_key)
        } else {
            return Err(ErrorCode::NoNextURL);
        };
        match Self::request(next_url) {
            Ok(mut map) => Ok(T::parse(&mut map)),
            Err(e) => return Err(e),
        }
    }
}
