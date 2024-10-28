pub mod aggregates;
pub mod bbo;
pub mod currency_conversion;
pub mod daily;
pub mod grouped_bars;
pub mod last_quote;
pub mod last_trade;
pub mod pair_quote;
pub mod pair_trade;
pub mod previous;
pub mod quotes;
pub mod snapshot;
pub mod technical_indicators;
pub mod trades;

use aggregates::AggregatesRequest;
use bbo::BBORequest;
use currency_conversion::CurrencyConversionRequest;
use daily::DailyRequest;
use grouped_bars::GroupedBarsRequest;
use last_quote::LastQuoteRequest;
use last_trade::LastTradeRequest;
use pair_quote::PairQuoteRequest;
use pair_trade::PairTradeRequest;
use previous::PreviousRequest;
use quotes::QuotesRequest;
use serde::{Deserialize, Serialize};
use snapshot::SnapshotRequest;
use trades::TradesRequest;

#[derive(Serialize, Deserialize)]
pub enum Market {
    Aggregates(aggregates::Aggregates),
    BBO(bbo::BBO), // Change all parse arrays to this parse method
    CurrencyConversion(currency_conversion::CurrencyConversion), 
    Daily(daily::Daily),
    Grouped(grouped_bars::GroupedBars),
    LastQuote(last_quote::LastQuote),
    LastTrade(last_trade::LastTrade),
    PairQuote(pair_quote::PairQuote),
    PairTrade(pair_trade::PairTrade),
    Previous(previous::Previous),
    Quotes(quotes::Quotes),
    Snapshots(snapshot::Snapshot),
    TechnicalIndicators(technical_indicators::TechnicalIndicators), //Not Done
    Trades(trades::Trades),
}

pub trait MarketRequest {}

impl SnapshotRequest for dyn MarketRequest {}

impl AggregatesRequest for dyn MarketRequest {}

impl BBORequest for dyn MarketRequest {}

impl CurrencyConversionRequest for dyn MarketRequest {}

impl DailyRequest for dyn MarketRequest {}

impl GroupedBarsRequest for dyn MarketRequest {}

impl LastQuoteRequest for dyn MarketRequest {}

impl LastTradeRequest for dyn MarketRequest {}

impl PairQuoteRequest for dyn MarketRequest {}

impl PairTradeRequest for dyn MarketRequest {}

impl PreviousRequest for dyn MarketRequest {}

impl QuotesRequest for dyn MarketRequest {}

impl TradesRequest for dyn MarketRequest {}
