use crate::enums::cancel_order_error::CancelOrderError;
use crate::traits::order::ReadOrderRef;
use crate::types::order::Order;
use core::future::Future;

pub trait CancelOrder<OIDI, EOIDI, II> {
    fn cancel_order(
        &self,
        order: impl ReadOrderRef<OIDI, EOIDI, II>,
    ) -> impl Future<Output = Result<Order<OIDI, EOIDI, II>, CancelOrderError>> + Send;
}
