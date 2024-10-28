pub mod exponential_moving_average;
pub mod macd;
pub mod relative_strength_index;
pub mod simple_moving_average;

use exponential_moving_average::ExponentialMovingAverageRequest;
use relative_strength_index::RelativeStrengthIndexRequest;
use serde::{Deserialize, Serialize};
use simple_moving_average::SimpleMovingAverageRequest;

#[derive(Serialize, Deserialize)]
pub enum TechnicalIndicators {
    ExponentialMovingAverage(exponential_moving_average::ExponentialMovingAverage),
    MACD,
    RelativeStrengthIndex(relative_strength_index::RelativeStrengthIndex),
    SimpleMovingAverage(simple_moving_average::SimpleMovingAverage),
}

pub trait TechnicalIndicatorsRequest {}

impl ExponentialMovingAverageRequest for dyn TechnicalIndicatorsRequest {}

impl RelativeStrengthIndexRequest for dyn TechnicalIndicatorsRequest {}

impl SimpleMovingAverageRequest for dyn TechnicalIndicatorsRequest {}
