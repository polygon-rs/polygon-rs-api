use crate::{data_types, rest::error::ErrorCode};
use serde_json::Value;

pub trait Request {
    #[tokio::main]
    async fn request(&self, url: String) -> Result<serde_json::Map<String, Value>, ErrorCode> {
        let r = match reqwest::get(url).await {
            Ok(response) => match response.text().await {
                Ok(text) => text,
                Err(e) => {
                    println!("{}", e);
                    return Err(ErrorCode::RequestError);
                }
            },
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


}

pub trait Next {
    fn next<T: data_types::Parse>(url: Option<String>, api_key: String, request: &impl Request) -> Result<T, ErrorCode> {
        if url.is_none() {
            return Err(ErrorCode::NoNextURL);
        }
        let next_url = if let Some(next_url) = url {
            format!("{}&apiKey={}",next_url, api_key)
        } else { return Err(ErrorCode::NoNextURL); };
        match request.request(next_url) {
            Ok(mut map) =>  Ok(T::parse(&mut map)),
            Err(e) => return Err(e),
        }
    }
}
