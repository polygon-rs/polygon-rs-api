pub mod aggregates;
pub mod bbo;
pub mod currency_quote;
pub mod daily;
pub mod grouped;
pub mod last_quote;
pub mod last_trade;
pub mod ltc;
pub mod nbbo;
pub mod previous;
pub mod rtc;
pub mod snapshots;
pub mod technical_indicators;
pub mod trades;

#[derive(serde::Deserialize)]
pub enum Market {
    Aggregates(aggregates::Aggregates), //Done
    Grouped(grouped::Grouped),          //Done
    Daily(daily::Daily),                //Done
    Previous(previous::Previous),       //Done
    Trades(trades::Trades),
    LastTrade(last_trade::LastTrade), //Done
    Nbbo(nbbo::NBBO),
    LastQuote(last_quote::LastQuote), //Done
    Snapshots(snapshots::Snapshots),
    TechnicalIndicators(technical_indicators::TechnicalIndicators),
    //Rtc(rtc::Rtc),
    //CurrencyQuote(currency_quote::CurrencyQuote),
    //Bbo(bbo::Bbo),
    //Ltc(ltc::Ltc),
}
