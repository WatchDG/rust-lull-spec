use crate::enums::place_order_error::PlaceOrderError;
use crate::types::new_order::NewOrder;
use crate::types::order::Order;
use core::future::Future;
pub trait PlaceOrder<OIDI, EOIDI, II> {
    fn place_order(
        &self,
        place_order: NewOrder<OIDI, II>,
    ) -> impl Future<Output = Result<Order<OIDI, EOIDI, II>, PlaceOrderError>> + Send;
}
