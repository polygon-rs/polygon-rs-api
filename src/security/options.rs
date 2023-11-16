use crate::Call;
use crate::Daily;
use crate::Polygon;
use crate::NBBO;
use crate::Trades;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Options {}

impl Options {
    pub fn request(polygon: &Polygon) -> Result<Call, Box<dyn Error>> {
        match &polygon.call {
            Some(call) => match call {
                Call::NBBO(_) => match NBBO::nbbo(polygon.clone()) {
                    Ok(nbbo) => Ok(Call::NBBO(nbbo)),
                    Err(e) => panic!("The following error occured: {}", e),
                },
                Call::Daily(_) => match Daily::daily(polygon.clone()) {
                    Ok(daily) => Ok(Call::Daily(daily)),
                    Err(e) => panic!("The following error occured: {}", e),
                },
                Call::Trades(_) => match Trades::trades(polygon.clone()) {
                    Ok(trades) => Ok(Call::Trades(trades)),
                    Err(e) => panic!("The following error occured: {}", e),
                },
            },
            None => panic!("There is no call type set"),
        }
    }
}
