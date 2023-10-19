use serde::{Deserialize, Serialize};
use std::error::Error;
use crate::Polygon;
use crate::Call;
use crate::NBBO;
use crate::Daily;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Options {

}

impl Options {
    pub fn request(polygon: &Polygon) -> Result<Call, Box<dyn Error>> {
        match &polygon.call {
            Some(s) => match s {
                Call::NBBO(_) => Ok(Call::NBBO(Self::nbbo(polygon.clone()).unwrap())),
                Call::Daily(_) => Ok(Call::Daily(Self::daily(polygon.clone()).unwrap())),
            },
            None => panic!("There is no call type set"),
        }
    }

    #[tokio::main]
    async fn nbbo(p: Polygon) -> Result<NBBO, Box<dyn Error>> {
        let ticker = match p.ticker {
            Some(t) => t,
            None => panic!("There is no ticker set"),
        };
        let api_key = match p.api_key {
            Some(a) => a,
            None => panic!("There is no api key set"),
        };
        let url = format!("https://api.polygon.io/v3/quotes/{}?apiKey={}", ticker, api_key);
        Ok(serde_json::from_str(
            reqwest::get(url)
                .await
                .unwrap()
                .text()
                .await
                .unwrap()
                .as_str(),
        )
        .unwrap())
    }

    #[tokio::main]
    async fn daily(p: Polygon) -> Result<Daily, Box<dyn Error>> {
        let ticker = match p.ticker {
            Some(t) => t,
            None => panic!("There is no ticker set"),
        };
        let api_key = match p.api_key {
            Some(a) => a,
            None => panic!("There is no api key set"),
        };
        let date = match p.date {
            Some(d) => d,
            None => panic!("There is no date set"),
        };
        let url = format!("https://api.polygon.io/v1/open-close/{}/{}?apiKey={}", ticker, date, api_key);
        Ok(serde_json::from_str(
            reqwest::get(url)
                .await
                .unwrap()
                .text()
                .await
                .unwrap()
                .as_str(),
        )
        .unwrap())
    }
}