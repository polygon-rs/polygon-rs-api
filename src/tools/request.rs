use crate::rest::error::ErrorCode;
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
