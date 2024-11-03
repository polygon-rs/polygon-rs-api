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
use snapshot::gainers_losers::GainersLosersRequest;
use snapshot::indicies_snapshot::IndiciesSnapshotRequest;
use snapshot::l2_snapshot::L2SnapshotRequest;
use snapshot::options_chain::OptionsChainRequest;
use snapshot::options_contract::OptionsContractRequest;
use snapshot::ticker_snapshot::TickerSnapshotRequest;
use snapshot::tickers_snapshot::TickersSnapshotRequest;
use snapshot::universal_snapshot::UniversalSnapshotRequest;
use technical_indicators::exponential_moving_average::ExponentialMovingAverageRequest;
use technical_indicators::moving_average_converge_divergence::MovingAverageConvergenceDivergenceRequest;
use technical_indicators::relative_strength_index::RelativeStrengthIndexRequest;
use technical_indicators::simple_moving_average::SimpleMovingAverageRequest;
use trades::TradesRequest;

#[derive(Serialize, Deserialize)]
pub enum Market {
    Aggregates(aggregates::Aggregates),
    BBO(bbo::BBO),
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
    TechnicalIndicators(technical_indicators::TechnicalIndicators),
    Trades(trades::Trades),
}

pub struct MarketRequest {}

impl AggregatesRequest for MarketRequest {}

impl BBORequest for MarketRequest {}

impl CurrencyConversionRequest for MarketRequest {}

impl DailyRequest for MarketRequest {}

impl GroupedBarsRequest for MarketRequest {}

impl LastQuoteRequest for MarketRequest {}

impl LastTradeRequest for MarketRequest {}

impl PairQuoteRequest for MarketRequest {}

impl PairTradeRequest for MarketRequest {}

impl PreviousRequest for MarketRequest {}

impl QuotesRequest for MarketRequest {}

impl TradesRequest for MarketRequest {}

impl GainersLosersRequest for MarketRequest {}

impl IndiciesSnapshotRequest for MarketRequest {}

impl L2SnapshotRequest for MarketRequest {}

impl OptionsChainRequest for MarketRequest {}

impl OptionsContractRequest for MarketRequest {}

impl TickerSnapshotRequest for MarketRequest {}

impl TickersSnapshotRequest for MarketRequest {}

impl UniversalSnapshotRequest for MarketRequest {}

impl ExponentialMovingAverageRequest for MarketRequest {}

impl MovingAverageConvergenceDivergenceRequest for MarketRequest {}

impl RelativeStrengthIndexRequest for MarketRequest {}

impl SimpleMovingAverageRequest for MarketRequest {}
