use crate::traits::order::ReadOrderRef;
use crate::traits::order_id::{ReadOrderId, ReadOrderIdRef};
use crate::types::order_id::OrderId;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Order<OI, OIDI> {
    pub inner: OI,
    pub id: OrderId<OIDI>,
}

impl<OI, OIDI> Order<OI, OIDI>
where
    OI: ReadOrderId<OIDI>,
{
    pub fn new(inner: OI) -> Self {
        let id = inner.read_order_id();
        Self { inner, id }
    }
}

impl<OI, OIDI> ReadOrderRef<OI, OIDI> for Order<OI, OIDI> {
    fn read_order_ref(&self) -> &Order<OI, OIDI> {
        self
    }
}

impl<OIDI> ReadOrderIdRef<OIDI> for Order<OIDI, OIDI> {
    fn read_order_id_ref(&self) -> &OrderId<OIDI> {
        &self.id
    }
}
