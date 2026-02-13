use crate::types::order_id::OrderId;

pub trait ReadOrderIdInner<OIDI> {
    fn read_order_id_inner(&self) -> OIDI;
}

pub trait ReadOrderId<OID> {
    fn read_order_id(&self) -> OrderId<OID>;
}

pub trait WriteOrderIdInner<OIDI> {
    fn write_order_id_inner(&mut self, inner: OIDI) -> &mut Self;
}

pub trait WriteOrderId<OID> {
    fn write_order_id(&mut self, id: &impl ReadOrderIdInner<OID>) -> &mut Self;
}

impl<OIDI, O> ReadOrderId<OIDI> for O
where
    O: ReadOrderIdInner<OIDI>,
{
    fn read_order_id(&self) -> OrderId<OIDI> {
        OrderId::new(self.read_order_id_inner())
    }
}

impl<OIDI: Clone, O> WriteOrderId<OIDI> for O
where
    O: WriteOrderIdInner<OIDI>,
{
    fn write_order_id(&mut self, id: &impl ReadOrderIdInner<OIDI>) -> &mut Self {
        self.write_order_id_inner(id.read_order_id_inner())
    }
}
