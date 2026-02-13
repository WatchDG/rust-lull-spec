use crate::types::order_id::OrderId;

pub trait ReadOrderIdInner<OID> {
    fn read_order_id_inner(&self) -> OID;
}

pub trait ReadOrderId<OID> {
    fn read_order_id(&self) -> OrderId<OID>;
}

pub trait WriteOrderIdInner<OID> {
    fn write_order_id_inner(&mut self, inner: OID) -> &mut Self;
}

pub trait WriteOrderId<OID> {
    fn write_order_id(&mut self, id: &impl ReadOrderIdInner<OID>) -> &mut Self;
}

impl<OID, T> ReadOrderId<OID> for T
where
    T: ReadOrderIdInner<OID>,
{
    fn read_order_id(&self) -> OrderId<OID> {
        OrderId::new(self.read_order_id_inner())
    }
}

impl<OID: Clone, T> WriteOrderId<OID> for T
where
    T: WriteOrderIdInner<OID>,
{
    fn write_order_id(&mut self, id: &impl ReadOrderIdInner<OID>) -> &mut Self {
        self.write_order_id_inner(id.read_order_id_inner())
    }
}
