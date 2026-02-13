use crate::types::order_id::OrderId;
pub trait GenerateNewOrderId<OIDI> {
    fn generate_new_order_id(&self) -> OrderId<OIDI>;
}
