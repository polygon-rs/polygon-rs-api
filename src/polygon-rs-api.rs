#![allow(unused)]

pub mod polygon;
pub mod rest;
pub mod web_socket;

use crate::polygon::{Polygon, sort::Sort, timespan::Timespan};
use crate::rest::{
    market::{daily::Daily, nbbo::NBBO, snapshots::chain::Chain, trades::Trades},
    reference,
};

