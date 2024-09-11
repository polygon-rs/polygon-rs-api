use crate::{ErrorCode, Parameter, ParameterRequirment, Parameters, Request, ContractType, ContractStyle, Timeframe};

#[derive(serde::Deserialize, Clone, Debug, Default)]
pub struct Chain {
    chain_parameters: Parameters,
    chain_url: String,
    pub request_id: String,
    pub next_url: String,
    pub results: Vec<Contract>,
    pub status: String,
}

#[derive(serde::Deserialize, Clone, Debug, Default)]
pub struct Contract {
    pub break_even_price: f64,
    pub day: Day,
    pub details: Details,
    pub fair_market_value: f64,
    pub greeks: Greeks,
    pub implied_volatility: f64,
    pub quote: Quote,
    pub trade: Trade,
    pub open_interest: i64,
    pub underlying_asset: UnderlyingAsset,
}

#[derive(serde::Deserialize, Clone, Debug, Default)]
pub struct Day {
    pub change: f64,
    pub change_percent: f64,
    pub close: f64,
    pub high: f64,
    pub last_updated: i64,
    pub low: f64,
    pub open: f64,
    pub previous_close: f64,
    pub volume: i64,
    pub volume_weighted_average_price: f64,
}

#[derive(serde::Deserialize, Clone, Debug, Default)]
pub struct Details {
    pub contract_type: ContractType,
    pub contract_style: ContractStyle,
    pub expiration_date: String,
    pub shares_per_contract: i64,
    pub strike_price: f64,
    pub ticker: String,
}

#[derive(serde::Deserialize, Clone, Debug, Default)]
pub struct Greeks {
    pub delta: f64,
    pub gamma: f64,
    pub theta: f64,
    pub vega: f64,

}
#[derive(serde::Deserialize, Clone, Debug, Default)]
pub struct Quote {
    pub bid: f64,
    pub bid_size: i64,
    pub ask: f64,
    pub ask_size: i64,
    pub bid_exchange_id: i64,
    pub ask_exchange_id: i64,
    pub last_updated: i64,
    pub mid_point: f64,
    pub timeframe: Timeframe,
}

#[derive(serde::Deserialize, Clone, Debug, Default)]
pub struct Trade {
    pub conditions: Vec<i64>,
    pub exchange_id: i64,
    pub price: f64,
    pub sip_timestamp: i64,
    pub size: i64,
    pub timeframe: Timeframe,
}

#[derive(serde::Deserialize, Clone, Debug, Default)]
pub struct UnderlyingAsset {
    pub change_to_break_even: f64,
    pub last_updated: i64,
    pub price: f64,
    pub ticker: String,
    pub timeframe: Timeframe,
    pub value: f64,
}

impl Chain {
    pub fn set_parameters(
        &mut self,
        api_key: String,
        ticker: String,
        date: String,
        adjusted: Option<bool>,
    ) {
        self.chain_parameters = Parameters {
            api_key: api_key,
            ticker: Some(ticker),
            date: Some(date),
            adjusted: adjusted,
            ..Parameters::default()
        }
    }
}

impl Request for Chain {
    const VERSION: &'static str = "v3";
    const CALL: &'static str = "snapshot/options";
    const PARAMETERS: &'static [&'static ParameterRequirment] = &[
        &ParameterRequirment {
            required: true,
            parameter: Parameter::Ticker,
        },
        &ParameterRequirment {
            required: false,
            parameter: Parameter::StrikePrice,
        },
        &ParameterRequirment {
            required: false,
            parameter: Parameter::Date,
        },
        &ParameterRequirment {
            required: false,
            parameter: Parameter::To,
        },
        &ParameterRequirment {
            required: false,
            parameter: Parameter::From,
        },
        &ParameterRequirment{
            required: false,
            parameter: Parameter::ContractType,
        },
        &ParameterRequirment{
            required: false,
            parameter: Parameter::Order,
        },
        &ParameterRequirment{
            required: false,
            parameter: Parameter::Limit,
        },
        &ParameterRequirment{
            required: false,
            parameter: Parameter::Sortv3,
        },
    ];

    fn parameters(&self) -> &Parameters {
        &self.chain_parameters
    }

    fn url(&mut self) -> &String {
        &self.chain_url
    }

    fn set_url(&mut self) -> Result<(), ErrorCode> {
        if let Err(check) = self.check_parameters() {
            return Err(check);
        }
        self.chain_url = String::from(format!(
            "{}/{}/{}/{}?{}{}{}{}{}{}{}{}{}{}apiKey={}",
            Self::BASE_URL,
            Self::VERSION,
            Self::CALL,
            self.parameters().clone().ticker.unwrap(),
            if let Some(strike_price) = self.parameters().clone().strike_price {
                format!("strike_price={}&", strike_price)
            } else {
                "".to_string()
            },
            if let Some(strike_price_from) = self.parameters().clone().strike_price_from {
                format!("strike_price.gte={}&", strike_price_from)
            } else {
                "".to_string()
            },
            if let Some(strike_price_to) = self.parameters().clone().strike_price_to {
                format!("strike_price.lte={}&", strike_price_to)
            } else {
                "".to_string()
            },
            if let Some(date) = self.parameters().clone().date {
                format!("expiration_date={}&", date)
            } else {
                "".to_string()
            },
            if let Some(from) = self.parameters().clone().from {
                format!("expiration_date.gte={}&", from)
            } else {
                "".to_string()
            },
            if let Some(to) = self.parameters().clone().to {
                format!("expiration_date.lte={}&", to)
            } else {
                "".to_string()
            },
            if let Some(contract_type) = self.parameters().clone().contract_type {
                format!("contract_type={}&", contract_type)
            } else {
                "".to_string()
            }, 
            if let Some(order) = self.parameters().clone().order {
                format!("order={}&", order)
            } else {
                "".to_string()
            }, 
            if let Some(limit) = self.parameters().clone().limit {
                format!("limit={}&", limit)
            } else {
                "".to_string()
            }, 
            if let Some(sort) = self.parameters().clone().sortv3 {
                format!("sort={}&", sort)
            } else {
                "".to_string()
            },           
            self.parameters().clone().api_key,
        ));
        Ok(())
    }

    fn request(&mut self) -> Result<(), ErrorCode> {
        match self.polygon_request() {
            Ok(response) => {
                if let Some(status) = response["status"].as_str() {
                    self.status = status.to_string()
                }
                if let Some(next_url) = response["next_url"].as_str() {
                    self.next_url = next_url.to_string()
                } else {
                    self.next_url = "".to_string()
                }
                if let Some(request_id) = response["request_id"].as_str() {
                    self.request_id = request_id.to_string()
                }
                if let Some(results) = response["results"].as_array() {
                    let mut contract = Contract::default();
                    for result in results {
                        if let Some(break_even_price) = result["break_even_price"].as_f64() {
                            contract.break_even_price = break_even_price
                        }
                        if let Some(day) = result["day"].as_object() {
                            if let Some(change) = day["change"].as_f64() {
                                contract.day.change = change
                            }
                            if let Some(change_percent) = day["change_percent"].as_f64() {
                                contract.day.change_percent = change_percent
                            }
                            if let Some(close) = day["close"].as_f64() {
                                contract.day.close = close
                            }
                            if let Some(high) = day["high"].as_f64() {
                                contract.day.high = high
                            }
                            if let Some(last_updated) = day["last_updated"].as_i64() {
                                contract.day.last_updated = last_updated
                            }
                            if let Some(low) = day["low"].as_f64() {
                                contract.day.low = low
                            }
                            if let Some(open) = day["open"].as_f64() {
                                contract.day.open = open
                            }
                            if let Some(previous_close) = day["previous_close"].as_f64() {
                                contract.day.previous_close = previous_close
                            }
                            if let Some(volume) = day["volume"].as_i64() {
                                contract.day.volume = volume
                            }
                            if let Some(volume_weighted_average_price) =
                                day["volume_weighted_average_price"].as_f64()
                            {
                                contract.day.volume_weighted_average_price =
                                    volume_weighted_average_price
                            }
                        }
                        if let Some(details) = result["details"].as_object() {
                            if let Some(contract_type) = details["contract_type"].as_str() {
                                contract.details.contract_type = match contract_type {
                                    "call" => ContractType::Call,
                                    "put" => ContractType::Put,
                                    _ => ContractType::Unknown,
                                
                                }
                            }
                            if let Some(contract_style) = details["contract_style"].as_str() {
                                contract.details.contract_style = match contract_style {
                                    "american" => ContractStyle::American,
                                    "european" => ContractStyle::European,
                                    "bermudan" => ContractStyle::Bermudan,
                                    _ => ContractStyle::Unknown,
                                }
                            }
                            if let Some(expiration_date) = details["expiration_date"].as_str() {
                                contract.details.expiration_date = expiration_date.to_string()
                            }
                            if let Some(shares_per_contract) =
                                details["shares_per_contract"].as_i64()
                            {
                                contract.details.shares_per_contract = shares_per_contract
                            }
                            if let Some(strike_price) = details["strike_price"].as_f64() {
                                contract.details.strike_price = strike_price
                            }
                            if let Some(ticker) = details["ticker"].as_str() {
                                contract.details.ticker = ticker.to_string()
                            }
                        }
                        if let Some(fair_market_value) = result["fair_market_value"].as_f64() {
                            contract.fair_market_value = fair_market_value
                        }
                        if let Some(greeks) = result["greeks"].as_object() {
                            if let Some(delta) = greeks["delta"].as_f64() {
                                contract.greeks.delta = delta
                            }
                            if let Some(gamma) = greeks["gamma"].as_f64() {
                                contract.greeks.gamma = gamma
                            }
                            if let Some(theta) = greeks["theta"].as_f64() {
                                contract.greeks.theta = theta
                            }
                            if let Some(vega) = greeks["vega"].as_f64() {
                                contract.greeks.vega = vega
                            }
                        }
                        if let Some(implied_volatility) = result["implied_volatility"].as_f64() {
                            contract.implied_volatility = implied_volatility
                        }
                        if let Some(quote) = result["quote"].as_object() {
                            if let Some(bid) = quote["bid"].as_f64() {
                                contract.quote.bid = bid
                            }
                            if let Some(bid_size) = quote["bid_size"].as_i64() {
                                contract.quote.bid_size = bid_size
                            }
                            if let Some(ask) = quote["ask"].as_f64() {
                                contract.quote.ask = ask
                            }
                            if let Some(ask_size) = quote["ask_size"].as_i64() {
                                contract.quote.ask_size = ask_size
                            }
                            if let Some(bid_exchange_id) = quote["bid_exchange_id"].as_i64() {
                                contract.quote.bid_exchange_id = bid_exchange_id
                            }
                            if let Some(ask_exchange_id) = quote["ask_exchange_id"].as_i64() {
                                contract.quote.ask_exchange_id = ask_exchange_id
                            }
                            if let Some(last_updated) = quote["last_updated"].as_i64() {
                                contract.quote.last_updated = last_updated
                            }
                            if let Some(mid_point) = quote["mid_point"].as_f64() {
                                contract.quote.mid_point = mid_point
                            }
                            if let Some(timeframe) = quote["timeframe"].as_str() {
                                contract.quote.timeframe = match timeframe {
                                    "DELAYED" => Timeframe::Delayed,
                                    "REAL-TIME" => Timeframe::RealTime,
                                    _ => Timeframe::Unknown,
                                }
                            }
                        }
                        if let Some(trade) = result["trade"].as_object() {
                            if let Some(conditions) = trade["conditions"].as_array() {
                                for condition in conditions {
                                    if let Some(c) = condition.as_i64() {
                                        contract.trade.conditions.push(c)
                                    }
                                }
                            }
                            if let Some(exchange_id) = trade["exchange_id"].as_i64() {
                                contract.trade.exchange_id = exchange_id
                            }
                            if let Some(price) = trade["price"].as_f64() {
                                contract.trade.price = price
                            }
                            if let Some(sip_timestamp) = trade["sip_timestamp"].as_i64() {
                                contract.trade.sip_timestamp = sip_timestamp
                            }
                            if let Some(size) = trade["size"].as_i64() {
                                contract.trade.size = size
                            }
                            if let Some(timeframe) = trade["timeframe"].as_str() {
                                contract.trade.timeframe = match timeframe {
                                    "DELAYED" => Timeframe::Delayed,
                                    "REAL-TIME" => Timeframe::RealTime,
                                    _ => Timeframe::Unknown,
                                }
                            }
                        }
                        if let Some(open_interest) = result["open_interest"].as_i64() {
                            contract.open_interest = open_interest
                        }
                        if let Some(underlying_asset) = result["underlying_asset"].as_object() {
                            if let Some(change_to_break_even) =
                                underlying_asset["change_to_break_even"].as_f64()
                            {
                                contract.underlying_asset.change_to_break_even =
                                    change_to_break_even
                            }
                            if let Some(last_updated) = underlying_asset["last_updated"].as_i64() {
                                contract.underlying_asset.last_updated = last_updated
                            }
                            if let Some(price) = underlying_asset["price"].as_f64() {
                                contract.underlying_asset.price = price
                            }
                            if let Some(ticker) = underlying_asset["ticker"].as_str() {
                                contract.underlying_asset.ticker = ticker.to_string()
                            }
                            if let Some(timeframe) = underlying_asset["timeframe"].as_str() {
                                contract.underlying_asset.timeframe = match timeframe {
                                    "DELAYED" => Timeframe::Delayed,
                                    "REAL-TIME" => Timeframe::RealTime,
                                    _ => Timeframe::Unknown,
                                }
                            }
                            if let Some(value) = underlying_asset["value"].as_f64() {
                                contract.underlying_asset.value = value
                            }
                        }
                    }
                    self.results.push(contract);
                }
            }
            Err(e) => return Err(e),
        }

        Ok(())
    }
    
}
