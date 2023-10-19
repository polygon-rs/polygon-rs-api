use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Polygon {
    pub security: Option<Secuirty>,
    pub call: Option<Call>,
    pub api_key: Option<String>,
    pub ticker: Option<String>,
    pub multiplier: Option<u16>,
    pub timespan: Option<Timespan>,
    pub from: Option<String>,
    pub to: Option<String>,
    pub adjusted: Option<bool>,
    pub sort: Option<Sort>,
    pub limit: Option<u16>,
    pub date: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Stocks {}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Options {}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Indices {}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Forex {}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Crypto {}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NBBO {
    pub next_url: Option<String>,
    pub request_id: Option<String>,
    pub results: Option<Vec<Quote>>,
    pub status: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Quote {
    pub ask_exchange: Option<i32>,
    pub ask_price: Option<f64>,
    pub ask_size: Option<i32>,
    pub bid_exchange: Option<i32>,
    pub bid_price: Option<f64>,
    pub bid_size: Option<i32>,
    pub indicators: Option<Vec<i32>>,
    pub participant_timestamp: Option<i64>,
    pub sequence_number: Option<i64>,
    pub sip_timestamp: Option<i64>,
    pub tape: Option<i32>,
}

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
    pub volume: Option<i32>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Secuirty {
    Stocks(Stocks),
    Options(Options),
    Indices(Indices),
    Forex(Forex),
    Crypto(Crypto),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Call {
    Daily(Daily),
    NBBO(NBBO),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Timespan {
    Second,
    Minute,
    Hour,
    Day,
    Week,
    Month,
    Quater,
    Year,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Sort {
    Asc,
    Desc,
}

impl Polygon {
    pub fn polygon(
        security: Option<Secuirty>,
        call: Option<Call>,
        api_key: Option<String>,
        ticker: Option<String>,
        multiplier: Option<u16>,
        timespan: Option<Timespan>,
        from: Option<String>,
        to: Option<String>,
        adjusted: Option<bool>,
        sort: Option<Sort>,
        limit: Option<u16>,
        date: Option<String>,
    ) -> Polygon {
        Polygon {
            security: security,
            call: call,
            api_key: api_key,
            ticker: ticker,
            multiplier: multiplier,
            timespan: timespan,
            from: from,
            to: to,
            adjusted: adjusted,
            sort: sort,
            limit: limit,
            date: date,
        }
    }

    pub fn request(&self) -> Result<Call, Box<dyn Error>> {
        match &self.security {
            Some(v) => match v {
                Secuirty::Stocks(_) => Stocks::request(&self.clone()),
                Secuirty::Options(_) => Options::request(&self.clone()),
                Secuirty::Indices(_) => Indices::request(&self.clone()),
                Secuirty::Forex(_) => Forex::request(&self.clone()),
                Secuirty::Crypto(_) => Crypto::request(&self.clone()),
            },
            None => panic!("There is either no security type set"),
        }
    }
}

impl Stocks {
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

impl Indices {
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

impl Forex {
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

impl Crypto {
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

/*
#[derive(Serialize, Deserialize)]
pub struct Response {
    pub response: Option<serde_json::Value>,
}
*/
