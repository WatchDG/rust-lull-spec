use crate::enums::signal::Signal;
use crate::enums::signal::SignalError;
use core::future::Future;

pub trait ReceiveSignal<SI> {
    fn receive_signal(&self) -> impl Future<Output = Result<Signal, SignalError>> + Send;
}
