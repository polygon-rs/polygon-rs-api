use crate::{polygon::error::ErrorCode, Polygon};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Daily {
    pub after_hours: Option<f64>,
    pub close: Option<f64>,
    pub from: Option<String>,
    pub high: Option<f64>,
    pub low: Option<f64>,
    pub open: Option<f64>,
    pub pre_market: Option<f64>,
    pub status: Option<String>,
    pub symbol: Option<String>,
    pub volume: Option<f64>,
}

impl Daily {
    #[tokio::main]
    pub async fn daily(p: Polygon) -> Result<Daily, ErrorCode> {
        match p.verify_ticker() {
            Ok(t) => (),
            Err(e) => return Err(ErrorCode::TickerError),
        };
        match p.verify_date() {
            Ok(d) => (),
            Err(e) => return Err(ErrorCode::DateError),
        };
        let mut url_options = String::from("");
        match p.adjusted {
            Some(a) => match a {
                true => url_options = format!("{}?adjusted=true&", url_options),
                false => url_options = format!("{}?adjusted=false&", url_options),
            },
            None => (),
        }
        let url = format!(
            "https://api.polygon.io/v1/open-close/{}/{}{}?apiKey={}",
            p.clone().ticker.unwrap(),
            url_options,
            p.clone().date.unwrap(),
            p.api_key
        );
        match p.request(url) {
            Ok(response) => match serde_json::from_str(response.as_str()) {
                Ok(daily) => Ok(daily),
                Err(e) => return Err(ErrorCode::FormatError),
            },
            Err(e) => return Err(ErrorCode::RequestError),
        }
    }
}
