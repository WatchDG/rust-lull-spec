use crate::enums::candlestick_interval::CandlestickInterval;
use crate::types::datetime::DateTime;
use crate::types::instrument_id::InstrumentId;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CandlesticksRequest<II, DTI> {
    pub instrument_id: InstrumentId<II>,
    pub interval: CandlestickInterval,
    pub from: Option<DateTime<DTI>>,
    pub to: Option<DateTime<DTI>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ChannelMarketDataRequest<II, DTI> {
    Candlesticks(CandlesticksRequest<II, DTI>),
}
