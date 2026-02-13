use crate::types::order_id::OrderId;

pub trait ReadOrderId<OID> {
    fn read_order_id(&self) -> OrderId<OID>;
}

pub trait WriteOrderId<OID> {
    fn write_order_id(&mut self, id: OrderId<OID>) -> &mut Self;
}
