use crate::enums::candlestick_error::CandlestickError;
use crate::enums::candlestick_interval::CandlestickInterval;
use crate::types::candlestick::Candlestick;
use crate::types::datetime::DateTime;
use crate::types::instrument_id::InstrumentId;
use core::future::Future;

pub trait ReceiveCandlesticks<II, DTI, PVI, QI> {
    fn receive_candlesticks(
        &self,
        instrument_id: InstrumentId<II>,
        interval: CandlestickInterval,
        from: Option<DateTime<DTI>>,
        to: Option<DateTime<DTI>>,
    ) -> impl Future<Output = Result<Vec<Candlestick<PVI, QI>>, CandlestickError>> + Send;
}
