use crate::enums::trade_signal::TradeSignal;
use crate::enums::trade_signal::TradeSignalError;
use core::future::Future;

pub trait ReceiveTradeSignal<SI> {
    fn receive_trade_signal(
        &self,
    ) -> impl Future<Output = Result<TradeSignal, TradeSignalError>> + Send;
}
