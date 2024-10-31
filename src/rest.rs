pub mod error;
pub mod market;
pub mod parameters;
pub mod reference;

#[derive(Serialize, Deserialize)]
pub enum Rest {
    Market(market::Market),
}

use market::aggregates::AggregatesRequest;
use market::bbo::BBORequest;
use market::currency_conversion::CurrencyConversionRequest;
use market::daily::DailyRequest;
use market::grouped_bars::GroupedBarsRequest;
use market::last_quote::LastQuoteRequest;
use market::last_trade::LastTradeRequest;
use market::pair_quote::PairQuoteRequest;
use market::pair_trade::PairTradeRequest;
use market::previous::PreviousRequest;
use market::quotes::QuotesRequest;
use serde::{Deserialize, Serialize};
use market::trades::TradesRequest;
use market::snapshot::gainers_losers::GainersLosersRequest;
use market::snapshot::indicies_snapshot::IndiciesSnapshotRequest;
use market::snapshot::l2_snapshot::L2SnapshotRequest;
use market::snapshot::options_chain::OptionsChainRequest;
use market::snapshot::options_contract::OptionsContractRequest;
use market::snapshot::ticker_snapshot::TickerSnapshotRequest;
use market::snapshot::tickers_snapshot::TickersSnapshotRequest;
use market::snapshot::universal_snapshot::UniversalSnapshotRequest;
use market::technical_indicators::exponential_moving_average::ExponentialMovingAverageRequest;
use market::technical_indicators::moving_average_converge_divergence::MovingAverageConvergenceDivergenceRequest;
use market::technical_indicators::relative_strength_index::RelativeStrengthIndexRequest;
use market::technical_indicators::simple_moving_average::SimpleMovingAverageRequest;

pub struct RestRequest {}

impl AggregatesRequest for RestRequest {}

impl BBORequest for RestRequest {}

impl CurrencyConversionRequest for RestRequest {}

impl DailyRequest for RestRequest {}

impl GroupedBarsRequest for RestRequest {}

impl LastQuoteRequest for RestRequest {}

impl LastTradeRequest for RestRequest {}

impl PairQuoteRequest for RestRequest {}

impl PairTradeRequest for RestRequest {}

impl PreviousRequest for RestRequest {}

impl QuotesRequest for RestRequest {}

impl TradesRequest for RestRequest {}

impl GainersLosersRequest for RestRequest {}

impl IndiciesSnapshotRequest for RestRequest {}

impl L2SnapshotRequest for RestRequest {}

impl OptionsChainRequest for RestRequest {}

impl OptionsContractRequest for RestRequest {}

impl TickerSnapshotRequest for RestRequest {}

impl TickersSnapshotRequest for RestRequest {}

impl UniversalSnapshotRequest for RestRequest {}

impl ExponentialMovingAverageRequest for RestRequest {}

impl MovingAverageConvergenceDivergenceRequest for RestRequest {}

impl RelativeStrengthIndexRequest for RestRequest {}

impl SimpleMovingAverageRequest for RestRequest {}
